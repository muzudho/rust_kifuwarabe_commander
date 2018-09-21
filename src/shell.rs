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
use node::*;
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
pub struct Request {
    pub line: Box<String>, // String型は長さが可変なので、固定長のBoxでラップする。
    pub line_len: usize,
    pub caret: usize,
    pub groups: Box<Vec<String>>,
}

fn new_request(line2: Box<String>) -> Box<Request> {
    let len = line2.chars().count();
    Box::new(Request {
        line: line2,
        line_len: len,
        caret: 0,
        groups: Box::new(Vec::new()),
    })
}

impl RequestAccessor for Request {
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
    fn get_line(&self) -> &Box<String> {
        &self.line
    }
    fn get_line_len(&self) -> usize {
        self.line_len
    }
    fn get_caret(&self) -> usize {
        self.caret
    }
    fn get_groups(&self) -> &Box<Vec<String>> {
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
/// * `groups` - あれば、正規表現の結果を入れておく。
/// * `next_node_alies` - 次のノードの登録名です。カンマ区切り。
pub struct Response<T> {
    pub caret: usize,
    pub done_line: bool,
    pub quits: bool,
    pub next_node_alies: &'static str,
    pub linebreak_controller_changed: bool,
    pub linebreak_controller: Controller<T>,
}

fn new_response<T>() -> Box<Response<T>> {
    Box::new(Response {
        caret: 0,
        done_line: false,
        quits: false,
        next_node_alies: "",
        linebreak_controller_changed: false,
        linebreak_controller: empty_controller,
    })
}

impl<T: 'static> ResponseAccessor<T> for Response<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn forward(&mut self, next_node_alies2: &'static str) {
        self.next_node_alies = next_node_alies2
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
    fn set_linebreak_controller_changed(&mut self, value: bool) {
        self.linebreak_controller_changed = value
    }
    fn set_linebreak_controller(&mut self, value: Controller<T>) {
        self.linebreak_controller = value
    }
}

/// [token]文字列の長さだけ [starts]キャレットを進めます。
/// [token]文字列の続きに半角スペース「 」が１つあれば、1つ分だけ読み進めます。
///
/// # Arguments
///
/// * `request` - 読み取るコマンドラインと、読取位置。
/// * returns - 一致したら真。
pub fn starts_with<T, S: ::std::hash::BuildHasher>(node: &Node<T, S>, request: &Box<RequestAccessor>) -> bool {
    let caret_end = request.get_caret() + node.token.len();
    //println!("response.starts={} + self.token.len()={} <= request.line_len={} [{}]==[{}]", response.starts, self.token.len(), request.line_len,
    //    &request.line[response.starts..caret_end], self.token);
    caret_end <= request.get_line_len()
        && &request.get_line()[request.get_caret()..caret_end] == node.token
}

/// 正規表現を使う。
///
/// # Arguments
///
/// * `request` - 読み取るコマンドライン。
/// * returns - 一致したら真。
pub fn starts_with_re<T, S: ::std::hash::BuildHasher>(
    node: &Node<T, S>,
    request: &mut Box<RequestAccessor>,
) -> bool {
    if VERBOSE {
        println!("starts_with_re");
    }

    if request.get_caret() < request.get_line_len() {
        if VERBOSE {
            println!("node.token: {}", node.token);
        }

        let re = Regex::new(node.token).unwrap();

        let text;
        let mut group_num = 0;
        if let Some(req) = request.as_mut_any().downcast_mut::<Request>() {
            req.groups.clear();
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

fn forward<T: 'static, S: ::std::hash::BuildHasher>(
    node: &Node<T, S>,
    request: &Box<RequestAccessor>,
    response: &mut Box<dyn ResponseAccessor<T>>,
) {
    response.set_caret(request.get_caret() + node.token.len());
    let res_caret;
    if let Some(res) = response.as_any().downcast_ref::<Response<T>>() {
        res_caret = res.caret;
    } else {
        panic!("Downcast fail.");
    }

    // 続きにスペース「 」が１つあれば読み飛ばす
    if 0 < (request.get_line_len() - res_caret)
        && &request.get_line()[res_caret..(res_caret + 1)] == " "
    {
        response.set_caret(res_caret + 1);
    }
}

/// TODO キャレットを進める。正規表現はどこまで一致したのか分かりにくい。
fn forward_re<T: 'static>(
    request: &Box<RequestAccessor>,
    response: &mut Box<dyn ResponseAccessor<T>>,
) {
    let res_caret;
    if let Some(res) = response.as_any().downcast_ref::<Response<T>>() {
        res_caret = res.caret;
    } else {
        panic!("Downcast fail.");
    }

    let pseud_token_len = request.get_groups()[0].chars().count();
    response.set_caret(request.get_caret() + pseud_token_len);
    // 続きにスペース「 」が１つあれば読み飛ばす
    if 0 < (request.get_line_len() - res_caret)
        && &request.get_line()[res_caret..(res_caret + 1)] == " "
    {
        response.set_caret(res_caret + 1);
    }
}

/// シェル。
///
/// # Arguments
///
/// * `vec_row` - コマンドを複数行 溜めておくバッファーです。
/// * `node_table` - 複数件のトークンです。
/// * `next` - カンマ区切りの登録ノード名です。
pub struct Shell {
    vec_row: Vec<String>,
    pub next: &'static str,
}

pub fn new_shell() -> Shell {
    Shell {
        vec_row: Vec::new(),
        next: "",
    }
}

pub fn set_next(shell: &mut Shell, next2: &'static str) {
    shell.next = next2;
}

/// コマンドを1行も入力していなければ真を返します。
pub fn is_empty(shell: &Shell) -> bool {
    shell.vec_row.len() == 0
}

/// コンソール入力以外の方法で、コマンド1行を追加したいときに使います。
/// 行の末尾に改行は付けないでください。
pub fn push_row(shell: &mut Shell, row: &str) {
    shell.vec_row.push(format!("{}\n", row));
}

/// 先頭のコマンド1行をキューから削除して返します。
pub fn pop_row(shell: &mut Shell) -> Box<String> {
    Box::new(shell.vec_row.pop().unwrap())
}

/// コマンドラインの入力受付、および コールバック関数呼出を行います。
/// スレッドはブロックします。
/// 強制終了する場合は、 [Ctrl]+[C] を入力してください。
pub fn run<T: 'static, S: ::std::hash::BuildHasher>(graph: &Graph<T, S>, shell: &mut Shell, t: &mut T) {
    'lines: loop {
        // リクエストは、キャレットを更新するのでミュータブル。
        let mut request: Box<dyn RequestAccessor> = if is_empty(shell) {
            let mut line_string = String::new();
            // コマンド プロンプトからの入力があるまで待機します。
            io::stdin()
                .read_line(&mut line_string)
                .expect("info Failed to read_line"); // OKでなかった場合のエラーメッセージ。

            // 末尾の 改行 を除きます。前後の空白も消えます。
            line_string = line_string.trim().parse().expect("info Failed to parse");

            new_request(Box::new(line_string))
        } else {
            // バッファーの先頭行です。
            new_request(pop_row(shell))
        };

        if parse_line(graph, shell, t, &mut request) {
            break 'lines;
        }
    } // loop
}

/// # Returns.
///
/// 0. シェルを終了するなら真。
fn parse_line<T: 'static, S: ::std::hash::BuildHasher>(
    graph: &Graph<T, S>,
    shell: &mut Shell,
    t: &mut T,
    request: &mut Box<dyn RequestAccessor>,
) -> bool {
    let mut response: Box<dyn ResponseAccessor<T>> = new_response::<T>();
    let mut next_node_list = shell.next;
    let mut current_linebreak_controller: Controller<T> = empty_controller;

    'line: while request.get_caret() < request.get_line_len() {
        // キャレットの位置を、レスポンスからリクエストへ移して、次のトークンへ。
        reset(&mut response);
        let mut is_done = false;
        let mut is_done_re = false;

        let vec_next: Vec<&str>;
        {
            let split = next_node_list.split(',');
            // for s in split {
            //     println!("{}", s)
            // }
            vec_next = split.collect();
        }

        // 最初は全てのノードが対象。
        let mut max_token_len = 0;
        
        let mut best_node_name = "".to_string();
        let mut best_node_re_name = "".to_string();

        // 次の候補。
        for i_next_node_name in vec_next {
            let next_node_name = i_next_node_name.trim();
            // println!("next_node_name: {}", next_node_name);
            if contains_node(graph, &next_node_name.to_string()) {
                //println!("contains.");

                let node_name = next_node_name.to_string();
                let node = &graph.node_table[&node_name];

                let matched;
                if node.token_regex {
                    if starts_with_re(node, request) {
                        // 正規表現で一致したなら。
                        best_node_re_name = node_name;
                        is_done_re = true;
                    }
                } else {
                    matched = starts_with(node, request);
                    if matched {
                        // 固定長トークンで一致したなら。
                        //println!("starts_with.");
                        let token_len = node.token.chars().count();
                        if max_token_len < token_len {
                            max_token_len = token_len;
                            best_node_name = node_name;
                        };
                        is_done = true;
                        //} else {
                        //    println!("not starts_with. request.line={}, request.line_len={}, response.starts={}", request.line, request.line_len, response.starts);
                    }
                }
            }
        }

        // キャレットを進める。
        if is_done || is_done_re {
            if is_done {
                response.set_caret(request.get_caret());
                forward(&graph.node_table[&best_node_name], request, &mut response);

                if let Some(req) = request.as_mut_any().downcast_mut::<Request>() {
                    if let Some(res) = response.as_any().downcast_ref::<Response<T>>() {
                        req.caret = res.caret;
                    };
                };

                response.set_caret(0);
            } else {
                response.set_caret(request.get_caret());
                forward_re(request, &mut response);

                if let Some(req) = request.as_mut_any().downcast_mut::<Request>() {
                    if let Some(res) = response.as_any().downcast_ref::<Response<T>>() {
                        req.caret = res.caret;
                    } else {
                        panic!("Downcast fail.");
                    }
                } else {
                    panic!("Downcast fail.");
                }

                response.set_caret(0);

                // まとめる。
                is_done = is_done_re;
                best_node_name = best_node_re_name;
            }
        }

        if is_done {
            // コントローラーに処理を移譲。
            response.set_caret(request.get_caret());
            response.forward("");
            (&graph.node_table[&best_node_name].controller)(t, request, &mut response);

            if let Some(req) = request.as_mut_any().downcast_mut::<Request>() {
                if let Some(res) = response.as_any().downcast_ref::<Response<T>>() {
                    req.caret = res.caret;

                    if res.next_node_alies == "" {
                        next_node_list = "";
                    } else {
                        next_node_list = &graph.node_table[&best_node_name].next_link[res.next_node_alies];
                    }

                    // 行終了時コントローラーの更新
                    if res.linebreak_controller_changed {
                        current_linebreak_controller = res.linebreak_controller;
                    }
                } else {
                    panic!("Downcast fail.");
                }
            } else {
                panic!("Downcast fail.");
            }

            response.set_caret(0);
            response.forward("");


            if let Some(res) = response.as_any().downcast_ref::<Response<T>>() {
                if res.done_line {
                    // 行解析の終了。
                    let len = request.get_line_len();

                    if let Some(req) = request.as_mut_any().downcast_mut::<Request>() {
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
            (graph.complementary_controller)(t, request, &mut response);
            // responseは無視する。

            // 次のラインへ。
            break 'line;
        }

        if let Some(res) = response.as_any().downcast_ref::<Response<T>>() {
            if res.quits {
                // ループを抜けて、アプリケーションを終了します。
                return true;
            }
        } else {
            panic!("Downcast fail.");
        }
    }

    // 1行読取終了。
    (current_linebreak_controller)(t, request, &mut response);
    // responseは無視する。
    false
}
