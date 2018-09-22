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
//     fn(t: &mut T, request: &Box<RequestAccessor>, response: &mut Box<ResponseAccessor>);

pub trait ResponseAccessor {
    fn as_any(&self) -> &dyn Any; // トレイトを実装している方を返すのに使う。
    fn as_mut_any(&mut self) -> &mut dyn Any; // トレイトを実装している方を返すのに使う。
    fn set_caret(&mut self, usize);
    fn set_done_line(&mut self, bool);
    fn set_quits(&mut self, bool);
    fn forward(&mut self, &'static str);
}

/// トークンと、コントローラーのペアです。
///
/// # Members
///
/// * `token` - 全文一致させたい文字列です。
/// * `controller` - コールバック関数です。
/// * `token_regex` - トークンに正規表現を使うなら真です。
/// * `next_link` - 次はどのノードにつながるか。<任意の名前, ノード名>
pub struct Node<T, S: ::std::hash::BuildHasher> {
    pub token: &'static str,
    pub controller: Controller<T>,
    pub token_regex: bool,
    // 特殊な任意の名前 '#linebreak'
    next_link: HashMap<&'static str, &'static str, S>,
}
impl<T, S: ::std::hash::BuildHasher> Node<T, S> {
    pub fn get_next(&self, name:&'static str) -> &'static str {
        if self.next_link.contains_key(name) {
            self.next_link[name]
        } else {
            panic!("{} next link is not found.", name);
        }
    }
    pub fn contains_next_link(&self, name:&'static str) -> bool {
        self.next_link.contains_key(name)
    }
}

pub fn empty_controller<T>(
    _t: &mut T,
    _request: &RequestAccessor,           // &Box<RequestAccessor>
    _response: &mut dyn ResponseAccessor, // &mut Box<dyn ResponseAccessor>
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
    node_table: HashMap<String, Node<T, S>>,
    pub entrance: &'static str,
}
impl<T, S: ::std::hash::BuildHasher> Graph<T, S> {
    /// アプリケーション１つにつき、１つのフローチャートを共有します。
    pub fn new() -> Graph<T, S> {
        Graph {
            node_table: HashMap::new(),
            entrance: "",
        }
    }
    pub fn set_entrance(&mut self, entrance2: &'static str) {
        self.entrance = entrance2;
    }
    pub fn get_node(&self, name: &str) -> &Node<T, S> {
        if self.contains_node(name) {
            &self.node_table[name]
        } else {
            panic!("{} node is not found.", name);
        }
    }
    pub fn contains_node(&self, name: &str) -> bool {
        self.node_table.contains_key(name)
    }
    /// # Arguments
    ///
    /// * `name` - 登録用の名前です。
    /// * `node` - ノードです。
    /// * `next_link2` - 次はどのノードにつながるか。<任意の名前, ノード名>
    pub fn insert_node(
        &mut self,
        name: &'static str,
        token2: &'static str,
        controller2: Controller<T>,
        next_link2: HashMap<&'static str, &'static str, S>,
    ) {
        self.node_table.insert(
            name.to_string(),
            Node {
                token: token2,
                controller: controller2,
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
        name: &'static str,
        token2: &'static str,
        controller2: Controller<T>,
        next_link2: HashMap<&'static str, &'static str, S>,
    ) {
        self.node_table.insert(
            name.to_string(),
            Node {
                token: token2,
                controller: controller2,
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
    pub fn insert_node_single(&mut self, name: &'static str, controller2: Controller<T>)
    where
        S: ::std::default::Default,
    {
        let next_link2: HashMap<&'static str, &'static str, S> = [].iter().cloned().collect();
        self.node_table.insert(
            name.to_string(),
            Node {
                token: "",
                controller: controller2,
                token_regex: false,
                next_link: next_link2,
            },
        );
    }
}
