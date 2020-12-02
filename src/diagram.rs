/// アプリケーション１つにつき、１つのグラフを持ちます。
// 参考:
// https://github.com/serde-rs/json
extern crate serde_json;
use serde_json::Value;

use std::fs::File;
use std::io::Read;
use std::io::Write;

use models::diagram_json::*;
use std::any::Any; // https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait
use std::collections::HashMap;
use std::fs::OpenOptions;

pub trait Request {
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
/// * `req` - 入力されたコマンドライン文字列など。
/// * `res` - 読取位置や、次のトークンの指定など。
///
/// # 参考
/// - Rustのコールバック関数について。  
/// [2016-12-10 Idiomatic callbacks in Rust](https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust)
pub type Controller<T> = fn(t: &mut T, req: &Request, res: &mut dyn Response);

/// シェルに指示を出す。
pub enum ResponseOption {
    None,
    Quits,
    Reloads(String),
    Saves(String),
}

pub trait Response {
    fn as_any(&self) -> &dyn Any; // トレイトを実装している方を返すのに使う。
    fn as_mut_any(&mut self) -> &mut dyn Any; // トレイトを実装している方を返すのに使う。
    fn set_caret(&mut self, usize);
    fn set_done_line(&mut self, bool);
    fn set_option(&mut self, ResponseOption);
    // .rs にハードコーディングして使う。
    fn forward(&mut self, &'static str);
}

/// トークンと、コントローラーのペアです。
///
/// # Members
///
/// * `token` - 全文一致させたい文字列です。
/// * `fn_label` - コールバック関数の登録名です。
/// * `regex_flag` - トークンに正規表現を使うなら真です。
/// * `exit_link` - 次はどのノードにつながるか。<任意の名前, ノード名>
pub struct Node {
    // 行き先が複数パターンある。
    exit_map: HashMap<String, Vec<String>>,

    label: String,
    token: String,
    fn_label: String,
    regex_flag: bool,
}
impl Node {
    pub fn get_label(&self) -> &str {
        &self.label
    }
    pub fn get_token(&self) -> &str {
        &self.token
    }
    pub fn get_fn_label(&self) -> &str {
        &self.fn_label
    }
    pub fn is_regex(&self) -> bool {
        self.regex_flag
    }
    /// 確認用。
    pub fn get_exit_map(&self) -> &HashMap<String, Vec<String>> {
        &self.exit_map
    }
    pub fn get_exit_vec(&self, door_label: &str) -> &Vec<String> {
        if self.contains_exit(&door_label.to_string()) {
            &self.exit_map[door_label]
        } else {
            panic!("\"{}\" door is not found. ({} node)", door_label, self.label);
        }
    }
    pub fn contains_exit(&self, name: &str) -> bool {
        self.exit_map.contains_key(name)
    }
}

pub fn empty_controller<T>(_t: &mut T, _req: &Request, _res: &mut dyn Response) {}

/// # Parameters.
///
/// * `fn_map` - 任意の名前と、コントローラー。遷移先を振り分けるルーチン。
/// * `node_map` - 複数件のトークンです。
#[derive(Default)]
pub struct Diagram<T> {
    entry_point: String,
    node_map: HashMap<String, Node>,

    fn_map: HashMap<String, Controller<T>>,
}
impl<T> Diagram<T> {
    /// アプリケーション１つにつき、１つのフローチャートを共有します。
    pub fn new() -> Diagram<T> {
        Diagram {
            node_map: HashMap::new(),
            entry_point: "".to_string(),

            fn_map: HashMap::new(),
        }
    }
    /// 確認用。
    pub fn get_node_map(&self) -> &HashMap<String, Node> {
        &self.node_map
    }
    /// クリアー。（登録したコントローラーを除く）
    pub fn clear(&mut self) {
        self.entry_point = "".to_string();
        self.node_map.clear();
    }
    pub fn get_entry_point(&self) -> String {
        self.entry_point.to_string()
    }
    pub fn set_entry_point(&mut self, value: String) {
        self.entry_point = value;
    }
    pub fn get_node(&self, node_label: &str) -> &Node {
        if self.contains_node(&node_label.to_string()) {
            &self.node_map[node_label]
        } else {
            panic!("\"{}\" node is not found.", node_label);
        }
    }
    pub fn contains_node(&self, node_label: &str) -> bool {
        self.node_map.contains_key(&node_label.to_string())
    }

    pub fn get_fn(&self, name: &str) -> &Controller<T> {
        match self.fn_map.get(&name.to_string()) {
            Some(f) => &f,
            None => panic!("\"{}\" fn is not found. Please use contains_fn().", name),
        }
    }
    pub fn contains_fn(&self, name: &str) -> bool {
        self.fn_map.contains_key(&name.to_string())
    }
    /// name は ハードコーディングするので、 &'static str にする。
    pub fn insert_fn(&mut self, name: &'static str, fn2: Controller<T>) {
        self.fn_map.insert(name.to_string(), fn2);
    }
    /// # Arguments
    ///
    /// * `label2` - 登録用のノード名です。
    /// * `exit_map2` - 次はどのノードにつながるか。<任意の名前, ノード名>
    pub fn insert_node(
        &mut self,
        label2: &str,
        token2: String,
        fn_label2: String,
        exit_map2: HashMap<String, Vec<String>>,
    ) {
        self.node_map.insert(
            label2.to_string(),
            Node {
                label: label2.to_string(),
                token: token2,
                fn_label: fn_label2,
                regex_flag: false,
                exit_map: exit_map2,
            },
        );
    }
    /// 正規表現を使うなら。
    ///
    /// # Arguments
    ///
    /// * `label` - 登録用のノード名です。
    /// * `node` - ノードです。
    /// * `exit_map2` - 次はどのノードにつながるか。<任意の名前, ノード名>
    pub fn insert_node_reg(
        &mut self,
        label: &str,
        token2: String,
        fn_label2: String,
        exit_map2: HashMap<String, Vec<String>>,
    ) {
        self.node_map.insert(
            label.to_string(),
            Node {
                label: label.to_string(),
                token: token2,
                fn_label: fn_label2,
                regex_flag: true,
                exit_map: exit_map2,
            },
        );
    }
    /// パーサーしないノード。任意の名前とコントローラーのマッピング。
    ///
    /// # Arguments
    ///
    /// * `label` - 登録用のノード名です。
    pub fn insert_node_single(
        &mut self,
        label: &str,
        fn_label2: String,
        exit_map2: HashMap<String, Vec<String>>,
    ) {
        // let exit_map2: HashMap<String, Vec<String>> = [].iter().cloned().collect();
        self.node_map.insert(
            label.to_string(),
            Node {
                label: label.to_string(),
                token: "".to_string(),
                fn_label: fn_label2,
                regex_flag: false,
                exit_map: exit_map2,
            },
        );
    }

    /*
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
    */
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
    pub fn read_file(&mut self, file: &str) {
        self.clear();

        let mut file = match File::open(file) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        // https://docs.serde.rs/serde_json/value/enum.Value.html
        let v: Value = match serde_json::from_str(&data) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        // エントリー・ポイント取得。
        self.entry_point = v["entry_point"].as_str().unwrap().to_string();

        for node in v["nodes"].as_array().unwrap().iter() {
            let mut exit_map: HashMap<String, Vec<String>> = HashMap::new();
            self.object_to_map(&node["exit"], &mut exit_map);
            if !node["token"].is_null() {
                self.insert_node(
                    &node["label"].as_str().unwrap().to_string(),
                    node["token"].as_str().unwrap().to_string(),
                    if node["fn"].is_null() {
                        "".to_string()
                    } else {
                        node["fn"].as_str().unwrap().to_string()
                    },
                    exit_map,
                );
            } else if !node["regex"].is_null() {
                self.insert_node_reg(
                    &node["label"].as_str().unwrap().to_string(),
                    node["regex"].as_str().unwrap().to_string(),
                    if node["fn"].is_null() {
                        "".to_string()
                    } else {
                        node["fn"].as_str().unwrap().to_string()
                    },
                    exit_map,
                );
            } else {
                self.insert_node_single(
                    &node["label"].as_str().unwrap().to_string(),
                    if node["fn"].is_null() {
                        "".to_string()
                    } else {
                        node["fn"].as_str().unwrap().to_string()
                    },
                    exit_map,
                );
            }
        }
    }
    /// ファイル上書き書込。
    /// https://qiita.com/garkimasera/items/0442ee896403c6b78fb2 |JSON文字列と構造体の相互変換
    pub fn write_file(&mut self, file: &str) {
        // 移し替え。
        let mut diagram_json = DiagramJson::new();
        // エントランス
        {
            let entry_point = &self.entry_point;
            diagram_json.set_entry_point(entry_point.to_string());
        }

        // ノード
        for (node_label, node) in &self.node_map {
            let mut node_json = NodeJson::new();
            node_json.set_label(node_label.to_string());
            if node.is_regex() {
                node_json.set_regex(Some(node.get_token().to_string()));
            } else if node.get_token() != "" {
                node_json.set_token(Some(node.get_token().to_string()));
            }
            if node.get_fn_label() != "" {
                node_json.set_fnc(Some(node.get_fn_label().to_string()));
            }

            for (exit_label, node_vec) in node.get_exit_map().iter() {
                let mut vec = Vec::new();
                for exit_node in node_vec.iter() {
                    vec.push(exit_node.to_string());
                }
                node_json.insert_exit(&exit_label.to_string(), vec);
            }

            diagram_json.push_node(node_json);
        }
        let json_str = serde_json::to_string(&diagram_json).unwrap();

        // 上書き書込。
        match &mut OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file)
        {
            Ok(contents_file) => contents_file.write_all(json_str.as_bytes()),
            Err(err) => panic!("Log file open (write mode) error. {}", err),
        };
    }
}
