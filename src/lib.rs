/// # Rust きふわらべ シェル
/// 行単位です。
extern crate regex;

/// 不具合を取りたいときに真にする。
const VERBOSE : bool = false;

use regex::Regex;
use std::collections::HashMap;
use std::io;

/// コマンドライン文字列。
/// 
/// # Members
/// 
/// * `line` - コマンドライン文字列の1行全体です。
/// * `line_len` - コマンドライン文字列の1行全体の文字数です。
pub struct Request {
    pub line: String,
    pub line_len: usize,
    pub caret: usize,
}
impl Request {
    pub fn new(line: String) -> Request {
        let len = line.chars().count();
        Request {
            line: line,
            line_len: len,
            caret: 0,
        }
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
/// * `next` - 次のノードの登録名です。カンマ区切り。
pub struct Response {
    pub caret: usize,
    pub done_line: bool,
    pub quits: bool,
    pub groups: Vec<String>,
    pub next: &'static str,
    linebreak_controller_changed: bool,
    linebreak_controller: Controller,
}
impl Response {
    pub fn new() -> Response {
        Response {
            caret: 0,
            done_line: false,
            quits: false,
            groups: Vec::new(),
            next: "",
            linebreak_controller_changed: false,
            linebreak_controller: empty_controller,
        }
    }
    pub fn reset(&mut self) {
        self.caret = 0;
        self.done_line = false;
        self.quits = false;
        self.groups.clear();
        self.next = "";
        self.linebreak_controller_changed = false;
        self.linebreak_controller = empty_controller;
    }
    pub fn set_linebreak_controller(&mut self, controller: Controller) {
        self.linebreak_controller_changed = true;
        self.linebreak_controller = controller;
    }
    pub fn is_linebreak_controller_changed(&self) -> bool {
        self.linebreak_controller_changed
    }
}

/// コールバック関数です。トークンを読み取った時に対応づく作業内容を書いてください。
///
/// # Arguments
///
/// * `request` - 入力されたコマンドライン文字列など。
/// * `response` - 読取位置や、次のトークンの指定など。
///
/// # 参考
/// - Rustのコールバック関数について。  
/// [2016-12-10 Idiomatic callbacks in Rust](https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust)
type Controller = fn(request: &Request, response: &mut Response);

pub fn empty_controller(_request: &Request, _response: &mut Response) {

}

/// トークンと、コントローラーのペアです。
///
/// # Members
///
/// * `token` - 全文一致させたい文字列です。
/// * `controller` - コールバック関数です。
/// * `token_regex` - トークンに正規表現を使うなら真です。
pub struct Node {
    pub token: &'static str,
    pub controller: Controller,
    pub token_regex: bool, 
}
impl Node {
    /// [token]文字列の長さだけ [starts]キャレットを進めます。
    /// [token]文字列の続きに半角スペース「 」が１つあれば、1つ分だけ読み進めます。
    ///
    /// # Arguments
    ///
    /// * `request` - 読み取るコマンドラインと、読取位置。
    /// * returns - 一致したら真。
    pub fn starts_with(&self, request: &Request) -> bool {
        let caret_end = request.caret + self.token.len();
        //println!("response.starts={} + self.token.len()={} <= request.line_len={} [{}]==[{}]", response.starts, self.token.len(), request.line_len,
        //    &request.line[response.starts..caret_end], self.token);
        if caret_end <= request.line_len
            && &request.line[request.caret..caret_end] == self.token {
            true
        } else {
            false
        }
    }

    /// 正規表現を使う。
    ///
    /// # Arguments
    ///
    /// * `request` - 読み取るコマンドライン。
    /// * `response` - 読取位置。
    /// * returns - 一致したら真。
    pub fn starts_with_re(&self, request: &Request, response: &mut Response) -> bool {

        if VERBOSE {
            println!("starts_with_re");
        }

        if request.caret < request.line_len {

            if VERBOSE {
                println!("self.token: {}", self.token);
            }

            let re = Regex::new(self.token).unwrap();

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
            };

            if VERBOSE {
                println!("Group num: {}", group_num);
            }

            0<group_num
        } else {
            false
        }
    }

    pub fn forward(&self, request: &Request, response: &mut Response) {
        response.caret = request.caret + self.token.len();
        // 続きにスペース「 」が１つあれば読み飛ばす
        if 0<(request.line_len-response.caret) && &request.line[response.caret..(response.caret+1)]==" " {
            response.caret += 1;
        }
    }

    /// TODO キャレットを進める。正規表現はどこまで一致したのか分かりにくい。
    pub fn forward_re(&self, request: &Request, response: &mut Response) {
        let pseud_token_len = response.groups[0].chars().count();
        response.caret = request.caret + pseud_token_len;
        // 続きにスペース「 」が１つあれば読み飛ばす
        if 0<(request.line_len-response.caret) && &request.line[response.caret..(response.caret+1)]==" " {
            response.caret += 1;
        }
    }
}

/// このアプリケーションです。
///
/// # Arguments
///
/// * `vec_row` - コマンドを複数行 溜めておくバッファーです。
/// * `node_table` - 複数件のトークンです。
/// * `complementary_controller` - トークン マッピングに一致しなかったときに呼び出されるコールバック関数の名前です。
/// * `next` - カンマ区切りの登録ノード名です。
pub struct Shell{
    empty_node: Node,
    vec_row: Vec<String>,
    node_table: HashMap<String, Node>,
    complementary_controller: Controller,
    pub next: &'static str,
}
impl Shell {
    pub fn new()->Shell{
        Shell {
            empty_node: Node {
                token: "",
                controller: empty_controller,
                token_regex: false,
            },
            vec_row : Vec::new(),
            node_table: HashMap::new(),
            complementary_controller: empty_controller,
            next: "",
        }
    }

    pub fn set_next(&mut self, next: &'static str) {
        self.next = next;
    }

    pub fn contains_node(&self, name: &String) -> bool {
        self.node_table.contains_key(name)
    }

    pub fn get_node(&self, name: &String) -> &Node {
        self.node_table.get(name).unwrap()
    }

    /// # Arguments
    /// 
    /// * `name` - 登録用の名前です。
    /// * `node` - ノードです。
    pub fn insert_node(&mut self, name: &'static str, token: &'static str, controller: Controller){
        self.node_table.insert(
            name.to_string(),
            Node {
                token: token,
                controller: controller,
                token_regex: false,
            }
        );
    }

    /// 正規表現を使うなら。
    /// 
    /// # Arguments
    /// 
    /// * `name` - 登録用の名前です。
    /// * `node` - ノードです。
    pub fn insert_node_re(&mut self, name: &'static str, token: &'static str, controller: Controller){
        self.node_table.insert(
            name.to_string(),
            Node {
                token: token,
                controller: controller,
                token_regex: true,
            }
        );
    }

    /// # Arguments
    /// 
    /// * `map` - 一致するトークンが無かったときに呼び出されるコールバック関数です。
    pub fn set_complementary_controller(&mut self, controller: Controller){
        self.complementary_controller = controller;
    }

    /// コマンドを1行も入力していなければ真を返します。
    pub fn is_empty(&mut self) -> bool {
        self.vec_row.len()==0
    }

    /// コンソール入力以外の方法で、コマンド1行を追加したいときに使います。
    /// 行の末尾に改行は付けないでください。
    pub fn push_row(&mut self, row: &String) {
        self.vec_row.push( format!("{}\n", row ) );
    }

    /// 先頭のコマンド1行をキューから削除して返します。
    pub fn pop_row(&mut self) -> String {
        self.vec_row.pop().unwrap()
    }

    /// コマンドラインの入力受付、および コールバック関数呼出を行います。
    /// スレッドはブロックします。
    /// 強制終了する場合は、 [Ctrl]+[C] を入力してください。
    pub fn run(&mut self) {

        'lines: loop{

            // リクエストは、キャレットを更新するのでミュータブル。
            let mut request : Request;
            if self.is_empty() {
                let mut line_string = String::new();
                // コマンド プロンプトからの入力があるまで待機します。
                io::stdin().read_line(&mut line_string)
                    .ok() // read_line が返す Resultオブジェクト の okメソッド。
                    .expect("info Failed to read_line"); // OKでなかった場合のエラーメッセージ。

                // 末尾の 改行 を除きます。前後の空白も消えます。
                line_string = line_string.trim().parse().ok().expect("info Failed to parse");

                request = Request::new(line_string);
            } else {
                // バッファーの先頭行です。
                request = Request::new(self.pop_row());
            }

            if self.parse_line(&mut request) {
                break 'lines;
            }

        } // loop
    }

    /// # Returns.
    ///
    /// 0. シェルを終了するなら真。
    pub fn parse_line(&mut self, request : &mut Request) -> bool {
        let mut response = Response::new();
        let mut next = self.next;
        let mut current_linebreak_controller : Controller = empty_controller;


        'line: while request.caret < request.line_len {

            // キャレットの位置を、レスポンスからリクエストへ移して、次のトークンへ。
            response.reset();
            let mut is_done = false;
            let mut is_done_re = false;

            let vec_next: Vec<&str>;
            {
                let split = next.split(",");
                // for s in split {
                //     println!("{}", s)
                // }
                vec_next = split.collect();
            }

            // 最初は全てのノードが対象。
            let mut max_token_len = 0;
            let mut best_node = &self.empty_node;
            let mut best_node_re = &self.empty_node;

            // 次の候補。
            for i_next_node_name in vec_next {
                let next_node_name = i_next_node_name.trim();
                // println!("next_node_name: {}", next_node_name);
                if self.contains_node(&next_node_name.to_string()) {
                    //println!("contains.");
                    let node = self.get_node(&next_node_name.to_string());

                    let matched;
                    if node.token_regex {
                        if node.starts_with_re(&request, &mut response) {
                            // 正規表現で一致したなら。
                            best_node_re = node;
                            is_done_re = true;
                        }

                    } else {
                        matched = node.starts_with(&request);
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
                    best_node.forward(&request, &mut response);
                    request.caret = response.caret;
                    response.caret = 0;

                } else {
                    response.caret = request.caret;
                    best_node_re.forward_re(&request, &mut response);
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
                (best_node.controller)(&request, &mut response);
                request.caret = response.caret;
                next = response.next;
                response.caret = 0;
                response.next = "";
                //println!("New next: {}", next);

                // 行終了時コントローラーの更新
                if response.is_linebreak_controller_changed() {
                    current_linebreak_controller = response.linebreak_controller;
                }

                if response.done_line {
                    // 行解析の終了。
                    request.caret = request.line_len;
                }

            } else {
                // 何とも一致しなかったら実行します。
                (self.complementary_controller)(&request, &mut response);
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
        (current_linebreak_controller)(&request, &mut response);
        // caret や、next が変更されていても、無視する。
        false
    }
}


