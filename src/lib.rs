/// # Rust きふわらべ シェル
/// 行単位です。
use std::collections::HashMap;
use std::io;

/// キャレット。
/// 
/// # Members
/// 
/// * `starts` - コマンドライン文字列の次のトークンの先頭位置が入っています。
/// * `quits` - アプリケーションを終了するなら真にします。
pub struct Caret {
    pub starts: usize,
    pub quits: bool,
}
impl Caret {
    pub fn new() -> Caret {
        Caret {
            starts: 0,
            quits: false,
        }
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

/// コマンドライン文字列に対応づく処理内容を書いてください。
///
/// # Arguments
///
/// * `row` - コマンドライン文字列の1行全体です。
/// * `starts` - コマンドライン文字列の次のトークンの先頭位置が入っています。
/// * `response` - このアプリケーションへ、このあとの対応を指示します。
///
/// # 参考
/// - Rustのコールバック関数について。  
/// [2016-12-10 Idiomatic callbacks in Rust](https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust)
type Callback = fn(line: &Commandline, caret: &mut Caret);

/// 何もしないコールバック関数です。
///
/// # Arguments
///
/// * `row` - コマンドライン文字列の1行全体です。
/// * `starts` - コマンドライン文字列の次のトークンの先頭位置が入っています。
/// * `len` - コマンドライン文字列の1行全体の文字数です。
/// * `response` - このアプリケーションへ、このあとの対応を指示します。
pub fn none_callback(_line: &Commandline, _caret: &mut Caret) {

}

pub struct StaticNode {
    pub token: &'static str,
    pub callback: &'static str,
}

/// トークンと、コールバック関数の組みです。
///
/// # Members
///
/// * `token` - 全文一致させたい文字列です。
/// * `callback` - コールバックの項目名です。
pub struct Node {
    pub token: String,
    pub token_len: usize,
    pub callback: String,
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
    pub fn starts_with_and_forward(&self, line: &Commandline, caret: &mut Caret) -> bool {
        if caret.starts + self.token.len() <= line.len
            && &line.contents[caret.starts..self.token.len()] == self.token {

            caret.starts += self.token.len();
            // 続きにスペース「 」が１つあれば読み飛ばす
            if 0<(line.len-caret.starts) && &line.contents[caret.starts..(caret.starts+1)]==" " {
                caret.starts += 1;
            }            

            true
        } else {
            false
        }
    }
}

/// このアプリケーションです。
///
/// # Arguments
///
/// * `vec_row` - コマンドを複数行 溜めておくバッファーです。
/// * `token_array` - 複数件のトークン マッピングです。
/// * `complementary_callback` - トークン マッピングに一致しなかったときに呼び出されるコールバック関数の名前です。
pub struct Shell{
    vec_row : Vec<String>,
    node_table: HashMap<String, Node>,
    callback_table: HashMap<String, Callback>,
    complementary_callback: String,
}
impl Shell {
    pub fn new()->Shell{
        Shell {
            vec_row : Vec::new(),
            node_table: HashMap::new(),
            callback_table: HashMap::new(),
            complementary_callback: "".to_string(),
        }
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
    /// * `static_node` - ノードです。
    pub fn insert_static_node(&mut self, name: &'static str, static_node: StaticNode){
        let len = static_node.token.chars().count();
        self.node_table.insert(
            name.to_string(),
            Node {
                token: static_node.token.to_string(),
                token_len: len,
                callback: static_node.callback.to_string()
            }
        );
    }

    /// # Arguments
    /// 
    /// * `node_name` - 登録用の名前です。
    /// * `node` - ノードです。
    pub fn insert_node(&mut self, name: String, node: Node){
        self.node_table.insert(name, node);
    }

    pub fn contains_callback(&self, name: &String) -> bool {
        self.callback_table.contains_key(name)
    }

    pub fn get_callback(&self, name: &String) -> &Callback {
        self.callback_table.get(name).unwrap()
    }

    /// # Arguments
    /// 
    /// * `callback_name` - 登録用の名前です。
    /// * `callback` - コールバック関数です。
    pub fn insert_callback(&mut self, name: String, callback: Callback){
        self.callback_table.insert(name, callback);
    }

    /// # Arguments
    /// 
    /// * `map` - 一致するトークンが無かったときに呼び出されるコールバック関数です。
    pub fn set_complementary_callback(&mut self, name: String){
        self.complementary_callback = name;
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
        loop{
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
            let mut is_done = false;

            // 最初は全てのノードが対象。
            let mut max_len = 0;
            let mut best_callback = "".to_string();
            for (_name, node) in &self.node_table {
                if node.starts_with_and_forward(&line, &mut caret) {
                    if max_len < node.token_len {
                        max_len = node.token_len;
                        best_callback = node.callback.to_string();
                    };

                    is_done = true;
                }
            }

            {
                if self.contains_callback(&best_callback) {
                    let callback = self.get_callback(&best_callback);
                    (callback)(&line, &mut caret);
                }
            }

            // 何とも一致しなかったら実行します。
            if !is_done {
                if self.contains_callback(&self.complementary_callback) {
                    let callback = self.get_callback(&self.complementary_callback);
                    (callback)(&line, &mut caret);
                }
            }

            if caret.quits {
                // ループを抜けて、アプリケーションを終了します。
                break;
            }
        } // loop
    }
}


