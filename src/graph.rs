/// アプリケーション１つにつき、１つのグラフを持ちます。
use node::*;
use std::collections::HashMap;

pub struct Graph<T> {
    pub node_table: HashMap<String, Node<T>>,
    pub complementary_controller: Controller<T>,
}

pub fn contains_node<T>(graph: &Graph<T>, name: &str) -> bool {
    graph.node_table.contains_key(name)
}

/// アプリケーション１つにつき、１つのフローチャートを共有します。
pub fn new_graph<T>() -> Graph<T> {
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
pub fn insert_node<T>(
    graph: &mut Graph<T>,
    name: &'static str,
    token2: &'static str,
    controller2: Controller<T>,
    next_link2: HashMap<&'static str, &'static str>,
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
pub fn insert_node_re<T>(
    graph: &mut Graph<T>,
    name: &'static str,
    token2: &'static str,
    controller2: Controller<T>,
    next_link2: HashMap<&'static str, &'static str>,
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

/// # Arguments
///
/// * `map` - 一致するトークンが無かったときに呼び出されるコールバック関数です。
pub fn set_complementary_controller<T>(graph: &mut Graph<T>, controller2: Controller<T>) {
    graph.complementary_controller = controller2;
}
