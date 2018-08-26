/// ```
/// ### 以下のコマンドで実行。 
/// cargo run --example main
/// ```
extern crate kifuwarabe_shell;

use kifuwarabe_shell::*;



// 任意のオブジェクト。
struct ShellVar {

}
impl ShellVar {
    pub fn new() -> ShellVar {
        ShellVar {

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
/// - [Ctrl]+[C]キー を押すなら、強制終了。
fn main() {

    println!("Please enter command.");

    let mut shell_var = ShellVar::new();

    let mut shell = new_shell();

    // ノードを登録する。
    insert_node(&mut shell, "ND_a", "a", do_a);
    insert_node(&mut shell, "ND_ab", "ab", do_ab);
    insert_node(&mut shell, "ND_abc", "abc", do_abc);
    insert_node(&mut shell, "ND_cde", "cde", do_cde);
    insert_node(&mut shell, "ND_end", "end", do_end);
    insert_node_re(&mut shell, "ND_numvar", r"(\d+)", do_numvar);
    insert_node(&mut shell, "ND_quit", "quit", do_quit);
    insert_node_re(&mut shell, "ND_wordvar", r"(\w+)", do_wordvar);
    // 正規表現は、うまく作れていない。全体を丸括弧で囲む。1個だけ。

    // 該当なしの場合のコールバック関数を登録する。
    set_complementary_controller(&mut shell, do_other);

    // 開始ノードを選択する。
    set_next(&mut shell, "ND_a,ND_ab,ND_abc,ND_end,ND_numvar,
        ND_quit,ND_wordvar");

    // 実行。
    run(&mut shell, &mut shell_var);
}




pub fn do_a<ShellVar>(_t: &mut ShellVar, _request: &Request, _response:&mut Response<ShellVar>){
    println!("A.");
}

pub fn do_ab<ShellVar>(_t: &mut ShellVar, _request: &Request, response:&mut Response<ShellVar>){
    println!("Ab.");
    response.next = "ND_cde";

    // 行終了時に実行されるコールバック関数を１つ設定できる。
    set_linebreak_controller(response, do_ab_linebreak);
}

pub fn do_ab_linebreak<ShellVar>(_t: &mut ShellVar, _request: &Request, _response:&mut Response<ShellVar>){
    println!("Ab-LineBreak.");
}

pub fn do_abc<ShellVar>(_t: &mut ShellVar, _request: &Request, _response:&mut Response<ShellVar>){
    println!("Abc.");
}

pub fn do_cde<ShellVar>(_t: &mut ShellVar, _request: &Request, response:&mut Response<ShellVar>){
    println!("Cde.");
    response.next = "ND_wordvar";
}

pub fn do_end<ShellVar>(_t: &mut ShellVar, _request: &Request, response:&mut Response<ShellVar>){
    response.done_line = true;
    println!("End.");
}

pub fn do_numvar<ShellVar>(_t: &mut ShellVar, _request: &Request, response:&mut Response<ShellVar>){
    let cap = &response.groups[0];
    println!("Number({}).", cap);
}

pub fn do_other<ShellVar>(_t: &mut ShellVar, request: &Request, _response:&mut Response<ShellVar>){
    println!("Not match. request.line=[{}], request.caret={}", request.line, request.caret);
}

pub fn do_quit<ShellVar>(_t: &mut ShellVar, _request: &Request, response:&mut Response<ShellVar>){
    println!("Quit.");
    response.quits = true;
}

pub fn do_wordvar<ShellVar>(_t: &mut ShellVar, _request: &Request, response:&mut Response<ShellVar>){
    let cap = &response.groups[0];
    println!("Word({}).", cap);
}
