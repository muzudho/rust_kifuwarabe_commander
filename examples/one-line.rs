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
use kifuwarabe_shell::graph::*;
use kifuwarabe_shell::shell::*;

mod test_scenario;
use test_scenario::*;

/// # テスト方法。
///
/// graph.json ファイルに書かれているスクリプトをテストします。
///
/// - 「ab cde」と打鍵して [Enter]キーを押す。
///     Ab.
///     Cde.
///     Ab-NewLine.
/// - 「end xyz」と打鍵して [Enter]キーを押す。
///     End.
/// - 「xyz」と打鍵して [Enter]キーを押す。
///     Word(xyz).
/// - 「ab cde xyz」と打鍵して [Enter]キーを押す。
///     Ab.
///     Cde.
///     Word(xyz).
///     Ab-NewLine.
/// - 「quit」と打鍵して [Enter]キーを押す。
///     Quit.
/// - 強制終了したいなら、[Ctrl]+[C]キー を押す。
///
/// - また、「reload」と打鍵して [Enter]キーを押す。
///     Reload.
///     graph.json ファイルを再読み込みするはず。
fn main() {
    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();
    // シェルの作成。
    let mut shell = Shell::new();

    // グラフの作成。
    let mut graph : Graph<ShellVar> = Graph::new();
    setup_graph(&mut graph); // test_scenario.rs 参照。

    // 内容確認出力。
    {
        println!("entrance");
        for node in graph.get_entrance_vec().iter() {
            println!("  - {}", node);
        }

        println!("nodes");
        for (node_label, node) in graph.get_node_map().iter() {
            println!("  - {} {}", node_label, node.get_token());
            for (exits_label, exits_vec) in node.get_exits_map().iter() {
                println!("    - {}", exits_label);
                for exits_item in exits_vec.iter() {
                    println!("      - {}", exits_item);
                }
            }
        }
    }

    // ****************************************************************************************************
    //  実行。
    // ****************************************************************************************************
    println!("Please enter command.");
    shell.execute_line(&mut graph, &mut shell_var, "ab cde xyz");
    println!("Finished. shell_var.count: {}.", shell_var.count);
}
