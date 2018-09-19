use std::collections::HashMap;

/// コマンドライン文字列。
///
/// # Members
///
/// * `line` - コマンドライン文字列の1行全体です。
/// * `line_len` - コマンドライン文字列の1行全体の文字数です。
pub struct Request {
    pub line: String,
    pub line_len: usize,
    pub caret: usize,
}
impl Request {
    pub fn new(line2: String) -> Request {
        let len = line2.chars().count();
        Request {
            line: line2,
            line_len: len,
            caret: 0,
        }
    }
}

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
pub type Controller<T> = fn(t: &mut T, request: &Request, response: &mut Response<T>);

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

/// アプリケーション１つにつき、１つのフローチャートを持ちます。
pub struct Flowchart<T> {
    pub node_table: HashMap<String, Node<T>>,
    pub complementary_controller: Controller<T>,
}

pub fn contains_node<T>(flowchart: &Flowchart<T>, name: &str) -> bool {
    flowchart.node_table.contains_key(name)
}

pub fn empty_controller<T>(_t: &mut T, _request: &Request, _response: &mut Response<T>) {}

/// アプリケーション１つにつき、１つのフローチャートを共有します。
pub fn new_flowchart<T>() -> Flowchart<T> {
    Flowchart {
        node_table: HashMap::new(),
        complementary_controller: empty_controller,
    }
}

/*
trait FlowchartTrait {
    fn new<T>() -> Flowchart<T>;
}
impl Flowchart{
    fn new<T>() -> Flowchart<T> {
        Flowchart {
            node_table: HashMap::new(),
        }
    }
}
*/


/// # Arguments
///
/// * `name` - 登録用の名前です。
/// * `node` - ノードです。
pub fn insert_node<T>(
    flowchart: &mut Flowchart<T>,
    name: &'static str,
    token2: &'static str,
    controller2: Controller<T>,
) {
    flowchart.node_table.insert(
        name.to_string(),
        Node {
            token: token2,
            controller: controller2,
            token_regex: false,
        },
    );
}

/// 正規表現を使うなら。
///
/// # Arguments
///
/// * `name` - 登録用の名前です。
/// * `node` - ノードです。
pub fn insert_node_re<T>(
    flowchart: &mut Flowchart<T>,
    name: &'static str,
    token2: &'static str,
    controller2: Controller<T>,
) {
    flowchart.node_table.insert(
        name.to_string(),
        Node {
            token: token2,
            controller: controller2,
            token_regex: true,
        },
    );
}

/// # Arguments
///
/// * `map` - 一致するトークンが無かったときに呼び出されるコールバック関数です。
pub fn set_complementary_controller<T>(flowchart: &mut Flowchart<T>, controller2: Controller<T>) {
    flowchart.complementary_controller = controller2;
}
