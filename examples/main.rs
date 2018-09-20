/// ```
/// ### 以下のコマンドで実行。 
/// cargo run --example main
/// ```
extern crate kifuwarabe_shell;

use kifuwarabe_shell::graph::*;
use kifuwarabe_shell::node::*;
use kifuwarabe_shell::shell::*;


// 任意のオブジェクト。
pub struct ShellVar {
    pub count: i32,
}
impl ShellVar {
    fn new() -> ShellVar {
        ShellVar {
            count: 0,
        }
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

    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();

    // グラフの作成。
    let mut graph = new_graph();

    // シェルの作成。
    let mut shell = new_shell();

    // ノードを登録する。
    insert_node(&mut graph, "ND_a", "a", do_a);
    insert_node(&mut graph, "ND_ab", "ab", do_ab);
    insert_node(&mut graph, "ND_abc", "abc", do_abc);
    insert_node(&mut graph, "ND_cde", "cde", do_cde);
    insert_node(&mut graph, "ND_end", "end", do_end);
    insert_node_re(&mut graph, "ND_numvar", r"(\d+)", do_numvar);
    insert_node(&mut graph, "ND_quit", "quit", do_quit);
    insert_node_re(&mut graph, "ND_wordvar", r"(\w+)", do_wordvar);
    // 正規表現は、うまく作れていない。全体を丸括弧で囲む。1個だけ。

    // 該当なしの場合のコールバック関数を登録する。
    set_complementary_controller(&mut graph, do_other);

    // 開始ノードを選択する。
    set_next(&mut shell, "ND_a,ND_ab,ND_abc,ND_end,ND_numvar,
        ND_quit,ND_wordvar");

    // 実行。
    run(&mut graph, &mut shell, &mut shell_var);

    println!("shell_var.count: {}", shell_var.count);
}



pub fn do_a(shell_var: &mut ShellVar, _request: &Box<RequestAccessor>, _response:&mut Box<dyn ResponseAccessor<ShellVar>>){
    shell_var.count += 1;
    println!("A.");
}

pub fn do_ab(shell_var: &mut ShellVar, _request: &Box<RequestAccessor>, response:&mut Box<dyn ResponseAccessor<ShellVar>>){
    shell_var.count += 1;
    println!("Ab.");
    response.set_next("ND_cde");

    // 行終了時に実行されるコールバック関数を１つ設定できる。
    set_linebreak_controller(response, do_ab_linebreak);
}

pub fn do_ab_linebreak(shell_var: &mut ShellVar, _request: &Box<RequestAccessor>, _response:&mut Box<dyn ResponseAccessor<ShellVar>>){
    shell_var.count += 1;
    println!("Ab-LineBreak.");
}

pub fn do_abc(shell_var: &mut ShellVar, _request: &Box<RequestAccessor>, _response:&mut Box<dyn ResponseAccessor<ShellVar>>){
    shell_var.count += 1;
    println!("Abc.");
}

pub fn do_cde(shell_var: &mut ShellVar, _request: &Box<RequestAccessor>, response:&mut Box<dyn ResponseAccessor<ShellVar>>){
    shell_var.count += 1;
    println!("Cde.");
    response.set_next("ND_wordvar");
}

pub fn do_end(shell_var: &mut ShellVar, _request: &Box<RequestAccessor>, response:&mut Box<dyn ResponseAccessor<ShellVar>>){
    shell_var.count += 1;
    response.set_done_line(true);
    println!("End.");
}

pub fn do_numvar(shell_var: &mut ShellVar, _request: &Box<RequestAccessor>, response:&mut Box<dyn ResponseAccessor<ShellVar>>){
    shell_var.count += 1;
    let cap = &response.get_groups()[0];
    println!("Number({}).", cap);
}

pub fn do_other(shell_var: &mut ShellVar, request: &Box<RequestAccessor>, _response:&mut Box<dyn ResponseAccessor<ShellVar>>){
    shell_var.count += 1;
    println!("Not match. request.line=[{}], request.caret={}", request.get_line(), request.get_caret());
}

pub fn do_quit(shell_var: &mut ShellVar, _request: &Box<RequestAccessor>, response:&mut Box<dyn ResponseAccessor<ShellVar>>){
    shell_var.count += 1;
    println!("Quit.");
    response.set_quits(true);
}

pub fn do_wordvar(shell_var: &mut ShellVar, _request: &Box<RequestAccessor>, response:&mut Box<dyn ResponseAccessor<ShellVar>>){
    shell_var.count += 1;
    let cap = &response.get_groups()[0];
    println!("Word({}).", cap);
}
