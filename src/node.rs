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
}
pub trait RequestAccessor {
    // fn new(line: Box<String>) -> Request;
    fn get_line(&self) -> &Box<String>;
    fn set_line(&mut self, s:Box<String>);
    fn get_line_len(&self) -> usize;
    fn set_line_len(&mut self, len:usize);
    fn get_caret(&self) -> usize;
    fn set_caret(&mut self, caret:usize);
}

/// コールバック関数です。トークンを読み取った時に対応づく作業内容を書いてください。
///
/// # Arguments
///
/// * `t` - 任意のオブジェクト。
/// * `request` - 入力されたコマンドライン文字列など。
/// * `response` - 読取位置や、次のトークンの指定など。
///
/// # 参考
/// - Rustのコールバック関数について。  
/// [2016-12-10 Idiomatic callbacks in Rust](https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust)
pub type Controller<T> = fn(t: &mut T, request: &Box<RequestAccessor>, response: &mut Response<T>);

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
pub struct Response<T> {
    pub caret: usize,
    pub done_line: bool,
    pub quits: bool,
    pub groups: Vec<String>,
    pub next: &'static str,
    pub linebreak_controller_changed: bool,
    pub linebreak_controller: Controller<T>,
}

/// トークンと、コントローラーのペアです。
///
/// # Members
///
/// * `token` - 全文一致させたい文字列です。
/// * `controller` - コールバック関数です。
/// * `token_regex` - トークンに正規表現を使うなら真です。
pub struct Node<T> {
    pub token: &'static str,
    pub controller: Controller<T>,
    pub token_regex: bool,
}

pub fn empty_controller<T>(_t: &mut T, _request: &Box<RequestAccessor>, _response: &mut Response<T>) {}

pub fn new_response<T>() -> Response<T> {
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
pub fn reset<T>(response: &mut Response<T>) {
    response.caret = 0;
    response.done_line = false;
    response.quits = false;
    response.groups.clear();
    response.next = "";
    response.linebreak_controller_changed = false;
    response.linebreak_controller = empty_controller;
}
pub fn set_linebreak_controller<T>(response: &mut Response<T>, controller: Controller<T>) {
    response.linebreak_controller_changed = true;
    response.linebreak_controller = controller;
}
pub fn is_linebreak_controller_changed<T>(response: &Response<T>) -> bool {
    response.linebreak_controller_changed
}
