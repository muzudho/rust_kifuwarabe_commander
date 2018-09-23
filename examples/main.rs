/// ```
/// ### 以下のコマンドで実行。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shell
/// cargo run --example main
/// ```
// #[macro_use(hashmap)]
extern crate kifuwarabe_shell;

use kifuwarabe_shell::graph::*;
use kifuwarabe_shell::shell::*;
// use std::collections::HashMap;

// 参考:
// https://github.com/serde-rs/json
extern crate serde_json;
// use serde_json::Value;
// use std::fs::File;
// use std::io::Read;

// 任意のオブジェクト。
pub struct ShellVar {
    pub count: i32,
}
impl ShellVar {
    fn new() -> ShellVar {
        ShellVar { count: 0 }
    }
}


/// # テスト方法。
///
/// - 「ab cde」と打鍵して [Enter]キーを押す。
///     Ab.
///     Cde.
///     Ab-LineBreak.
/// - 「end xyz」と打鍵して [Enter]キーを押す。
///     End.
/// - 「xyz」と打鍵して [Enter]キーを押す。
///     Word(xyz).
/// - 「ab cde xyz」と打鍵して [Enter]キーを押す。
///     Ab.
///     Cde.
///     Word(xyz).
///     Ab-LineBreak.
/// - 「quit」と打鍵して [Enter]キーを押す。
///     Quit.
/// - 強制終了したいなら、[Ctrl]+[C]キー を押す。
fn main() {
    // グラフの作成。
    let mut graph = Graph::new();
    // コントローラーを登録。
    graph.insert_controller("do_a", do_a);
    graph.insert_controller("do_ab", do_ab);
    graph.insert_controller("do_abc", do_abc);
    graph.insert_controller("do_cde", do_cde);
    graph.insert_controller("do_end", do_end);
    graph.insert_controller("do_numvar", do_numvar);
    graph.insert_controller("do_quit", do_quit);
    graph.insert_controller("do_wordvar", do_wordvar);
    graph.insert_controller("do_ab_linebreak", do_ab_linebreak);
    graph.insert_controller("do_other", do_other);

    // グラフのノード構成。
    graph.read_graph_file("graph.json".to_string());
    /*
    graph.insert_node("ND_a".to_string(), "a".to_string(), "do_a".to_string(), hashmap![]);
    graph.insert_node(
        "ND_ab".to_string(),
        "ab".to_string(),
        "do_ab".to_string(),
        hashmap!["next".to_string() => "ND_cde".to_string(), "#linebreak".to_string() => "ND_ab_linebreak".to_string()],
    ); // #linebreak コールバック関数は行終了時に実行される。
    graph.insert_node("ND_abc".to_string(), "abc".to_string(), "do_abc".to_string(), hashmap![]);
    graph.insert_node("ND_cde".to_string(), "cde".to_string(), "do_cde".to_string(), hashmap!["next".to_string() => "ND_wordvar".to_string()]);
    graph.insert_node("ND_end".to_string(), "end".to_string(), "do_end".to_string(), hashmap![]);
    graph.insert_node_reg("ND_numvar".to_string(), r"(\d+)".to_string(), "do_numvar".to_string(), hashmap![]);
    graph.insert_node("ND_quit".to_string(), "quit".to_string(), "do_quit".to_string(), hashmap![]);
    graph.insert_node_reg("ND_wordvar".to_string(), r"(\w+)".to_string(), "do_wordvar".to_string(), hashmap![]);
    graph.insert_node_single("ND_ab_linebreak".to_string(), "do_ab_linebreak".to_string());
    graph.insert_node_single("#ND_complementary".to_string(), "do_other".to_string()); // 該当なしの場合のコールバック関数を登録する。
                                                             // 正規表現は、うまく作れていない。全体を丸括弧で囲む。1個だけ。
                                                             // 開始ノードを選択する。
    graph.set_entrance(
        "ND_a,ND_ab,ND_abc,ND_end,ND_numvar,
        ND_quit,ND_wordvar".to_string(),
    );
    */

    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();
    // シェルの作成。
    let mut shell = Shell::new();

    // 実行。
    println!("Please enter command.");
    shell.run(&mut graph, &mut shell_var);

    println!("shell_var.count: {}", shell_var.count);
}

pub fn do_a(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("A.");
}

pub fn do_ab(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("Ab.");
    response.forward("next".to_string());
}

pub fn do_ab_linebreak(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("Ab-LineBreak.");
}

pub fn do_abc(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("Abc.");
}

pub fn do_cde(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("Cde.");
    response.forward("next".to_string());
}

pub fn do_end(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    response.set_done_line(true);
    println!("End.");
}

pub fn do_numvar(
    shell_var: &mut ShellVar,
    request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    let cap = &request.get_groups()[0];
    println!("Number({}).", cap);
}

pub fn do_other(
    shell_var: &mut ShellVar,
    request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!(
        "Not match. request.line=[{}], request.caret={}",
        request.get_line(),
        request.get_caret()
    );
}

pub fn do_quit(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("Quit.");
    response.set_quits(true);
}

pub fn do_wordvar(
    shell_var: &mut ShellVar,
    request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    let cap = &request.get_groups()[0];
    println!("Word({}).", cap);
}
