/// ```
/// ### 以下のコマンドで実行。 
/// cargo run --example main
/// ```
extern crate kifuwarabe_shell;

use kifuwarabe_shell::*;

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
/// - 「quit」と打鍵して [Enter]キーを押す。
///     Quit.
/// - [Ctrl]+[C]キー を押すなら、強制終了。
fn main() {

    println!("Please enter command.");

    let mut shell = Shell::new();

    // ノードを登録する。
    shell.insert_node("ND_a", "a", do_a);
    shell.insert_node("ND_ab", "ab", do_ab);
    shell.insert_node("ND_abc", "abc", do_abc);
    shell.insert_node("ND_cde", "cde", do_cde);
    shell.insert_node("ND_end", "end", do_end);
    shell.insert_node_re("ND_numvar", r"(\d+)", do_numvar);
    shell.insert_node("ND_quit", "quit", do_quit);
    shell.insert_node_re("ND_wordvar", r"(\w+)", do_wordvar);
    // 正規表現は、うまく作れていない。全体を丸括弧で囲む。1個だけ。

    // 該当なしの場合のコールバック関数を登録する。
    shell.set_complementary_controller(do_other);

    // 開始ノードを選択する。
    shell.set_next("ND_a,ND_ab,ND_abc,ND_end,ND_numvar,
        ND_quit,ND_wordvar");

    // 実行。
    shell.run();
}

pub fn do_a(_request: &Request, _response:&mut Response){
    println!("A.");
}

pub fn do_ab(_request: &Request, response:&mut Response){
    println!("Ab.");
    response.next = "ND_cde";

    // 行終了時に実行されるコールバック関数を１つ設定できる。
    response.set_linebreak_controller(do_ab_linebreak);
}

pub fn do_ab_linebreak(_request: &Request, _response:&mut Response){
    println!("Ab-LineBreak.");
}

pub fn do_abc(_request: &Request, _response:&mut Response){
    println!("Abc.");
}

pub fn do_cde(_request: &Request, response:&mut Response){
    println!("Cde.");
    response.next = "ND_wordvar";
}

pub fn do_end(_request: &Request, response:&mut Response){
    response.done_line = true;
    println!("End.");
}

pub fn do_numvar(_request: &Request, response:&mut Response){
    let cap = &response.groups[0];
    println!("Number({}).", cap);
}

pub fn do_other(request: &Request, _response:&mut Response){
    println!("Not match. request.line=[{}], request.caret={}", request.line, request.caret);
}

pub fn do_quit(_request: &Request, response:&mut Response){
    println!("Quit.");
    response.quits = true;
}

pub fn do_wordvar(_request: &Request, response:&mut Response){
    let cap = &response.groups[0];
    println!("Word({}).", cap);
}
