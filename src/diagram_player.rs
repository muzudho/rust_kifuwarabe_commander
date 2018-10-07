use diagram::*;
use shell::*;

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

    // cyclomatic complexity を避けたいだけ。
    pub fn parse_line_else<T>(
        &self,
        diagram: &Diagram<T>,
        t: &mut T,
        req: &mut dyn Request,
        res: &mut dyn Response,
    ) {
        if diagram.contains_node(&ELSE_NODE_LABEL.to_string()) {
            let fn_label = diagram
                .get_node(&ELSE_NODE_LABEL.to_string())
                .get_fn_label();
            if diagram.contains_fn(&fn_label) {
                // ****************************************************************************************************
                //  コールバック関数を実行。
                // ****************************************************************************************************
                (diagram.get_fn(&fn_label))(t, req, res);
            // responseは無視する。
            } else {
                // 無い関数が設定されていた場合は、コンソール表示だけする。
                println!(
                    "IGNORE: \"{}\" fn (in {} node) is not found.",
                    &fn_label, ELSE_NODE_LABEL
                );
            }
        }
    }

}

