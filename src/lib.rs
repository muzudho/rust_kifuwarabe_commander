/// # Rust きふわらべ シェル
/// 行単位です。
use std::io;

/// 「Rust きふわらべ シェル」への応答を入れてください。
///
/// # Members
///
/// * `quits` - アプリケーションを終了するなら真にします。
pub struct Response {
    pub quits: bool
}

/// コマンドライン文字列に対応づく処理内容を書いてください。
///
/// # Arguments
///
/// * `row` - コマンドライン文字列の1行全体です。
/// * `starts` - コマンドライン文字列の次のトークンの先頭位置が入っています。
/// * `len` - コマンドライン文字列の1行全体の文字数です。
/// * `response` - このアプリケーションへ、このあとの対応を指示します。
///
/// # 参考
/// - Rustのコールバック関数について。  
/// [2016-12-10 Idiomatic callbacks in Rust](https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust)
type Callback = fn(row: &String, starts: &mut usize, len: usize, response: &mut Response);

/// トークンと、コールバック関数の組みです。
///
/// # Members
///
/// * `token` - 全文一致させたい文字列です。
/// * `callback` - コールバック関数です。
pub struct TokenMapping {
    pub token: String,
    pub callback: Callback,
}
impl TokenMapping {
    /// [row]コマンドライン文字列の [starts]番目から [token]が全文一致していれば真を返します。
    ///
    /// # Arguments
    ///
    /// * `row` - コマンドライン文字列の1行全体です。
    /// * `starts` - コマンドライン文字列の次のトークンの先頭位置が入っています。
    /// * `len` - コマンドライン文字列の1行全体の文字数です。
    pub fn is_matched(&self, row: &String, starts: &usize, len: usize) -> bool {
        return self.token.len()<=len && &row[*starts..self.token.len()] == self.token
    }

    /// [token]文字列の長さだけ [starts]キャレットを進めます。
    /// [token]文字列の続きに半角スペース「 」が１つあれば、1つ分だけ読み進めます。
    ///
    /// # Arguments
    ///
    /// * `row` - コマンドライン文字列の1行全体です。
    /// * `starts` - コマンドライン文字列の次のトークンの先頭位置が入っています。
    /// * `len` - コマンドライン文字列の1行全体の文字数です。
    /// * `response` - コールバック関数に渡します。
    pub fn move_caret_and_go(&self, row: &String, starts: &mut usize, len: usize, response: &mut Response) {
        *starts += self.token.len();
        // 続きにスペース「 」が１つあれば読み飛ばす
        if 0<(len-*starts) && &row[*starts..(*starts+1)]==" " {
            *starts+=1;
        }            

        (self.callback)(row, starts, len, response);
    }
}

/// 何もしないコールバック関数です。
///
/// # Arguments
///
/// * `row` - コマンドライン文字列の1行全体です。
/// * `starts` - コマンドライン文字列の次のトークンの先頭位置が入っています。
/// * `len` - コマンドライン文字列の1行全体の文字数です。
/// * `response` - このアプリケーションへ、このあとの対応を指示します。
pub fn none_callback(_row: &String, _starts: &mut usize, _len: usize, _response: &mut Response) {

}

/// このアプリケーションです。
///
/// # Arguments
///
/// * `vec_row` - コマンドを複数行 溜めておくバッファーです。
/// * `other_token_mapping` - トークン マッピングに一致しなかったときに呼び出される例外トークン マッピングです。
/// * `token_mapping_array` - 複数件のトークン マッピングです。
pub struct Shell{
    pub vec_row : Vec<String>,
    pub other_token_mapping: TokenMapping,
    pub token_mapping_array: Vec<TokenMapping>,
}
impl Shell {
    pub fn new()->Shell{
        Shell{
            vec_row : Vec::new(),
            other_token_mapping: TokenMapping { token: "".to_string(), callback: none_callback },
            token_mapping_array: Vec::new(),
        }
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
            let mut row : String;
            if self.is_empty() {
                row = String::new();
            } else {
                // バッファーの先頭行です。
                row = self.pop_row();
            }

            // コマンド プロンプトからの入力があるまで待機します。
            io::stdin().read_line(&mut row)
                .ok() // read_line が返す Resultオブジェクト の okメソッド。
                .expect("info Failed to read_line"); // OKでなかった場合のエラーメッセージ。

            // 末尾の 改行 を除きます。前後の空白も消えます。
            row = row.trim().parse().ok().expect("info Failed to parse");

            // 1行の文字数です。
            let len = row.chars().count();
            let mut starts = 0;

            let mut is_done = false;
            let mut response = Response {
                quits: false
            };

            for element in self.token_mapping_array.iter() {
                if element.is_matched(&row, &starts, len) {
                    element.move_caret_and_go(&row, &mut starts, len, &mut response);
                    is_done = true;
                    break;
                }
            }

            // 何とも一致しなかったら実行します。
            if !is_done {
                self.other_token_mapping.move_caret_and_go(&row, &mut starts, len, &mut response);
            }

            if response.quits {
                // ループを抜けて、アプリケーションを終了します。
                break;
            }
        } // loop
    }
}


