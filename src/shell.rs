use diagram::ResponseOption;
/// クライアント１つにつき、１つのシェルを与えます。
/// 行単位です。
///
/// コマンド例
///
/// ```
/// cls
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shell
/// cargo clippy
/// ```
use diagram_player::*;
use diagram::*;
use regex::Regex;
use std::any::Any; // https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait
use std::io;

/// 不具合を取りたいときに真にする。
const VERBOSE: bool = false;

const NEXT_EXIT_LABEL: &str = "#next";
/// デフォルトのラベル。
const NEWLINE_EXIT_LABEL: &str = "#newline";
const ELSE_NODE_LABEL: &str = "#else";

/// コマンドライン文字列。
///
/// # Members
///
/// * `line` - コマンドライン文字列の1行全体です。
/// * `line_len` - コマンドライン文字列の1行全体の文字数です。
/// * `groups` - あれば、正規表現の結果を入れておく。
pub struct RequestStruct {
    pub line: Box<String>, // String型は長さが可変なので、固定長のBoxでラップする。
    pub line_len: usize,
    pub caret: usize,
    pub groups: Vec<String>,
}
impl RequestStruct {
    fn new(line2: Box<String>) -> RequestStruct {
        let len = line2.chars().count();
        RequestStruct {
            line: line2,
            line_len: len,
            caret: 0,
            groups: Vec::new(),
        }
    }
}
impl Request for RequestStruct {
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
    fn get_line(&self) -> &String {
        &self.line
    }
    fn get_line_len(&self) -> usize {
        self.line_len
    }
    fn get_caret(&self) -> usize {
        self.caret
    }
    fn get_groups(&self) -> &Vec<String> {
        &self.groups
    }
}

/// キャレット。本来、文字列解析のカーソル位置だが、ほかの機能も持たされている。
/// - シェルを終了するなど、シェルに対して指示することができる。
/// - また、字句解析の次のノードを指定する役割も持つ。
///
/// # Members
///
/// * `starts` - コマンドライン文字列の次のトークンの先頭位置です。
/// * `done_line` - 行の解析を中断するなら真にします。
/// * `option` - シェルに指示を出す。アプリケーション終了、ファイル再読込など。
/// * `exit_label` - 次のノード ラベルです。
pub struct ResponseStruct {
    pub caret: usize,
    pub done_line: bool,
    pub option: ResponseOption,
    pub exit_label: String,
}
impl ResponseStruct {
    fn new() -> ResponseStruct {
        ResponseStruct {
            caret: 0,
            done_line: false,
            option: ResponseOption::None,
            exit_label: "".to_string(),
        }
    }
    fn reset(&mut self) {
        self.set_caret(0);
        self.set_done_line(false);
        self.set_option(ResponseOption::None);
        self.forward("");
    }
}

impl Response for ResponseStruct {
    fn as_any(&self) -> &dyn Any {
        self
    }
    /// トレイトを実装している方を返すのに使う。
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
    // .rs にハードコーディングして使う。
    fn forward(&mut self, exit_label2: &'static str) {
        self.exit_label = exit_label2.to_string();
    }
    fn set_caret(&mut self, caret2: usize) {
        self.caret = caret2
    }
    fn set_done_line(&mut self, done_line2: bool) {
        self.done_line = done_line2
    }
    fn set_option(&mut self, value: ResponseOption) {
        self.option = value;
    }
}

pub type Reader<T> = fn(t: &mut T) -> String;

pub fn standard_input_reader<T>(_t: &mut T) -> String {
    let mut line_string = String::new();
    // コマンド プロンプトからの入力があるまで待機します。
    io::stdin()
        .read_line(&mut line_string)
        .expect("info Failed to read_line"); // OKでなかった場合のエラーメッセージ。

    // 末尾の 改行 を除きます。前後の空白も消えます。
    line_string.trim().parse().expect("info Failed to parse")
}

/// シェル。
///
/// # Arguments
///
/// * `vec_row` - コマンドを複数行 溜めておくバッファーです。
pub struct Shell<T: 'static> {
    diagram_player: DiagramPlayer,
    vec_row: Vec<String>,
    reader: Reader<T>,
}
impl<T> Default for Shell<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: 'static> Shell<T> {
    pub fn new() -> Shell<T> {
        Shell {
            diagram_player: DiagramPlayer::new(),
            vec_row: Vec::new(),
            reader: standard_input_reader,
        }
    }

    /// 現在ノードのラベル。
    pub fn get_current(&self) -> String {
        self.diagram_player.get_current().to_string()
    }

    /// 現在地が遷移図の外か。
    pub fn is_out(&self) -> bool {
        self.diagram_player.is_out()
    }

    pub fn set_reader(&mut self, reader2: Reader<T>) {
        self.reader = reader2;
    }
    /// コマンドを1行も入力していなければ真を返します。
    pub fn is_empty(&self) -> bool {
        self.vec_row.len() == 0
    }
    /// コンソール入力以外の方法で、コマンド1行を追加したいときに使います。
    /// 行の末尾に改行は付けないでください。
    pub fn push_row(&mut self, row: &str) {
        self.vec_row.push(format!("{}\n", row));
    }
    /// 先頭のコマンド1行をキューから削除して返します。
    pub fn pop_row(&mut self) -> Box<String> {
        Box::new(self.vec_row.pop().unwrap())
    }

    /// [token]文字列の長さだけ [starts]キャレットを進めます。
    /// [token]文字列の続きに半角スペース「 」が１つあれば、1つ分だけ読み進めます。
    ///
    /// # Arguments
    ///
    /// * `req` - 読み取るコマンドラインと、読取位置。
    /// * returns - 一致したら真。
    fn starts_with_literal(&self, node: &Node, req: &mut dyn Request) -> bool {
        let caret_end = req.get_caret() + node.get_token().len();
        caret_end <= req.get_line_len()
            && &req.get_line()[req.get_caret()..caret_end] == node.get_token()
    }

    /// 正規表現を使う。
    ///
    /// # Arguments
    ///
    /// * `req` - 読み取るコマンドライン。
    /// * returns - 一致したら真。
    fn starts_with_reg(&self, node: &Node, req: &mut dyn Request) -> bool {
        if VERBOSE {
            println!("Starts_with_re");
        }

        if req.get_caret() < req.get_line_len() {
            if VERBOSE {
                println!("node.token: {}", node.get_token());
            }

            let re = Regex::new(&node.get_token()).unwrap();

            let text;
            let mut group_num = 0;
            if let Some(req) = req.as_mut_any().downcast_mut::<RequestStruct>() {
                text = &req.line[req.caret..];

                if VERBOSE {
                    println!("text: [{}]", text);
                }

                for caps in re.captures_iter(text) {
                    // caps は サイズ 2 の配列 で同じものが入っている。
                    let cap = &caps[0];

                    req.groups.push(cap.to_string());

                    group_num += 1;
                }

                if VERBOSE {
                    println!("Group num: {}", group_num);
                }
            } else {
                panic!("Downcast fail.");
            }

            0 < group_num
        } else {
            false
        }
    }

    fn forward_literal(&self, node: &Node, req: &dyn Request, res: &mut dyn Response) {
        res.set_caret(req.get_caret() + node.get_token().len());
        let res_caret;
        if let Some(res) = res.as_any().downcast_ref::<ResponseStruct>() {
            res_caret = res.caret;
        } else {
            panic!("Downcast fail.");
        }

        // 続きにスペース「 」が１つあれば読み飛ばす
        if 0 < (req.get_line_len() - res_caret)
            && &req.get_line()[res_caret..(res_caret + 1)] == " "
        {
            res.set_caret(res_caret + 1);
        }
    }

    /// TODO キャレットを進める。正規表現はどこまで一致したのか分かりにくい。
    fn forward_reg(&self, req: &dyn Request, res: &mut dyn Response) {
        // グループ[0]の文字数で取る。
        let pseud_token_len = req.get_groups()[0].chars().count();
        res.set_caret(req.get_caret() + pseud_token_len);

        // 続きにスペース「 」が１つあれば読み飛ばす
        let res_caret;
        if let Some(res) = res.as_any().downcast_ref::<ResponseStruct>() {
            res_caret = res.caret;
        } else {
            panic!("Downcast fail.");
        }
        if 0 < (req.get_line_len() - res_caret)
            && &req.get_line()[res_caret..(res_caret + 1)] == " "
        {
            res.set_caret(res_caret + 1);
        }
    }

    /// コマンドラインの入力受付、および コールバック関数呼出を行います。
    /// スレッドはブロックします。
    /// 強制終了する場合は、 [Ctrl]+[C] を入力してください。
    pub fn run(&mut self, diagram: &mut Diagram<T>, t: &mut T) {
        loop {
            // リクエストは、キャレットを更新するのでミュータブル。
            let mut req = if self.is_empty() {
                let line_string = (self.reader)(t);
                RequestStruct::new(Box::new(line_string))
            } else {
                // バッファーの先頭行です。
                RequestStruct::new(self.pop_row())
            };

            use diagram::ResponseOption::*;
            let res: &mut dyn Response = &mut ResponseStruct::new();
            self.run_on_line(diagram, t, &mut req, res);
            if let Some(res_struct) = &mut res.as_mut_any().downcast_mut::<ResponseStruct>() {
                match res_struct.option {
                    None => {}
                    Quits => break, // response.quits したとき run ループを抜ける。
                    Reloads(ref file) => {
                        // ファイルからグラフのノード構成を読取。
                        diagram.read_file(&file);
                    }
                    Saves(ref file) => {
                        // ファイルを上書き。
                        diagram.write_file(&file);
                    }
                }
            } else {
                panic!("Downcast fail.");
            }
        }
    }

    /// 1行 処理するだけでいいとき。
    ///
    /// - Quits は無効になる。
    ///
    /// # Arguments.
    ///
    /// * 'diagram' -
    /// * 't' -
    /// * 'line' -
    pub fn execute_line(&mut self, diagram: &mut Diagram<T>, t: &mut T, line: &str) {
        // リクエストは、キャレットを更新するのでミュータブル。
        let mut req = RequestStruct::new(Box::new(line.to_string()));

        use diagram::ResponseOption::*;
        let res: &mut dyn Response = &mut ResponseStruct::new();
        self.run_on_line(diagram, t, &mut req, res);
        if let Some(res_struct) = &mut res.as_mut_any().downcast_mut::<ResponseStruct>() {
            match res_struct.option {
                None => {}
                Quits => {} // ループの中ではないので無効。
                Reloads(ref file) => {
                    // ファイルからグラフのノード構成を読取。
                    diagram.read_file(&file);
                }
                Saves(ref file) => {
                    // ファイルを上書き。
                    diagram.write_file(&file);
                }
            }
        } else {
            panic!("Downcast fail.");
        }
    }

    /// この関数の中では、 Diagram をイミュータブルに保つ。 Diagram の編集は この関数の外で行う。
    ///
    /// # Returns.
    ///
    /// 0. シェルを終了するなら真。
    fn run_on_line(
        &mut self,
        diagram: &Diagram<T>,
        t: &mut T,
        req: &mut dyn Request,
        res: &mut dyn Response,
    ) {
        let empty_exit_vec = &Vec::new();
        let mut current_newline_fn: Controller<T> = empty_controller;
        let mut registered_next_head_node_label = "".to_string();
        let mut current_exit_vec: &Vec<String> = &Vec::new(); // 状態の初期化。

        // 現在地が遷移図の外なら、入り口から入れだぜ☆（＾～＾）
        // println!("元入り口: [{}].", self.current_label);
        if self.is_out() {
            self.diagram_player.set_current(&diagram.get_entry_point().to_string());
            // println!("入り口を初期化: [{}].", self.current_label);
        }
        // まず 現在ノードを取得。
        let current_node = diagram.get_node(&self.diagram_player.get_current());

        current_exit_vec = match &current_node.get_exit_map().get(NEXT_EXIT_LABEL) {
            Some(n) => n,
            None => panic!(
                "run_on_line Get_exit_map: [{}] node - [{}] is not found.",
                self.get_current(), NEXT_EXIT_LABEL
            ),
        };

        'line: while req.get_caret() < req.get_line_len() {
            // キャレットの位置を、レスポンスからリクエストへ移して、次のトークンへ。
            if let Some(res) = res.as_mut_any().downcast_mut::<ResponseStruct>() {
                res.reset();
            }
            if let Some(req) = req.as_mut_any().downcast_mut::<RequestStruct>() {
                req.groups.clear(); // クリアー
            } else {
                panic!("Downcast fail.");
            }

            // 次のノード名
            let (mut best_node_label, best_node_re_label) =
                self.next_node_label(diagram, req, current_exit_vec);

            // キャレットを進める。
            let mut has_token = false;
            if best_node_label != "" {
                // 固定長での一致を優先。
                res.set_caret(req.get_caret());
                self.forward_literal(&diagram.get_node(&best_node_label), req, res);

                if let Some(req) = req.as_mut_any().downcast_mut::<RequestStruct>() {
                    if let Some(res) = res.as_any().downcast_ref::<ResponseStruct>() {
                        req.caret = res.caret;
                    };
                };

                has_token = true;
                res.set_caret(0);
            } else if best_node_re_label != "" {
                res.set_caret(req.get_caret());
                self.forward_reg(req, res);

                if let Some(req) = req.as_mut_any().downcast_mut::<RequestStruct>() {
                    if let Some(res) = res.as_any().downcast_ref::<ResponseStruct>() {
                        req.caret = res.caret;
                    } else {
                        panic!("Downcast fail.");
                    }
                } else {
                    panic!("Downcast fail.");
                }

                res.set_caret(0);

                // 一方に、まとめる。
                has_token = true;
                best_node_label = best_node_re_label;
            }

            if has_token {
                res.set_caret(req.get_caret());
                res.forward(NEXT_EXIT_LABEL); // デフォルト値。

                // 次のノード名に変更する。
                self.diagram_player.set_current(&best_node_label.to_string());

                let node = &diagram.get_node(&best_node_label);

                // あれば、コントローラーに処理を移譲。
                if node.get_fn_label() == "" {
                    // コントローラーを指定していなければ、出口ラベルは、デフォルト値のまま。
                } else if diagram.contains_fn(&node.get_fn_label()) {
                    (diagram.get_fn(&node.get_fn_label()))(t, req, res);
                } else {
                    // 無い関数が設定されていた場合は、コンソール表示だけする。
                    println!(
                        "IGNORE: \"{}\" fn (in {} node) is not found.",
                        &node.get_fn_label(),
                        best_node_label
                    );
                }

                // ****************************************************************************************************
                //  (指定があるなら)行終了を「登録」。(行終了するわけではない)
                // ****************************************************************************************************
                if node.contains_exit(&NEWLINE_EXIT_LABEL.to_string()) {
                    // 次の「行末」ノードへ。抽出するノード ラベルは 必ず先頭の1つだけ とする。
                    let tail_node_label = &node.get_exit_vec(&NEWLINE_EXIT_LABEL.to_string())[0];

                    // 「行末」の関数を「登録」する。
                    let tail_node = diagram.get_node(&tail_node_label);
                    let fn_label = tail_node.get_fn_label();
                    if diagram.contains_fn(&fn_label) {
                        current_newline_fn = *diagram.get_fn(&fn_label);
                    } else {
                        // 無い関数が設定されていた場合は、コンソール表示だけする。
                        println!(
                            "IGNORE: \"{}\" fn (in {} node) is not found.",
                            &fn_label, NEWLINE_EXIT_LABEL
                        );
                    }

                    // 次の「行頭」ノードを「登録」。抽出するノード ラベルは 必ず先頭の1つだけ とする。
                    registered_next_head_node_label =
                        tail_node.get_exit_vec(NEXT_EXIT_LABEL)[0].to_string();
                    /*
                    println!(
                        "行終了登録 tail_node_label: [{}], registered_next_head_node_label: [{}].",
                        tail_node_label, registered_next_head_node_label
                    );
                     */
                }

                // ****************************************************************************************************
                // * 次の行き先に遷移。（フォワードを受け取り）                                                           *
                // ****************************************************************************************************
                if let Some(req) = req.as_mut_any().downcast_mut::<RequestStruct>() {
                    if let Some(res) = res.as_any().downcast_ref::<ResponseStruct>() {
                        req.caret = res.caret;

                        if res.exit_label == ""
                            || !node.contains_exit(&res.exit_label.to_string())
                        {
                            // 未指定（デフォルト値ではなくて）なら、次の行き先は無し。
                            current_exit_vec = empty_exit_vec;
                        } else {
                            current_exit_vec = node.get_exit_vec(&res.exit_label.to_string());
                        }
                    // current_exit_vec は無くてもいい。 panic!("\"{}\" next node (of \"{}\" node) alies is not found.", res.exit_label.to_string(), best_node_label)
                    } else {
                        panic!("Downcast fail.");
                    }
                } else {
                    panic!("Downcast fail.");
                }

                res.set_caret(0);
                res.forward(NEXT_EXIT_LABEL); // デフォルト値にリセット。

                if let Some(res) = res.as_any().downcast_ref::<ResponseStruct>() {
                    if res.done_line {
                        // 行解析の終了。
                        let len = req.get_line_len();

                        if let Some(req) = req.as_mut_any().downcast_mut::<RequestStruct>() {
                            req.caret = len;
                        } else {
                            panic!("Downcast fail.");
                        }
                    }
                } else {
                    panic!("Downcast fail.");
                }
            } else {
                // 何とも一致しなかったら実行します
                self.parse_line_else(&diagram, t, req, res);
                // 次のラインへ。
                break 'line;
            }

            if let Some(res) = res.as_any().downcast_ref::<ResponseStruct>() {
                use diagram::ResponseOption::*;
                match res.option {
                    None => {}
                    Quits => {
                        return;
                    }
                    Reloads(ref _file) => {
                        return;
                    }
                    Saves(ref _file) => {
                        return;
                    }
                }
            } else {
                panic!("Downcast fail.");
            }

            // 次のトークンへ。
        }

        // ****************************************************************************************************
        //  改行（1行読取）に対応したコールバック関数を実行。
        // ****************************************************************************************************
        (current_newline_fn)(t, req, res); // responseは無視する。
        self.diagram_player.set_current(&registered_next_head_node_label);
        /*
        println!(
            "行終了 self.current_label: [{}].",
            self.current_label
        );
         */
    }
    // cyclomatic complexity を避けたいだけ。
    fn parse_line_else(
        &self,
        diagram: &Diagram<T>,
        t: &mut T,
        req: &mut dyn Request,
        res: &mut dyn Response,
    ) {
        if diagram.contains_node(&ELSE_NODE_LABEL.to_string()) {
            let fn_label = diagram
                .get_node(&ELSE_NODE_LABEL.to_string())
                .get_fn_label();
            if diagram.contains_fn(&fn_label) {
                // ****************************************************************************************************
                //  コールバック関数を実行。
                // ****************************************************************************************************
                (diagram.get_fn(&fn_label))(t, req, res);
            // responseは無視する。
            } else {
                // 無い関数が設定されていた場合は、コンソール表示だけする。
                println!(
                    "IGNORE: \"{}\" fn (in {} node) is not found.",
                    &fn_label, ELSE_NODE_LABEL
                );
            }
        }
    }

    /// 次に一致するノード名。
    fn next_node_label(
        &self,
        diagram: &Diagram<T>,
        req: &mut dyn Request,
        current_exit_map: &[String],
    ) -> (String, String) {
        // 一番優先されるものを探す。
        let mut best_node_label = "".to_string();
        let mut best_node_re_label = "".to_string();

        // 次の候補。
        let mut max_token_len = 0;
        for i_next_node_label in current_exit_map {
            let next_node_label = i_next_node_label.trim();
            // println!("next_node_label: {}", next_node_label);
            if diagram.contains_node(&next_node_label.to_string()) {
                //println!("contains.");

                let node_name = next_node_label.to_string();
                let node = &diagram.get_node(&node_name);

                let matched;
                if node.is_regex() {
                    if self.starts_with_reg(node, req) {
                        // 正規表現で一致したなら。
                        best_node_re_label = node_name;
                        // 固定長で一致するものも探したい。
                    }
                } else {
                    matched = self.starts_with_literal(node, req);
                    if matched {
                        //println!("starts_with_literal.");
                        let token_len = node.get_token().chars().count();
                        if max_token_len < token_len {
                            max_token_len = token_len;
                            best_node_label = node_name;
                            // まだ、一番長い、固定長トークンを探したい。
                        };
                        //} else {
                        //    println!("not starts_with_literal. req.line={}, req.line_len={}, res.starts={}", req.line, req.line_len, res.starts);
                    }
                }
            }
        }
        (best_node_label, best_node_re_label)
    }
}
