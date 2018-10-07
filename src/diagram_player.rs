use diagram::*;
use shell::*;
use regex::Regex;

/// 不具合を取りたいときに真にする。
const VERBOSE: bool = false;

/// ダイアグラム再生機。
///
/// # Members.
///
/// * `current_label` - 現在のノードのラベル。
pub struct DiagramPlayer {
    current_label: String,
}
impl Default for DiagramPlayer {
    fn default() -> Self {
        Self::new()
    }
}
impl DiagramPlayer {
    pub fn new() -> DiagramPlayer {
        DiagramPlayer {
            current_label: "".to_string(),
        }
    }

    /// 現在ノードのラベル。
    pub fn get_current(&self) -> String {
        self.current_label.to_string()
    }

    /// 現在地が遷移図の外か。
    pub fn is_out(&self) -> bool {
        self.current_label == ""
    }

    /// 現在ノードのラベル。
    pub fn set_current(&mut self, value:&str) {
        self.current_label = value.to_string()
    }

    /// 入り口に入っていないなら、入り口に進む。
    pub fn enter_when_out<T>(&mut self,diagram: &Diagram<T>) {
        // println!("元入り口: [{}].", self.current_label);
        if self.is_out() {
            self.set_current(&diagram.get_entry_point().to_string());
            // println!("入り口を初期化: [{}].", self.current_label);
        }
    }
}

