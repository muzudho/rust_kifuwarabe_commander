/// ```
/// ### 以下のコマンドで実行。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shell
/// cargo run --example main
/// ```

// 参考:
// https://github.com/serde-rs/json |serde_json
extern crate serde_json;
extern crate kifuwarabe_shell;
use kifuwarabe_shell::graph::*;
use kifuwarabe_shell::shell::*;


// 任意のオブジェクト。
pub struct ShellVar {
    pub count: i32,
}
impl ShellVar {
    fn new() -> ShellVar {
        ShellVar { count: 0 }
    }
}

const GRAPH_JSON_FILE : &'static str = "graph.json";
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
    // グラフの作成。
    let mut graph = Graph::new();
    // コントローラーを登録。
    graph.insert_fn("do_a", do_a);
    graph.insert_fn("do_ab", do_ab);
    graph.insert_fn("do_abc", do_abc);
    graph.insert_fn("do_cde", do_cde);
    graph.insert_fn("do_end", do_end);
    graph.insert_fn("do_numvar", do_numvar);
    graph.insert_fn("do_quit", do_quit);
    graph.insert_fn("do_wordvar", do_wordvar);
    graph.insert_fn("do_ab_newline", do_ab_newline);
    graph.insert_fn("do_other", do_other);
    graph.insert_fn("do_reload", do_reload);

    // ファイルからグラフのノード構成を読取。
    graph.read_graph_file(GRAPH_JSON_FILE.to_string());

    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();
    // シェルの作成。
    let mut shell = Shell::new();

    // 実行。
    println!("Please enter command.");
    shell.run(&mut graph, &mut shell_var);
    println!("Finished. shell_var.count: {}.", shell_var.count);
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

pub fn do_ab_newline(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("Ab-NewLine.");
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
    _shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    response: &mut dyn ResponseAccessor,
) {
    println!("Reload.");
    response.set_reloads(GRAPH_JSON_FILE);
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
