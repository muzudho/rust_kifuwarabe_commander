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

const graph_json_file : &'static str = "graph.json";
/// # テスト方法。
///
/// graph.json ファイルに書かれているスクリプトをテストします。
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
/// 
/// - また、「reload」と打鍵して [Enter]キーを押す。
///     Reload.
///     graph.json ファイルを再読み込みするはず。
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
    graph.insert_controller("do_reload", do_reload);

    // ファイルからグラフのノード構成を読取。
    graph.read_graph_file(graph_json_file.to_string());
    // - 正規表現は、うまく作れていない。全体を丸括弧で囲む。1個だけ。
    // - #linebreak コールバック関数は行終了時に実行される。

    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();
    // シェルの作成。
    let mut shell = Shell::new();

    // 実行。
    println!("Please enter command.");
    shell.run(&mut graph, &mut shell_var);
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
    response.forward("next");
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
    response.forward("next");
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

pub fn do_reload(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    response: &mut dyn ResponseAccessor,
) {
    println!("Reload.");
    response.set_reloads(graph_json_file);
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
