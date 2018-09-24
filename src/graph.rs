/// アプリケーション１つにつき、１つのグラフを持ちます。
// 参考:
// https://github.com/serde-rs/json
extern crate serde_json;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::io::Write;

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
    token: String,
    fn_label: String,
    regex_flag: bool,
    // 特殊な任意の名前 '#newline'
    exits: HashMap<String, Vec<String>>,
}
impl Node {
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
    pub fn get_exits_map(&self) -> &HashMap<String, Vec<String>> {
        &self.exits
    }
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

pub fn empty_controller<T>(_t: &mut T, _req: &Request, _res: &mut dyn Response) {}

/// # Parameters.
///
/// * `node_map` - 複数件のトークンです。
/// * `entrance_vec` - カンマ区切りの登録ノード名です。
#[derive(Default)]
pub struct Graph<T> {
    /// 任意の名前と、コントローラー。
    fn_map: HashMap<String, Controller<T>>,
    entrance_vec: Vec<String>,
    /// 特殊なノード名
    /// '#else' 一致するトークンが無かったときに呼び出されるコールバック関数です。
    node_map: HashMap<String, Node>,
}
impl<T> Graph<T> {
    /// アプリケーション１つにつき、１つのフローチャートを共有します。
    pub fn new() -> Graph<T> {
        Graph {
            node_map: HashMap::new(),
            entrance_vec: Vec::new(),
            fn_map: HashMap::new(),
        }
    }
    /// 確認用。
    pub fn get_node_map(&self) -> &HashMap<String, Node> {
        &self.node_map
    }
    /// クリアー。（登録したコントローラーを除く）
    pub fn clear_graph(&mut self) {
        self.node_map.clear();
        self.entrance_vec.clear();
    }
    pub fn get_entrance_vec(&self) -> &Vec<String> {
        &self.entrance_vec
    }
    pub fn set_entrance_vec(&mut self, entrance_vec2: Vec<String>) {
        self.entrance_vec = entrance_vec2;
    }
    pub fn get_node(&self, label: &str) -> &Node {
        if self.contains_node(&label.to_string()) {
            &self.node_map[label]
        } else {
            panic!("\"{}\" node is not found.", label);
        }
    }
    pub fn contains_node(&self, label: &str) -> bool {
        self.node_map.contains_key(&label.to_string())
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
        self.node_map.insert(
            label,
            Node {
                token: token2,
                fn_label: fn_label2,
                regex_flag: false,
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
        self.node_map.insert(
            label.to_string(),
            Node {
                token: token2,
                fn_label: fn_label2,
                regex_flag: true,
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
        self.node_map.insert(
            label.to_string(),
            Node {
                token: "".to_string(),
                fn_label: fn_label2,
                regex_flag: false,
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
    pub fn read_graph_file(&mut self, file: &str) {
        self.clear_graph();

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

        // 文字列に変換する。
        let mut entrance_vec: Vec<String> = Vec::new();
        self.array_to_str_vec(&v["entrance"], &mut entrance_vec);
        self.set_entrance_vec(entrance_vec);

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
    /// TODO ファイル上書き書込。
    /// TODO バッファーせず、ストリームで保存したい。
    /// FIXME デシリアライズが分からないので自作している☆（＾～＾）
    /// TODO https://qiita.com/garkimasera/items/0442ee896403c6b78fb2 |JSON文字列と構造体の相互変換
    pub fn save_graph_file(&mut self, file: &str) {
        println!("セーブは開発中");

        // 上書き書込。
        let file_str = &format!("{}{}", file, ".TEST.json");

        // JSON ではなく、 Graph 構造体が持っている。
        let mut contents = String::new();

        contents.push_str(
            r#"{
    "entrance": [
"#,
        );
        // エントランス
        let mut i = 0;
        for node_label in &self.entrance_vec {
            if 0 < i {
                // カンマ
                contents.push_str(
                    r#",
"#,
                );
            }
            contents.push_str(&format!(r#"        "{}""#, node_label));
            i += 1;
        }

        contents.push_str(
            r#"
    ],
    "nodes": [
"#,
        );
        // ノード
        let mut j = 0;
        for (_node_label, _node) in &self.node_map {
            if 0 < j {
                // カンマ 改行
                contents.push_str(
                    r#",
"#,
                );
            }
            contents.push_str(
                r#"        {
"#,
            );

            contents.push_str(
                r#"            "label": "AAAA"
"#,
            );
            contents.push_str(
                r#"            "token": "AAAA"
"#,
            );
            contents.push_str(
                r#"            "regex": "AAAA"
"#,
            );
            contents.push_str(
                r#"            "fn": "AAAA"
"#,
            );
            contents.push_str(
                r#"            "exit": "AAAA"
"#,
            );

            contents.push_str(r#"        }"#);
            j += 1;
        }

        contents.push_str(
            r#"    ]
}"#,
        );

        // 全部書込み。
        match &mut OpenOptions::new().create(true).write(true).open(file_str) {
            Ok(contents_file) => contents_file.write_all(contents.as_bytes()),
            Err(err) => panic!("Log file open (write mode) error. {}", err),
        };
    }
}
