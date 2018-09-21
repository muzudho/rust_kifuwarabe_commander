use std::any::Any; // https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait
use std::collections::HashMap;
use std::clone::Clone;

pub trait RequestAccessor {
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn get_line(&self) -> &Box<String>;
    fn get_line_len(&self) -> usize;
    fn get_caret(&self) -> usize;
    fn get_groups(&self) -> &Box<Vec<String>>;
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
    fn as_any(&self) -> &dyn Any;
    fn set_caret(&mut self, usize);
    fn set_done_line(&mut self, bool);
    fn set_quits(&mut self, bool);
    fn set_next(&mut self, &'static str);
    fn set_linebreak_controller_changed(&mut self, bool);
    fn set_linebreak_controller(&mut self, Controller<T>);
}

/// トークンと、コントローラーのペアです。
///
/// # Members
///
/// * `token` - 全文一致させたい文字列です。
/// * `controller` - コールバック関数です。
/// * `token_regex` - トークンに正規表現を使うなら真です。
/// * `next_link` - 次はどのノードにつながるか。<任意の名前, ノード名>
pub struct Node<T> {
    pub token: &'static str,
    pub controller: Controller<T>,
    pub token_regex: bool,
    #[derive(Clone)]
    pub next_link: HashMap<String, String>,
}
/*
impl<T> Clone for Node<T> {
    fn clone(&self) -> Node<T> {
        self.clone()
    }
}
*/

pub fn empty_controller<T>(_t: &mut T, _request: &Box<RequestAccessor>, _response: &mut Box<dyn ResponseAccessor<T>>) {}

pub fn reset<T>(response: &mut Box<dyn ResponseAccessor<T>>) {
    response.set_caret(0);
    response.set_done_line(false);
    response.set_quits(false);
    response.set_next("");
    response.set_linebreak_controller_changed(false);
    response.set_linebreak_controller(empty_controller);
}
pub fn set_linebreak_controller<T>(response: &mut Box<dyn ResponseAccessor<T>>, controller: Controller<T>) {
    response.set_linebreak_controller_changed(true);
    response.set_linebreak_controller(controller);
}
