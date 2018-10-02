/// .json ファイルを書き出す際に使う。
extern crate serde_json;

use std::collections::HashMap;

/// JSONを出力するときにだけ使う入れ物。
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct DiagramJson {
    entry_point: String,
    nodes: Vec<NodeJson>,
}
impl DiagramJson {
    pub fn new() -> DiagramJson {
        DiagramJson {
            entry_point: "".to_string(),
            nodes: Vec::new(),
        }
    }
    pub fn get_entry_point(&self) -> String {
        self.entry_point.to_string()
    }
    pub fn set_entry_point(&mut self, value: String) {
        self.entry_point = value;
    }
    pub fn get_nodes(&self) -> &Vec<NodeJson> {
        &self.nodes
    }
    pub fn push_node(&mut self, node:NodeJson) {
        self.nodes.push(node);
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct NodeJson {
    label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fn")]
    fnc: Option<String>, // fn がキーワードで使えない。

    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    #[serde(rename = "exit")]
    exit_map: HashMap<String, Vec<String>>,
}
impl NodeJson {
    pub fn new() -> NodeJson {
        NodeJson {
            label: "".to_string(),
            token: None,
            regex: None,
            fnc: None,
            exit_map: HashMap::new(),
        }
    }
    pub fn get_label(&self) -> String {
        self.label.to_string()
    }
    pub fn set_label(&mut self, value: String) {
        self.label = value;
    }
    pub fn get_token(&self) -> &Option<String> {
        &self.token
    }
    pub fn set_token(&mut self, value: Option<String>) {
        self.token = value;
    }
    pub fn get_regex(&self) -> &Option<String> {
        &self.regex
    }
    pub fn set_regex(&mut self, value: Option<String>) {
        self.regex = value;
    }
    pub fn get_fnc(&self) -> &Option<String> {
        &self.fnc
    }
    pub fn set_fnc(&mut self, value: Option<String>) {
        self.fnc = value;
    }
    pub fn get_exit_map(&self) -> &HashMap<String, Vec<String>> {
        &self.exit_map
    }
    pub fn insert_exit(&mut self, exit_label:&str, entrance_nodes:Vec<String>) {
        self.exit_map.insert(exit_label.to_string(), entrance_nodes);
    }
}
