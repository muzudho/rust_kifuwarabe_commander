/// クライアント１つにつき、１つのシェルを与えます。
/// 行単位です。
///
/// コマンド例
///
/// ```
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shell
/// cargo clippy
/// ```
use graph::*;
use regex::Regex;
use std::any::Any; // https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait
use std::io;

/// 不具合を取りたいときに真にする。
const VERBOSE: bool = false;

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
/// * `quits` - アプリケーションを終了するなら真にします。
/// * `next_node_alies` - 次のノードの登録名です。カンマ区切り。
pub struct ResponseStruct {
    pub caret: usize,
    pub done_line: bool,
    pub quits: bool,
    pub reloads: &'static str,
    pub next_node_alies: String,
}
impl ResponseStruct {
    fn new() -> ResponseStruct {
        ResponseStruct {
            caret: 0,
            done_line: false,
            quits: false,
            reloads: "",
            next_node_alies: "".to_string(),
        }
    }
    fn reset(&mut self) {
        self.set_caret(0);
        self.set_done_line(false);
        self.set_quits(false);
        self.set_reloads("");
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
    fn forward(&mut self, next_node_alies2: &'static str) {
        self.next_node_alies = next_node_alies2.to_string();
    }
    fn set_caret(&mut self, caret2: usize) {
        self.caret = caret2
    }
    fn set_done_line(&mut self, done_line2: bool) {
        self.done_line = done_line2
    }
    fn set_quits(&mut self, quits2: bool) {
        self.quits = quits2
    }
    fn set_reloads(&mut self, value: &'static str) {
        self.reloads = value
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
            vec_row: Vec::new(),
            reader: standard_input_reader,
        }
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
        let caret_end = req.get_caret() + node.token.len();
        caret_end <= req.get_line_len() && req.get_line()[req.get_caret()..caret_end] == node.token
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
                println!("node.token: {}", node.token);
            }

            let re = Regex::new(&node.token).unwrap();

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
        res.set_caret(req.get_caret() + node.token.len());
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
    pub fn run(&mut self, graph: &Graph<T>, t: &mut T) {
        'lines: loop {
            // リクエストは、キャレットを更新するのでミュータブル。
            let mut req = if self.is_empty() {
                let line_string = (self.reader)(t);
                RequestStruct::new(Box::new(line_string))
            } else {
                // バッファーの先頭行です。
                RequestStruct::new(self.pop_row())
            };

            if self.parse_line(graph, t, &mut req) {
                break 'lines;
            }
        }
    }

    /// # Returns.
    ///
    /// 0. シェルを終了するなら真。
    fn parse_line(&self, graph: &Graph<T>, t: &mut T, req: &mut dyn Request) -> bool {
        let empty_exits = &Vec::new();
        let res: &mut dyn Response = &mut ResponseStruct::new();
        let mut current_exits: &Vec<String> = graph.get_entrance_vec();
        let mut current_newline_fn: Controller<T> = empty_controller;

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
            let (mut best_node_name, best_node_re_name) =
                self.next_node_name(graph, req, current_exits);

            // キャレットを進める。
            let mut is_done = false;
            if best_node_name != "" {
                res.set_caret(req.get_caret());
                self.forward_literal(&graph.get_node(&best_node_name), req, res);

                if let Some(req) = req.as_mut_any().downcast_mut::<RequestStruct>() {
                    if let Some(res) = res.as_any().downcast_ref::<ResponseStruct>() {
                        req.caret = res.caret;
                    };
                };

                is_done = true;
                res.set_caret(0);
            } else if best_node_re_name != "" {
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
                is_done = true;
                best_node_name = best_node_re_name;
            }

            if is_done {
                res.set_caret(req.get_caret());
                res.forward("");
                let node = &graph.get_node(&best_node_name);

                // あれば、コントローラーに処理を移譲。
                if &node.fn_label == "" {
                    // デフォルトで next を選ぶ。
                    res.forward("next");
                } else if graph.contains_fn(&node.fn_label) {
                    (graph.get_fn(&node.fn_label))(t, req, res);
                } else {
                    panic!(
                        "\"{}\" fn (in {} node) is not found. Please use contains_fn().",
                        &node.fn_label, best_node_name
                    );
                }

                // 行終了時コントローラーの更新。指定がなければ無視。
                if node.contains_exits(&"#newline".to_string()) {
                    // 対応するノードは 1つだけとする。
                    let next_node = &node.get_exits(&"#newline".to_string())[0];
                    current_newline_fn = *graph.get_fn(&graph.get_node(&next_node).fn_label);
                }

                // フォワードを受け取り。
                if let Some(req) = req.as_mut_any().downcast_mut::<RequestStruct>() {
                    if let Some(res) = res.as_any().downcast_ref::<ResponseStruct>() {
                        req.caret = res.caret;

                        if res.next_node_alies == ""
                            || !node.contains_exits(&res.next_node_alies.to_string())
                        {
                            current_exits = empty_exits;
                        } else {
                            current_exits = node.get_exits(&res.next_node_alies.to_string());
                        }
                    // current_exits は無くてもいい。 panic!("\"{}\" next node (of \"{}\" node) alies is not found.", res.next_node_alies.to_string(), best_node_name)
                    } else {
                        panic!("Downcast fail.");
                    }
                } else {
                    panic!("Downcast fail.");
                }

                res.set_caret(0);
                res.forward("");

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
                // 何とも一致しなかったら実行します。
                if graph.contains_node(&"#else".to_string()) {
                    (graph.get_fn(&graph.get_node(&"#else".to_string()).fn_label))(t, req, res);
                    // responseは無視する。
                }

                // 次のラインへ。
                break 'line;
            }

            if let Some(res) = res.as_any().downcast_ref::<ResponseStruct>() {
                if res.quits {
                    // ループを抜けて、アプリケーションを終了します。
                    return true;
                }
                if res.reloads != "" {}
            } else {
                panic!("Downcast fail.");
            }

            // 次のトークンへ。
        }

        // 1行読取終了。
        (current_newline_fn)(t, req, res);
        // responseは無視する。
        false
    }

    /// 一致するノード名。
    fn next_node_name(
        &self,
        graph: &Graph<T>,
        req: &mut dyn Request,
        current_exits: &[String],
    ) -> (String, String) {
        let mut best_node_name = "".to_string();
        let mut best_node_re_name = "".to_string();

        // 次の候補。
        let mut max_token_len = 0;
        for i_next_node_name in current_exits {
            let next_node_name = i_next_node_name.trim();
            // println!("next_node_name: {}", next_node_name);
            if graph.contains_node(&next_node_name.to_string()) {
                //println!("contains.");

                let node_name = next_node_name.to_string();
                let node = &graph.get_node(&node_name);

                let matched;
                if node.token_regex {
                    if self.starts_with_reg(node, req) {
                        // 正規表現で一致したなら。
                        best_node_re_name = node_name;
                    }
                } else {
                    matched = self.starts_with_literal(node, req);
                    if matched {
                        // 一番長い、固定長トークンの一致を探す。
                        //println!("starts_with_literal.");
                        let token_len = node.token.chars().count();
                        if max_token_len < token_len {
                            max_token_len = token_len;
                            best_node_name = node_name;
                        };
                        //} else {
                        //    println!("not starts_with_literal. req.line={}, req.line_len={}, res.starts={}", req.line, req.line_len, res.starts);
                    }
                }
            }
        }
        (best_node_name, best_node_re_name)
    }
}
