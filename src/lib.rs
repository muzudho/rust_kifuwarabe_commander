/// # Rust きふわらべ シェル
/// 行単位です。
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
pub struct CommandlineString {
    pub contents: String,
    pub len: usize,
}
impl CommandlineString {
    pub fn new(contents: String) -> CommandlineString {
        let len = contents.chars().count();
        CommandlineString {
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
type Callback = fn(row: &String, caret: &mut Caret);

/// 何もしないコールバック関数です。
///
/// # Arguments
///
/// * `row` - コマンドライン文字列の1行全体です。
/// * `starts` - コマンドライン文字列の次のトークンの先頭位置が入っています。
/// * `len` - コマンドライン文字列の1行全体の文字数です。
/// * `response` - このアプリケーションへ、このあとの対応を指示します。
pub fn none_callback(_row: &String, _caret: &mut Caret) {

}

/// トークンと、コールバック関数の組みです。
///
/// # Members
///
/// * `token` - 全文一致させたい文字列です。
/// * `callback` - コールバック関数です。
pub struct ExpectedToken {
    pub token: String,
    pub callback: Callback,
}
impl ExpectedToken {
    /// [token]文字列の長さだけ [starts]キャレットを進めます。
    /// [token]文字列の続きに半角スペース「 」が１つあれば、1つ分だけ読み進めます。
    ///
    /// # Arguments
    ///
    /// * `line_str` - 読み取るコマンドライン。
    /// * `caret` - 読取位置。
    /// * returns - 一致したら真。
    pub fn start_with_and_forward(&self, line_str: &CommandlineString, caret: &mut Caret) -> bool {
        if caret.starts + self.token.len() <= line_str.len
            && &line_str.contents[caret.starts..self.token.len()] == self.token {

            caret.starts += self.token.len();
            // 続きにスペース「 」が１つあれば読み飛ばす
            if 0<(line_str.len-caret.starts) && &line_str.contents[caret.starts..(caret.starts+1)]==" " {
                caret.starts += 1;
            }            

            (self.callback)(&line_str.contents, caret);

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
/// * `expected_token_array` - 複数件のトークン マッピングです。
/// * `other_callback` - トークン マッピングに一致しなかったときに呼び出されるコールバック関数です。
pub struct Shell{
    vec_row : Vec<String>,
    expected_token_array: Vec<ExpectedToken>,
    other_callback: Callback,
}
impl Shell {
    pub fn new()->Shell{
        Shell{
            vec_row : Vec::new(),
            expected_token_array: Vec::new(),
            other_callback: none_callback,
        }
    }

    /// # Arguments
    /// 
    /// * `map` - トークンと、コールバック関数の組みです。
    pub fn push_token_mapping(&mut self, map: ExpectedToken){
        self.expected_token_array.push(map);
    }

    /// # Arguments
    /// 
    /// * `map` - 一致するトークンが無かったときに呼び出されるコールバック関数です。
    pub fn set_other_callback(&mut self, callback: Callback){
        self.other_callback = callback;
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
            let line_str : CommandlineString;
            if self.is_empty() {
                let mut line = String::new();
                // コマンド プロンプトからの入力があるまで待機します。
                io::stdin().read_line(&mut line)
                    .ok() // read_line が返す Resultオブジェクト の okメソッド。
                    .expect("info Failed to read_line"); // OKでなかった場合のエラーメッセージ。

                // 末尾の 改行 を除きます。前後の空白も消えます。
                line = line.trim().parse().ok().expect("info Failed to parse");

                line_str = CommandlineString::new(line);
            } else {
                // バッファーの先頭行です。
                line_str = CommandlineString::new(self.pop_row());
            }



            let mut caret = Caret::new();
            let mut is_done = false;

            for element in self.expected_token_array.iter() {
                if element.start_with_and_forward(&line_str, &mut caret) {
                    is_done = true;
                    break;
                }
            }

            // 何とも一致しなかったら実行します。
            if !is_done {
                (self.other_callback)(&line_str.contents, &mut caret);
            }

            if caret.quits {
                // ループを抜けて、アプリケーションを終了します。
                break;
            }
        } // loop
    }
}


