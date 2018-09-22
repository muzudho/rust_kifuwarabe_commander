/// ```
/// ### 以下のコマンドで実行。
/// cargo run --example main
/// ```
#[macro_use(hashmap)]
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
    println!("Please enter command.");

    // グラフの作成。
    let mut graph = new_graph();
    // グラフのノード構成。
    insert_node(&mut graph, "ND_a", "a", do_a, hashmap![]);
    insert_node(
        &mut graph,
        "ND_ab",
        "ab",
        do_ab,
        hashmap!["next" => "ND_cde", "#linebreak" => "ND_ab_linebreak"],
    ); // #linebreak コールバック関数は行終了時に実行される。
    insert_node(&mut graph, "ND_abc", "abc", do_abc, hashmap![]);
    insert_node(
        &mut graph,
        "ND_cde",
        "cde",
        do_cde,
        hashmap!["next" => "ND_wordvar"],
    );
    insert_node(&mut graph, "ND_end", "end", do_end, hashmap![]);
    insert_node_re(&mut graph, "ND_numvar", r"(\d+)", do_numvar, hashmap![]);
    insert_node(&mut graph, "ND_quit", "quit", do_quit, hashmap![]);
    insert_node_re(&mut graph, "ND_wordvar", r"(\w+)", do_wordvar, hashmap![]);
    insert_node_single(&mut graph, "ND_ab_linebreak", do_ab_linebreak);
    insert_node_single(&mut graph, "#ND_complementary", do_other); // 該当なしの場合のコールバック関数を登録する。
                                                                   // 正規表現は、うまく作れていない。全体を丸括弧で囲む。1個だけ。
                                                                   // 開始ノードを選択する。
    graph.set_entrance(
        "ND_a,ND_ab,ND_abc,ND_end,ND_numvar,
        ND_quit,ND_wordvar",
    );

    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();
    // シェルの作成。
    let mut shell = Shell::new();

    // 実行。
    shell.run(&mut graph, &mut shell_var);

    println!("shell_var.count: {}", shell_var.count);
}

pub fn do_a(
    shell_var: &mut ShellVar,
    _request: &Box<RequestAccessor>,
    _response: &mut Box<dyn ResponseAccessor>,
) {
    shell_var.count += 1;
    println!("A.");
}

pub fn do_ab(
    shell_var: &mut ShellVar,
    _request: &Box<RequestAccessor>,
    response: &mut Box<dyn ResponseAccessor>,
) {
    shell_var.count += 1;
    println!("Ab.");
    response.forward("next");
}

pub fn do_ab_linebreak(
    shell_var: &mut ShellVar,
    _request: &Box<RequestAccessor>,
    _response: &mut Box<dyn ResponseAccessor>,
) {
    shell_var.count += 1;
    println!("Ab-LineBreak.");
}

pub fn do_abc(
    shell_var: &mut ShellVar,
    _request: &Box<RequestAccessor>,
    _response: &mut Box<dyn ResponseAccessor>,
) {
    shell_var.count += 1;
    println!("Abc.");
}

pub fn do_cde(
    shell_var: &mut ShellVar,
    _request: &Box<RequestAccessor>,
    response: &mut Box<dyn ResponseAccessor>,
) {
    shell_var.count += 1;
    println!("Cde.");
    response.forward("next");
}

pub fn do_end(
    shell_var: &mut ShellVar,
    _request: &Box<RequestAccessor>,
    response: &mut Box<dyn ResponseAccessor>,
) {
    shell_var.count += 1;
    response.set_done_line(true);
    println!("End.");
}

pub fn do_numvar(
    shell_var: &mut ShellVar,
    request: &Box<RequestAccessor>,
    _response: &mut Box<dyn ResponseAccessor>,
) {
    shell_var.count += 1;
    let cap = &request.get_groups()[0];
    println!("Number({}).", cap);
}

pub fn do_other(
    shell_var: &mut ShellVar,
    request: &Box<RequestAccessor>,
    _response: &mut Box<dyn ResponseAccessor>,
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
    _request: &Box<RequestAccessor>,
    response: &mut Box<dyn ResponseAccessor>,
) {
    shell_var.count += 1;
    println!("Quit.");
    response.set_quits(true);
}

pub fn do_wordvar(
    shell_var: &mut ShellVar,
    request: &Box<RequestAccessor>,
    _response: &mut Box<dyn ResponseAccessor>,
) {
    shell_var.count += 1;
    let cap = &request.get_groups()[0];
    println!("Word({}).", cap);
}
