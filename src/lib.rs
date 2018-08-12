/// # Rust きふわらべ シェル
/// 行単位です。
extern crate regex;

/// 不具合を取りたいときに真にする。
const VERBOSE : bool = false;

use regex::Regex;
use std::collections::HashMap;
use std::io;

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
pub struct Caret {
    pub starts: usize,
    pub done_line: bool,
    pub quits: bool,
    pub groups: Vec<String>,
    pub next: &'static str,
    line_end_controller_changed: bool,
    line_end_controller: Controller,
}
impl Caret {
    pub fn new() -> Caret {
        Caret {
            starts: 0,
            done_line: false,
            quits: false,
            groups: Vec::new(),
            next: "",
            line_end_controller_changed: false,
            line_end_controller: empty_controller,
        }
    }
    pub fn set_line_end_controller(&mut self, controller: Controller) {
        self.line_end_controller_changed = true;
        self.line_end_controller = controller;
    }
    pub fn reset_line_end_controller(&mut self) {
        self.line_end_controller_changed = false;
        self.line_end_controller = empty_controller;
    }
    pub fn is_line_end_controller_changed(&self) -> bool {
        self.line_end_controller_changed
    }
}

/// コマンドライン文字列。
/// 
/// # Members
/// 
/// * `contents` - コマンドライン文字列の1行全体です。
/// * `len` - コマンドライン文字列の1行全体の文字数です。
pub struct Commandline {
    pub contents: String,
    pub len: usize,
}
impl Commandline {
    pub fn new(contents: String) -> Commandline {
        let len = contents.chars().count();
        Commandline {
            contents: contents,
            len: len,
        }
    }
}

/// コールバック関数です。トークンを読み取った時に対応づく作業内容を書いてください。
///
/// # Arguments
///
/// * `line` - 入力されたコマンドライン文字列など。
/// * `caret` - 読取位置や、次のトークンの指定など。
///
/// # 参考
/// - Rustのコールバック関数について。  
/// [2016-12-10 Idiomatic callbacks in Rust](https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust)
type Controller = fn(line: &Commandline, caret: &mut Caret);

pub fn empty_controller(_line: &Commandline, _caret: &mut Caret) {

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
    /// * `line` - 読み取るコマンドライン。
    /// * `caret` - 読取位置。
    /// * returns - 一致したら真。
    pub fn starts_with(&self, line: &Commandline, caret: &mut Caret) -> bool {
        let caret_end = caret.starts + self.token.len();
        //println!("caret.starts={} + self.token.len()={} <= line.len={} [{}]==[{}]", caret.starts, self.token.len(), line.len,
        //    &line.contents[caret.starts..caret_end], self.token);
        if caret_end <= line.len
            && &line.contents[caret.starts..caret_end] == self.token {
            true
        } else {
            false
        }
    }

    /// 正規表現を使う。
    ///
    /// # Arguments
    ///
    /// * `line` - 読み取るコマンドライン。
    /// * `caret` - 読取位置。
    /// * returns - 一致したら真。
    pub fn starts_with_re(&self, line: &Commandline, caret: &mut Caret) -> bool {

        if VERBOSE {
            println!("starts_with_re");
        }

        if caret.starts < line.len {

            if VERBOSE {
                println!("self.token: {}", self.token);
            }

            let re = Regex::new(self.token).unwrap();

            let text = &line.contents[caret.starts..];

            if VERBOSE {
                println!("text: [{}]", text);
            }

            let mut group_num = 0;
            caret.groups.clear();
            for caps in re.captures_iter(text) {
                // caps は サイズ 2 の配列 で同じものが入っている。
                let cap = &caps[0];

                caret.groups.push(cap.to_string());

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

    pub fn forward(&self, line: &Commandline, caret: &mut Caret) {
        caret.starts += self.token.len();
        // 続きにスペース「 」が１つあれば読み飛ばす
        if 0<(line.len-caret.starts) && &line.contents[caret.starts..(caret.starts+1)]==" " {
            caret.starts += 1;
        }
    }

    /// TODO キャレットを進める。正規表現はどこまで一致したのか分かりにくい。
    pub fn forward_re(&self, line: &Commandline, caret: &mut Caret) {
        let pseud_token_len = caret.groups[0].chars().count();
        caret.starts += pseud_token_len;
        // 続きにスペース「 」が１つあれば読み飛ばす
        if 0<(line.len-caret.starts) && &line.contents[caret.starts..(caret.starts+1)]==" " {
            caret.starts += 1;
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
    vec_row : Vec<String>,
    node_table: HashMap<String, Node>,
    complementary_controller: Controller,
    pub next: &'static str,
}
impl Shell {
    pub fn new()->Shell{
        Shell {
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

        let empty_node = Node {
            token: "",
            controller: empty_controller,
            token_regex: false,
        };

        'lines: loop{

            let mut current_lineend_controller : Controller;
            current_lineend_controller = empty_controller;

            let line : Commandline;
            if self.is_empty() {
                let mut line_string = String::new();
                // コマンド プロンプトからの入力があるまで待機します。
                io::stdin().read_line(&mut line_string)
                    .ok() // read_line が返す Resultオブジェクト の okメソッド。
                    .expect("info Failed to read_line"); // OKでなかった場合のエラーメッセージ。

                // 末尾の 改行 を除きます。前後の空白も消えます。
                line_string = line_string.trim().parse().ok().expect("info Failed to parse");

                line = Commandline::new(line_string);
            } else {
                // バッファーの先頭行です。
                line = Commandline::new(self.pop_row());
            }



            let mut caret = Caret::new();
            let mut next = self.next;

            'line: while caret.starts < line.len {

                // キャレットの位置そのままで次のトークンへ。
                caret.next = "";
                caret.reset_line_end_controller();
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
                let mut best_node = &empty_node;
                let mut best_node_re = &empty_node;
                for i_next_node_name in vec_next {
                    let next_node_name = i_next_node_name.trim();
                    // println!("next_node_name: {}", next_node_name);
                    if self.contains_node(&next_node_name.to_string()) {
                        //println!("contains.");
                        let node = self.get_node(&next_node_name.to_string());

                        let matched;
                        if node.token_regex {
                            if node.starts_with_re(&line, &mut caret) {
                                // 正規表現で一致したなら。
                                best_node_re = node;
                                is_done_re = true;
                            }

                        } else {
                            matched = node.starts_with(&line, &mut caret);
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
                            //    println!("not starts_with. line.contents={}, line.len={}, caret.starts={}", line.contents, line.len, caret.starts);
                            }

                        }

                    }
                }

                // キャレットを進める。
                if is_done || is_done_re {
                    if is_done {
                        best_node.forward(&line, &mut caret);
                        
                    } else {
                        best_node_re.forward_re(&line, &mut caret);

                        // まとめる。
                        is_done = is_done_re;
                        best_node = best_node_re;
                    }
                }

                if is_done {
                    
                    (best_node.controller)(&line, &mut caret);

                    next = caret.next;
                    //println!("New next: {}", next);

                    // 行終了時コントローラーの更新
                    if caret.is_line_end_controller_changed() {
                        current_lineend_controller = caret.line_end_controller;
                    }

                    if caret.done_line {
                        // 行解析の終了。
                        caret.starts = line.len;
                    }

                } else {
                    // 何とも一致しなかったら実行します。
                    (self.complementary_controller)(&line, &mut caret);
                    // 次のラインへ。
                    break 'line;
                }

                if caret.quits {
                    // ループを抜けて、アプリケーションを終了します。
                    break 'lines;
                }
            }

            // 1行読取終了。
            (current_lineend_controller)(&line, &mut caret);


        } // loop
    }
}


