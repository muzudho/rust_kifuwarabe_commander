/// アプリケーション１つにつき、１つのグラフを持ちます。

use std::any::Any; // https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait
use std::collections::HashMap;

pub trait RequestAccessor {
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn get_line(&self) -> &String; // &Box<String>
    fn get_line_len(&self) -> usize;
    fn get_caret(&self) -> usize;
    fn get_groups(&self) -> &Vec<String>; // &Box<Vec<String>>
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
pub type Controller<T> =
    fn(t: &mut T, request: &RequestAccessor, response: &mut dyn ResponseAccessor);

pub trait ResponseAccessor {
    fn as_any(&self) -> &dyn Any; // トレイトを実装している方を返すのに使う。
    fn as_mut_any(&mut self) -> &mut dyn Any; // トレイトを実装している方を返すのに使う。
    fn set_caret(&mut self, usize);
    fn set_done_line(&mut self, bool);
    fn set_quits(&mut self, bool);
    fn forward(&mut self, String);
}

/// トークンと、コントローラーのペアです。
///
/// # Members
///
/// * `token` - 全文一致させたい文字列です。
/// * `controller` - コールバック関数の登録名です。
/// * `token_regex` - トークンに正規表現を使うなら真です。
/// * `next_link` - 次はどのノードにつながるか。<任意の名前, ノード名>
pub struct Node<S: ::std::hash::BuildHasher> {
    pub token: String,
    pub controller_name: String,
    pub token_regex: bool,
    // 特殊な任意の名前 '#linebreak'
    next_link: HashMap<String, String, S>,
}
impl<S: ::std::hash::BuildHasher> Node<S> {
    pub fn get_next_link(&self, name:String) -> &String {
        if self.contains_next_link(&name) {
            &self.next_link[&name]
        } else {
            panic!("\"{}\" next link is not found. Please use contains_next_link().", name);
        }
    }
    pub fn contains_next_link(&self, name: &String) -> bool {
        self.next_link.contains_key(name)
    }
}

pub fn empty_controller<T>(
    _t: &mut T,
    _request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
}

/// # Parameters.
///
/// * `node_table` - 複数件のトークンです。
/// * `entrance` - カンマ区切りの登録ノード名です。
#[derive(Default)]
pub struct Graph<T, S: ::std::hash::BuildHasher> {
    /// 特殊なノード名
    /// '#ND_complementary' 一致するトークンが無かったときに呼び出されるコールバック関数です。
    node_table: HashMap<String, Node<S>>,
    pub entrance: String, // pub entrance: &'static str,
    /// 任意の名前と、コントローラー。
    controller_table: HashMap<String, Controller<T>>,
}
impl<T, S: ::std::hash::BuildHasher> Graph<T, S> {
    /// アプリケーション１つにつき、１つのフローチャートを共有します。
    pub fn new() -> Graph<T, S> {
        Graph {
            node_table: HashMap::new(),
            entrance: "".to_string(),
            controller_table: HashMap::new(),
        }
    }
    pub fn set_entrance(&mut self, entrance2: String) { // pub fn set_entrance(&mut self, entrance2: &'static str) {
        self.entrance = entrance2;
    }
    pub fn get_node(&self, name: &String) -> &Node<S> { // pub fn get_node(&self, name: &str) -> &Node<S> {
        if self.contains_node(name) {
            &self.node_table[name]
        } else {
            panic!("{} node is not found.", name);
        }
    }
    pub fn contains_node(&self, name: &String) -> bool { // pub fn contains_node(&self, name: &str) -> bool {
        self.node_table.contains_key(name)
    }
    pub fn get_controller(&self, name: &String) -> &Controller<T> { // pub fn get_controller(&self, name: &str) -> &Controller<T> {
        if self.contains_controller(name) {
            &self.controller_table[name]
        } else {
            panic!("{} controller is not found.", name);
        }
    }
    pub fn contains_controller(&self, name: &String) -> bool { // pub fn contains_controller(&self, name: &str) -> bool {
        self.controller_table.contains_key(name)
    }
    /// name は ハードコーディングするので、 &'static str にする。
    pub fn insert_controller(
        &mut self,
        name: &'static str,
        controller2: Controller<T>,
    ){
        self.controller_table.insert(
            name.to_string(),
            controller2
        );
    }
    /// # Arguments
    ///
    /// * `name` - 登録用の名前です。
    /// * `node` - ノードです。
    /// * `next_link2` - 次はどのノードにつながるか。<任意の名前, ノード名>
    pub fn insert_node(
        &mut self,
        name: String, // name: &'static str
        token2: String, // token2: &'static str,
        controller_name2: String, // controller_name2: &'static str,
        next_link2: HashMap<String, String, S>, // next_link2: HashMap<&'static str, &'static str, S>,
    ) {
        self.node_table.insert(
            name, // name.to_string(),
            Node {
                token: token2,
                controller_name: controller_name2,
                token_regex: false,
                next_link: next_link2,
            },
        );
    }
    /// 正規表現を使うなら。
    ///
    /// # Arguments
    ///
    /// * `name` - 登録用の名前です。
    /// * `node` - ノードです。
    /// * `next_link2` - 次はどのノードにつながるか。<任意の名前, ノード名>
    pub fn insert_node_reg(
        &mut self,
        name: String, // name: &'static str,
        token2: String, // token2: &'static str,
        controller_name2: String, // controller_name2: &'static str,
        next_link2: HashMap<String, String, S>, // next_link2: HashMap<&'static str, &'static str, S>,
    ) {
        self.node_table.insert(
            name.to_string(),
            Node {
                token: token2,
                controller_name: controller_name2,
                token_regex: true,
                next_link: next_link2,
            },
        );
    }
    /// パーサーしないノード。任意の名前とコントローラーのマッピング。
    ///
    /// # Arguments
    ///
    /// * `name` - 登録用の名前です。
    pub fn insert_node_single(&mut self, name: String, controller_name2: String) // pub fn insert_node_single(&mut self, name: &'static str, controller_name2: &'static str)
    where
        S: ::std::default::Default,
    {
        let next_link2: HashMap<String, String, S> = [].iter().cloned().collect(); // HashMap<&'static str, &'static str, S>
        self.node_table.insert(
            name.to_string(),
            Node {
                token: "".to_string(),
                controller_name: controller_name2,
                token_regex: false,
                next_link: next_link2,
            },
        );
    }
}
