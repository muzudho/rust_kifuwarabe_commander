extern crate kifuwarabe_shell;
/// 1行実行するだけ。
/// ```
/// ### 以下のコマンドで実行。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shell
/// cargo run --example one-line
/// ```
// 参考:
// https://github.com/serde-rs/json |serde_json
extern crate serde_json;
use kifuwarabe_shell::diagram::*;
use kifuwarabe_shell::shell::*;

mod test_scenario;
use test_scenario::*;

/// # テスト方法。
///
/// diagram.json ファイルに書かれているスクリプトをテストします。
///
/// - 次のように表示される。
///     Ab.
///     Cde.
///     Word(xyz).
///     Ab-NewLine.
fn main() {
    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();
    // シェルの作成。
    let mut shell = Shell::new();

    // グラフの作成。
    let mut diagram : DiagramEx<ShellVar> = DiagramEx::new();
    setup_diagram(&mut diagram); // test_scenario.rs 参照。

    // 内容確認出力。
    {
        println!("entry_point: {}", diagram.get_diagram().get_entry_point());
        println!("nodes");
        for (node_label, node) in diagram.get_diagram().get_node_map().iter() {
            println!("  - {} {}", node_label, node.get_token());
            for (exit_label, exit_vec) in node.get_exit_map().iter() {
                println!("    - {}", exit_label);
                for exit_item in exit_vec.iter() {
                    println!("      - {}", exit_item);
                }
            }
        }
    }

    // ****************************************************************************************************
    //  実行。
    // ****************************************************************************************************
    println!("Please enter command.");
    shell.execute_line(&mut diagram, &mut shell_var, "ab cde xyz");
    println!("Finished. shell_var.count: {}.", shell_var.count);
}
