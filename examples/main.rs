/// ```
/// ### 以下のコマンドで実行。 
/// cargo run --example main
/// ```
extern crate kifuwarabe_shell;
use kifuwarabe_shell::*;

/// [Ctrl]+[C]キー で強制終了。
fn main() {

    println!("Please enter command.");

    let mut shell = Shell::new();

    // コールバック関数を登録する。
    shell.insert_callback("CB_a", do_a);
    shell.insert_callback("CB_ab", do_ab);
    shell.insert_callback("CB_abc", do_abc);
    shell.insert_callback("CB_quit", do_quit);
    shell.insert_callback("CB_other", do_other);
    shell.set_complementary_callback("CB_other");

    // ノードを登録する。
    shell.insert_node("ND_100", Node { token: "a", callback: "CB_a"});
    shell.insert_node("ND_101", Node { token: "ab", callback: "CB_ab"});
    shell.insert_node("ND_102", Node { token: "abc", callback: "CB_abc"});
    shell.insert_node("ND_200", Node { token: "quit", callback: "CB_quit"});

    // 実行。
    shell.run();
}

pub fn do_a(line: &Commandline, _caret:&mut Caret){
    println!("A! {}", line.contents);
}

pub fn do_ab(line: &Commandline, _caret:&mut Caret){
    println!("AB! {}", line.contents);
}

pub fn do_abc(line: &Commandline, _caret:&mut Caret){
    println!("ABC! {}", line.contents);
}

pub fn do_other(line: &Commandline, _caret:&mut Caret){
    println!("Not match. {}", line.contents);
}

pub fn do_quit(line: &Commandline, caret:&mut Caret){
    println!("Quit. {}", line.contents);
    caret.quits = true;
}
