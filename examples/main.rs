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
    shell.insert_callback("CB_a".to_string(), do_a);
    shell.insert_callback("CB_ab".to_string(), do_ab);
    shell.insert_callback("CB_abc".to_string(), do_abc);
    shell.insert_callback("CB_quit".to_string(), do_quit);
    shell.insert_callback("CB_other".to_string(), do_other);
    shell.set_complementary_callback("CB_other".to_string());

    // ノードを登録する。
    shell.insert_static_node("ND_100", StaticNode { token: "a", callback: "CB_a"});
    shell.insert_static_node("ND_101", StaticNode { token: "ab", callback: "CB_ab"});
    shell.insert_static_node("ND_102", StaticNode { token: "abc", callback: "CB_abc"});
    shell.insert_static_node("ND_200", StaticNode { token: "quit", callback: "CB_quit"});

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
