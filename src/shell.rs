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
use node::RequestTrait;
use regex::Regex;
use std::io;

impl RequestTrait for Request {
    fn new(line2: String) -> Request {
        let len = line2.chars().count();
        Request {
            line: line2,
            line_len: len,
            caret: 0,
        }
    }
}

/// 不具合を取りたいときに真にする。
const VERBOSE: bool = false;

/// [token]文字列の長さだけ [starts]キャレットを進めます。
/// [token]文字列の続きに半角スペース「 」が１つあれば、1つ分だけ読み進めます。
///
/// # Arguments
///
/// * `request` - 読み取るコマンドラインと、読取位置。
/// * returns - 一致したら真。
pub fn starts_with<T>(node: &Node<T>, request: &Request) -> bool {
    let caret_end = request.caret + node.token.len();
    //println!("response.starts={} + self.token.len()={} <= request.line_len={} [{}]==[{}]", response.starts, self.token.len(), request.line_len,
    //    &request.line[response.starts..caret_end], self.token);
    caret_end <= request.line_len && &request.line[request.caret..caret_end] == node.token
}

/// 正規表現を使う。
///
/// # Arguments
///
/// * `request` - 読み取るコマンドライン。
/// * `response` - 読取位置。
/// * returns - 一致したら真。
pub fn starts_with_re<T>(node: &Node<T>, request: &Request, response: &mut Response<T>) -> bool {
    if VERBOSE {
        println!("starts_with_re");
    }

    if request.caret < request.line_len {
        if VERBOSE {
            println!("node.token: {}", node.token);
        }

        let re = Regex::new(node.token).unwrap();

        let text = &request.line[request.caret..];

        if VERBOSE {
            println!("text: [{}]", text);
        }

        let mut group_num = 0;
        response.groups.clear();
        for caps in re.captures_iter(text) {
            // caps は サイズ 2 の配列 で同じものが入っている。
            let cap = &caps[0];

            response.groups.push(cap.to_string());

            group_num += 1;
        }

        if VERBOSE {
            println!("Group num: {}", group_num);
        }

        0 < group_num
    } else {
        false
    }
}

fn forward<T>(node: &Node<T>, request: &Request, response: &mut Response<T>) {
    response.caret = request.caret + node.token.len();
    // 続きにスペース「 」が１つあれば読み飛ばす
    if 0 < (request.line_len - response.caret)
        && &request.line[response.caret..(response.caret + 1)] == " "
    {
        response.caret += 1;
    }
}

/// TODO キャレットを進める。正規表現はどこまで一致したのか分かりにくい。
fn forward_re<T>(request: &Request, response: &mut Response<T>) {
    let pseud_token_len = response.groups[0].chars().count();
    response.caret = request.caret + pseud_token_len;
    // 続きにスペース「 」が１つあれば読み飛ばす
    if 0 < (request.line_len - response.caret)
        && &request.line[response.caret..(response.caret + 1)] == " "
    {
        response.caret += 1;
    }
}

/// シェル。
///
/// # Arguments
///
/// * `vec_row` - コマンドを複数行 溜めておくバッファーです。
/// * `node_table` - 複数件のトークンです。
/// * `complementary_controller` - トークン マッピングに一致しなかったときに呼び出されるコールバック関数の名前です。
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

pub fn set_next(shell: &mut Shell, next: &'static str) {
    shell.next = next;
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
pub fn pop_row(shell: &mut Shell) -> String {
    shell.vec_row.pop().unwrap()
}

/// コマンドラインの入力受付、および コールバック関数呼出を行います。
/// スレッドはブロックします。
/// 強制終了する場合は、 [Ctrl]+[C] を入力してください。
pub fn run<T>(graph: &Graph<T>, shell: &mut Shell, t: &mut T) {
    'lines: loop {
        // リクエストは、キャレットを更新するのでミュータブル。
        let mut request = if is_empty(shell) {
            let mut line_string = String::new();
            // コマンド プロンプトからの入力があるまで待機します。
            io::stdin()
                .read_line(&mut line_string)
                .expect("info Failed to read_line"); // OKでなかった場合のエラーメッセージ。

            // 末尾の 改行 を除きます。前後の空白も消えます。
            line_string = line_string.trim().parse().expect("info Failed to parse");

            Request::new(line_string)
        } else {
            // バッファーの先頭行です。
            Request::new(pop_row(shell))
        };

        if parse_line(graph, shell, t, &mut request) {
            break 'lines;
        }
    } // loop
}

/// # Returns.
///
/// 0. シェルを終了するなら真。
fn parse_line<T>(
    graph: &Graph<T>,
    shell: &mut Shell,
    t: &mut T,
    request: &mut Request,
) -> bool {
    let mut response = new_response();
    let mut next = shell.next;
    let mut current_linebreak_controller: Controller<T> = empty_controller;

    'line: while request.caret < request.line_len {
        // キャレットの位置を、レスポンスからリクエストへ移して、次のトークンへ。
        reset(&mut response);
        let mut is_done = false;
        let mut is_done_re = false;

        let vec_next: Vec<&str>;
        {
            let split = next.split(',');
            // for s in split {
            //     println!("{}", s)
            // }
            vec_next = split.collect();
        }

        // 最初は全てのノードが対象。
        let mut max_token_len = 0;
        let mut best_node: &Node<T> = &Node {
            token: "",
            controller: empty_controller,
            token_regex: false,
        };
        let mut best_node_re: &Node<T> = &Node {
            token: "",
            controller: empty_controller,
            token_regex: false,
        };

        // 次の候補。
        for i_next_node_name in vec_next {
            let next_node_name = i_next_node_name.trim();
            // println!("next_node_name: {}", next_node_name);
            if contains_node(graph, &next_node_name.to_string()) {
                //println!("contains.");

                let node = &graph.node_table[&next_node_name.to_string()];

                let matched;
                if node.token_regex {
                    if starts_with_re(node, &request, &mut response) {
                        // 正規表現で一致したなら。
                        best_node_re = node;
                        is_done_re = true;
                    }
                } else {
                    matched = starts_with(node, &request);
                    if matched {
                        // 固定長トークンで一致したなら。
                        //println!("starts_with.");
                        let token_len = node.token.chars().count();
                        if max_token_len < token_len {
                            max_token_len = token_len;
                            best_node = node;
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
                response.caret = request.caret;
                forward(&best_node, &request, &mut response);
                request.caret = response.caret;
                response.caret = 0;
            } else {
                response.caret = request.caret;
                forward_re(&request, &mut response);
                request.caret = response.caret;
                response.caret = 0;

                // まとめる。
                is_done = is_done_re;
                best_node = best_node_re;
            }
        }

        if is_done {
            // コントローラーに処理を移譲。
            response.caret = request.caret;
            response.next = next;
            (best_node.controller)(t, &request, &mut response);
            request.caret = response.caret;
            next = response.next;
            response.caret = 0;
            response.next = "";
            //println!("New next: {}", next);

            // 行終了時コントローラーの更新
            if is_linebreak_controller_changed(&response) {
                current_linebreak_controller = response.linebreak_controller;
            }

            if response.done_line {
                // 行解析の終了。
                request.caret = request.line_len;
            }
        } else {
            // 何とも一致しなかったら実行します。
            (graph.complementary_controller)(t, &request, &mut response);
            // caret や、next が変更されていても、無視する。

            // 次のラインへ。
            break 'line;
        }

        if response.quits {
            // ループを抜けて、アプリケーションを終了します。
            return true;
        }
    }

    // 1行読取終了。
    (current_linebreak_controller)(t, &request, &mut response);
    // caret や、next が変更されていても、無視する。
    false
}
