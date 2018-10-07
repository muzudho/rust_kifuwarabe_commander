use diagram::*;
/// 行単位のパーサー
use diagram_player::*;
use regex::Regex;
use shell::*;

/// 不具合を取りたいときに真にする。
const VERBOSE: bool = false;

pub struct LineParser {}
impl LineParser {
    /// 行単位パーサー。
    ///
    /// # Arguments.
    ///
    /// * `diagram` - この関数の中では、 Diagram をイミュータブルに保つ。 Diagram の編集は この関数の外で行う。
    /// * `req` - １行分だけ切り取ってテキストを送り返してくる。行をまたいでマッチできない。トークンに分解して送ってくることもできない。
    ///
    /// # Returns.
    ///
    /// 0. シェルを終了するなら真。
    pub fn run_on_line<T>(
        diagram_player: &mut DiagramPlayer,
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
        diagram_player.enter_when_out(&diagram);
        // まず 現在ノードを取得。
        let current_node = diagram.get_node(&diagram_player.get_current());

        current_exit_vec = match &current_node.get_exit_map().get(NEXT_EXIT_LABEL) {
            Some(n) => n,
            None => panic!(
                "run_on_line Get_exit_map: [{}] node - [{}] is not found.",
                diagram_player.get_current(),
                NEXT_EXIT_LABEL
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
                LineParser::next_node_label(diagram, req, current_exit_vec);

            // キャレットを進める。
            let mut has_token = false;
            if best_node_label != "" {
                // 固定長での一致を優先。
                res.set_caret(req.get_caret());
                LineParser::parse_literal(&diagram.get_node(&best_node_label), req, res);

                if let Some(req) = req.as_mut_any().downcast_mut::<RequestStruct>() {
                    if let Some(res) = res.as_any().downcast_ref::<ResponseStruct>() {
                        req.caret = res.caret;
                    };
                };

                has_token = true;
                res.set_caret(0);
            } else if best_node_re_label != "" {
                res.set_caret(req.get_caret());
                LineParser::parse_reg(req, res);

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
                diagram_player.set_current(&best_node_label.to_string());

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

                        if res.exit_label == "" || !node.contains_exit(&res.exit_label.to_string())
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
                LineParser::parse_line_else(&diagram, t, req, res);
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
        diagram_player.set_current(&registered_next_head_node_label);
        /*
        println!(
            "行終了 self.current_label: [{}].",
            self.current_label
        );
         */
    }

    // cyclomatic complexity を避けたいだけ。
    pub fn parse_line_else<T>(
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
    fn next_node_label<T>(
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
                    if LineParser::starts_with_reg(node, req) {
                        // 正規表現で一致したなら。
                        best_node_re_label = node_name;
                        // 固定長で一致するものも探したい。
                    }
                } else {
                    matched = LineParser::starts_with_literal(node, req);
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

    fn parse_literal(node: &Node, req: &dyn Request, res: &mut dyn Response) {
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
    fn parse_reg(req: &dyn Request, res: &mut dyn Response) {
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

    /// [token]文字列の長さだけ [starts]キャレットを進めます。
    /// [token]文字列の続きに半角スペース「 」が１つあれば、1つ分だけ読み進めます。
    ///
    /// # Arguments
    ///
    /// * `req` - 読み取るコマンドラインと、読取位置。
    /// * returns - 一致したら真。
    fn starts_with_literal(node: &Node, req: &mut dyn Request) -> bool {
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
    fn starts_with_reg(node: &Node, req: &mut dyn Request) -> bool {
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
}
