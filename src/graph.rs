/// アプリケーション１つにつき、１つのグラフを持ちます。
// 参考:
// https://github.com/serde-rs/json
extern crate serde_json;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

use std::any::Any; // https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait
use std::collections::HashMap;

pub trait RequestAccessor {
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn get_line(&self) -> &String;
    fn get_line_len(&self) -> usize;
    fn get_caret(&self) -> usize;
    fn get_groups(&self) -> &Vec<String>;
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
    fn set_reloads(&mut self, &'static str);
    // .rs にハードコーディングして使う。
    fn forward(&mut self, &'static str);
}

/// トークンと、コントローラーのペアです。
///
/// # Members
///
/// * `token` - 全文一致させたい文字列です。
/// * `fn_label` - コールバック関数の登録名です。
/// * `token_regex` - トークンに正規表現を使うなら真です。
/// * `exit_link` - 次はどのノードにつながるか。<任意の名前, ノード名>
pub struct Node {
    pub token: String,
    pub fn_label: String,
    pub token_regex: bool,
    // 特殊な任意の名前 '#newline'
    exits: HashMap<String, Vec<String>>,
}
impl Node {
    pub fn get_exits(&self, name: &str) -> &Vec<String> {
        if self.contains_exits(&name.to_string()) {
            &self.exits[name]
        } else {
            panic!(
                "\"{}\" exit is not found. Please use contains_exits().",
                name
            );
        }
    }
    pub fn contains_exits(&self, name: &str) -> bool {
        self.exits.contains_key(name)
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
pub struct Graph<T> {
    /// 特殊なノード名
    /// '#else' 一致するトークンが無かったときに呼び出されるコールバック関数です。
    node_table: HashMap<String, Node>,
    entrance: Vec<String>,
    /// 任意の名前と、コントローラー。
    controller_table: HashMap<String, Controller<T>>,
}
impl<T> Graph<T> {
    /// アプリケーション１つにつき、１つのフローチャートを共有します。
    pub fn new() -> Graph<T> {
        Graph {
            node_table: HashMap::new(),
            entrance: Vec::new(),
            controller_table: HashMap::new(),
        }
    }
    /// クリアー。（登録したコントローラーを除く）
    pub fn clear_graph(&mut self) {
        self.node_table.clear();
        self.entrance.clear();
    }
    pub fn get_entrance(&self) -> &Vec<String> {
        &self.entrance
    }
    pub fn set_entrance(&mut self, entrance2: Vec<String>) {
        self.entrance = entrance2;
    }
    pub fn get_node(&self, label: &str) -> &Node {
        if self.contains_node(&label.to_string()) {
            &self.node_table[label]
        } else {
            panic!("\"{}\" node is not found.", label);
        }
    }
    pub fn contains_node(&self, label: &str) -> bool {
        self.node_table.contains_key(&label.to_string())
    }
    pub fn get_controller(&self, name: &str) -> &Controller<T> {
        if self.contains_controller(&name.to_string()) {
            &self.controller_table[&name.to_string()]
        } else {
            panic!(
                "\"{}\" fn is not found. Please use contains_controller().",
                name
            );
        }
    }
    pub fn contains_controller(&self, name: &str) -> bool {
        self.controller_table.contains_key(&name.to_string())
    }
    /// name は ハードコーディングするので、 &'static str にする。
    pub fn insert_controller(&mut self, name: &'static str, controller2: Controller<T>) {
        self.controller_table.insert(name.to_string(), controller2);
    }
    /// # Arguments
    ///
    /// * `label` - 登録用のノード名です。
    /// * `node` - ノードです。
    /// * `exits2` - 次はどのノードにつながるか。<任意の名前, ノード名>
    pub fn insert_node(
        &mut self,
        label: String,
        token2: String,
        fn_label2: String,
        exits2: HashMap<String, Vec<String>>,
    ) {
        self.node_table.insert(
            label,
            Node {
                token: token2,
                fn_label: fn_label2,
                token_regex: false,
                exits: exits2,
            },
        );
    }
    /// 正規表現を使うなら。
    ///
    /// # Arguments
    ///
    /// * `label` - 登録用のノード名です。
    /// * `node` - ノードです。
    /// * `exits2` - 次はどのノードにつながるか。<任意の名前, ノード名>
    pub fn insert_node_reg(
        &mut self,
        label: &str,
        token2: String,
        fn_label2: String,
        exits2: HashMap<String, Vec<String>>,
    ) {
        self.node_table.insert(
            label.to_string(),
            Node {
                token: token2,
                fn_label: fn_label2,
                token_regex: true,
                exits: exits2,
            },
        );
    }
    /// パーサーしないノード。任意の名前とコントローラーのマッピング。
    ///
    /// # Arguments
    ///
    /// * `label` - 登録用のノード名です。
    pub fn insert_node_single(&mut self, label: &str, fn_label2: String) {
        let exits2: HashMap<String, Vec<String>> = [].iter().cloned().collect();
        self.node_table.insert(
            label.to_string(),
            Node {
                token: "".to_string(),
                fn_label: fn_label2,
                token_regex: false,
                exits: exits2,
            },
        );
    }

    /// JSON配列を、文字列の配列に変換。
    ///
    /// # Arguments.
    ///
    /// * 'v' - Json array.
    /// * 'str_vec' - let str_vec = Vec::new();
    fn array_to_str_vec(&self, v: &Value, str_vec: &mut Vec<String>) {
        let value_vec: Vec<Value> = v.as_array().unwrap().to_vec();
        for node_label in value_vec {
            str_vec.push(node_label.as_str().unwrap().to_string());
        }
    }
    /// JSONオブジェクトを、文字列のハッシュマップに変換。
    ///
    /// # Arguments.
    ///
    /// * 'v' - Json object.
    /// * 'str_vec' - let str_vec = Vec::new();
    fn object_to_map(&self, obj: &Value, map0: &mut HashMap<String, Vec<String>>) {
        if !obj.is_null() {
            for (name1, array1) in obj.as_object().unwrap().iter() {
                let mut array2: Vec<String> = Vec::new();
                for item1 in array1.as_array().unwrap().iter() {
                    array2.push(item1.as_str().unwrap().to_string());
                }
                map0.insert(name1.to_string(), array2);
            }
        }
    }
    /// ファイル読み込み
    pub fn read_graph_file(&mut self, file: String) {
        self.clear_graph();

        let mut file = match File::open(file) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err)
        };

        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err)
        };

        // https://docs.serde.rs/serde_json/value/enum.Value.html
        let v: Value = match serde_json::from_str(&data) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err)
        };

        // 文字列に変換する。
        let mut entrance_vec: Vec<String> = Vec::new();
        self.array_to_str_vec(&v["entrance"], &mut entrance_vec);
        self.set_entrance(entrance_vec);

        for node in v["nodes"].as_array().unwrap().iter() {
            if !node["token"].is_null() {
                let mut entrance_map: HashMap<String, Vec<String>> = HashMap::new();
                self.object_to_map(&node["exit"], &mut entrance_map);
                self.insert_node(
                    node["label"].as_str().unwrap().to_string(),
                    node["token"].as_str().unwrap().to_string(),
                    if node["fn"].is_null() {
                        "".to_string()
                    } else {
                        node["fn"].as_str().unwrap().to_string()
                    },
                    entrance_map,
                );
            } else if !node["regex"].is_null() {
                let mut entrance_map: HashMap<String, Vec<String>> = HashMap::new();
                self.object_to_map(&node["exit"], &mut entrance_map);
                self.insert_node_reg(
                    &node["label"].as_str().unwrap().to_string(),
                    node["regex"].as_str().unwrap().to_string(),
                    if node["fn"].is_null() {
                        "".to_string()
                    } else {
                        node["fn"].as_str().unwrap().to_string()
                    },
                    entrance_map,
                );
            } else {
                self.insert_node_single(
                    &node["label"].as_str().unwrap().to_string(),
                    if node["fn"].is_null() {
                        "".to_string()
                    } else {
                        node["fn"].as_str().unwrap().to_string()
                    },
                );
            }
        }
    }
}
