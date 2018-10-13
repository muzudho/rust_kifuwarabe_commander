use diagram::*;
use line_parser::*;

/// 不具合を取りたいときに真にする。
const VERBOSE: bool = false;

/// ダイアグラム再生機。
///
/// # Members.
///
/// * `current_label` - 現在のノードのラベル。
pub struct DiagramPlayer {
    current_label: String
}
impl Default for DiagramPlayer {
    fn default() -> Self {
        Self::new()
    }
}
impl DiagramPlayer {
    pub fn new() -> DiagramPlayer {
        DiagramPlayer {
            current_label: "".to_string()
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
    pub fn set_current(&mut self, value: &str) {
        self.current_label = value.to_string()
    }

    /// 入り口に入っていないなら、入り口に進む。
    pub fn enter_when_out<T>(&mut self, diagram: &Diagram<T>) {
        // println!("元入り口: [{}].", self.current_label);
        if self.is_out() {
            self.set_current(&diagram.get_entry_point().to_string());
            // println!("入り口を初期化: [{}].", self.current_label);
        }
    }

    /// 次に一致するノード名。
    /// `req` - 正規表現で一致があれば、 groups メンバーに内容を入れる。
    pub fn forward<T>(
        &self,
        diagram: &Diagram<T>,
        req: &mut dyn Request,
        current_exit_map: &[String],
    ) -> (String, bool) {
        // 一番優先されるものを探す。
        let mut best_node_label = "".to_string();
        let mut best_node_re_label = "".to_string();

        // 次の候補。
        let mut max_token_len = 0;
        for i_next_node_label in current_exit_map {
            let next_node_label = i_next_node_label.trim();
            // println!("next_node_label: {}", next_node_label);
            if diagram.contains_node(&next_node_label.to_string()) {
                //println!("contains.");

                let node_name = next_node_label.to_string();
                let node = &diagram.get_node(&node_name);

                let matched;
                if node.is_regex() {
                    if LineParser::starts_with_reg(node, req) {
                        // 正規表現で一致したなら。
                        best_node_re_label = node_name;
                        // 固定長で一致するものも探したい。
                    }
                } else {
                    matched = LineParser::starts_with_literal(node, req);
                    if matched {
                        //println!("starts_with_literal.");
                        let token_len = node.get_token().chars().count();
                        if max_token_len < token_len {
                            max_token_len = token_len;
                            best_node_label = node_name;
                            // まだ、一番長い、固定長トークンを探したい。
                        };
                        //} else {
                        //    println!("not starts_with_literal. req.line={}, req.line_len={}, res.starts={}", req.line, req.line_len, res.starts);
                    }
                }
            }
        }

        if best_node_label != "" {
            // 固定長での一致を優先。
            return (best_node_label, false);
        }
        // 正規表現は優先度低い。
        (best_node_re_label, true)
    }
}
