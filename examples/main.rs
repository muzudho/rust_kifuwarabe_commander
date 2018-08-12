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
    shell.push_token(Token { token: "abc".to_string(), callback: do_abc});
    shell.push_token(Token { token: "quit".to_string(), callback: do_quit});
    shell.set_other_callback(do_other);

    // 実行。
    shell.run();
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
