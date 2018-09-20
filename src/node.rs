pub trait RequestAccessor {
    // fn new(line: Box<String>) -> Request;
    fn get_line(&self) -> &Box<String>;
    fn get_line_len(&self) -> usize;
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
pub type Controller<T> = fn(t: &mut T, request: &Box<RequestAccessor>, response: &mut Box<ResponseAccessor<T>>);

pub trait ResponseAccessor<T> {
    fn get_caret(&self) -> usize;
    fn set_caret(&mut self, usize);
    fn is_done_line(&self) -> bool;
    fn set_done_line(&mut self, bool);
    fn is_quits(&self) -> bool;
    fn set_quits(&mut self, bool);
    fn clear_groups(&mut self);
    fn get_groups(&self) -> &Box<Vec<String>>;
    fn push_to_groups(&mut self, String);
    fn set_groups(&mut self, Box<Vec<String>>);
    fn get_next(&self) -> &'static str;
    fn set_next(&mut self, &'static str);
    fn is_linebreak_controller_changed(&self) -> bool;
    fn set_linebreak_controller_changed(&mut self, bool);
    fn get_linebreak_controller(&self) -> Controller<T>;
    fn set_linebreak_controller(&mut self, Controller<T>);
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

pub fn empty_controller<T>(_t: &mut T, _request: &Box<RequestAccessor>, _response: &mut Box<dyn ResponseAccessor<T>>) {}

pub fn reset<T>(response: &mut Box<dyn ResponseAccessor<T>>) {
    response.set_caret(0);
    response.set_done_line(false);
    response.set_quits(false);
    response.clear_groups();
    response.set_next("");
    response.set_linebreak_controller_changed(false);
    response.set_linebreak_controller(empty_controller);
}
pub fn set_linebreak_controller<T>(response: &mut Box<dyn ResponseAccessor<T>>, controller: Controller<T>) {
    response.set_linebreak_controller_changed(true);
    response.set_linebreak_controller(controller);
}
pub fn is_linebreak_controller_changed<T>(response: &Box<dyn ResponseAccessor<T>>) -> bool {
    response.is_linebreak_controller_changed()
}
