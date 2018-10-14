/// ```
/// ### 以下のコマンドで実行。
/// cls
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shell
/// cargo run --example main
/// ```

// 参考:
// https://github.com/serde-rs/json |serde_json
extern crate serde_json;
extern crate kifuwarabe_shell;
use kifuwarabe_shell::diagram::*;
use kifuwarabe_shell::shell::*;

mod test_scenario;
use test_scenario::*;

/// # テスト方法。
///
/// diagram.json ファイルに書かれているスクリプトをテストします。
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
///     diagram.json ファイルを再読み込みするはず。
fn main() {
    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();
    // シェルの作成。
    let mut shell = Shell::new();

    // ダイアグラムの作成。
    let mut diagram = Diagram::new();
    setup_diagram(&mut diagram); // test_scenario.rs 参照。
    // ダイアグラムの入り口に遷移。
    shell.enter(&diagram);

    // 内容確認出力。
    {
        println!("entry_point: {}", diagram.get_entry_point());
        println!("nodes");
        for (node_label, node) in diagram.get_node_map().iter() {
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
    shell.run(&mut diagram, &mut shell_var);
    println!("Finished. shell_var.count: {}.", shell_var.count);
}
