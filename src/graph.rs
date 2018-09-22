/// アプリケーション１つにつき、１つのグラフを持ちます。
use std::any::Any; // https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait
use std::collections::HashMap;

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
pub type Controller<T> =
    fn(t: &mut T, request: &Box<RequestAccessor>, response: &mut Box<ResponseAccessor>);

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
    pub next_link: HashMap<&'static str, &'static str, S>,
}

pub fn empty_controller<T>(
    _t: &mut T,
    _request: &Box<RequestAccessor>,
    _response: &mut Box<dyn ResponseAccessor>,
) {
}

pub struct Graph<T, S: ::std::hash::BuildHasher> {
    /// 特殊なノード名
    /// '#linebreak', '#complementary'
    pub node_table: HashMap<String, Node<T, S>>,
    pub complementary_controller: Controller<T>,
}

pub fn contains_node<T, S: ::std::hash::BuildHasher>(graph: &Graph<T, S>, name: &str) -> bool {
    graph.node_table.contains_key(name)
}

/// アプリケーション１つにつき、１つのフローチャートを共有します。
pub fn new_graph<T, S: ::std::hash::BuildHasher>() -> Graph<T, S> {
    Graph {
        node_table: HashMap::new(),
        complementary_controller: empty_controller,
    }
}

/// # Arguments
///
/// * `name` - 登録用の名前です。
/// * `node` - ノードです。
/// * `next_link2` - 次はどのノードにつながるか。<任意の名前, ノード名>
pub fn insert_node<T, S: ::std::hash::BuildHasher>(
    graph: &mut Graph<T, S>,
    name: &'static str,
    token2: &'static str,
    controller2: Controller<T>,
    next_link2: HashMap<&'static str, &'static str, S>,
) {
    graph.node_table.insert(
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
pub fn insert_node_re<T, S: ::std::hash::BuildHasher>(
    graph: &mut Graph<T, S>,
    name: &'static str,
    token2: &'static str,
    controller2: Controller<T>,
    next_link2: HashMap<&'static str, &'static str, S>,
) {
    graph.node_table.insert(
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
pub fn insert_node_single<T, S: ::std::hash::BuildHasher>(
    graph: &mut Graph<T, S>,
    name: &'static str,
    controller2: Controller<T>,
) where
    S: ::std::default::Default,
{
    let next_link2: HashMap<&'static str, &'static str, S> = [].iter().cloned().collect();
    graph.node_table.insert(
        name.to_string(),
        Node {
            token: "",
            controller: controller2,
            token_regex: false,
            next_link: next_link2,
        },
    );
}

/// # Arguments
///
/// * `map` - 一致するトークンが無かったときに呼び出されるコールバック関数です。
pub fn set_complementary_controller<T, S: ::std::hash::BuildHasher>(
    graph: &mut Graph<T, S>,
    controller2: Controller<T>,
) {
    graph.complementary_controller = controller2;
}
