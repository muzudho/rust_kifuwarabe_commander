/// アプリケーション１つにつき、１つのグラフを持ちます。
use node::*;
use std::collections::HashMap;

pub struct Graph<T, S: ::std::hash::BuildHasher> {
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
pub fn insert_node_single<T, S: ::std::hash::BuildHasher> (
    graph: &mut Graph<T, S>,
    name: &'static str,
    controller2: Controller<T>
) where S: ::std::default::Default {
    let next_link2 : HashMap<&'static str, &'static str, S> = [].iter().cloned().collect();
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
pub fn set_complementary_controller<T, S: ::std::hash::BuildHasher>(graph: &mut Graph<T, S>, controller2: Controller<T>) {
    graph.complementary_controller = controller2;
}
