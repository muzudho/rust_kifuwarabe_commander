/// ```
/// ### 以下のコマンドで実行。 
/// cargo run --example main
/// ```
extern crate kifuwarabe_shell;

use kifuwarabe_shell::*;

/// # テスト方法。
///
/// - 「ab cde」と打鍵して [Enter]キーを押す。
/// - 「end xyz」と打鍵して [Enter]キーを押す。
/// - 「quit」と打鍵して [Enter]キーを押す。
/// - [Ctrl]+[C]キー を押すなら、強制終了。
fn main() {

    println!("Please enter command.");

    let mut shell = Shell::new();

    // コールバック関数を登録する。
    shell.insert_callback("CB_a", do_a);
    shell.insert_callback("CB_ab", do_ab);
    shell.insert_callback("CB_abc", do_abc);
    shell.insert_callback("CB_cde", do_cde);
    shell.insert_callback("CB_end", do_end);
    shell.insert_callback("CB_num", do_num);
    shell.insert_callback("CB_quit", do_quit);
    shell.insert_callback("CB_other", do_other);
    shell.set_complementary_callback("CB_other");

    // ノードを登録する。
    shell.insert_node("ND_a", "a", "CB_a");
    shell.insert_node("ND_ab", "ab", "CB_ab");
    shell.insert_node("ND_abc", "abc", "CB_abc");
    shell.insert_node("ND_cde", "cde", "CB_cde");
    shell.insert_node("ND_end", "end", "CB_end");
    shell.insert_node("ND_quit", "quit", "CB_quit");
    // 正規表現は、うまく作れていない。全体を丸括弧で囲む。1個だけ。
    shell.insert_node_re("ND_num", r"(\d+)", "CB_num");

    // 開始ノードを選択する。
    shell.set_next("ND_a,ND_ab,ND_abc,ND_end,ND_quit,ND_num");

    // 実行。
    shell.run();
}

pub fn do_a(line: &Commandline, _caret:&mut Caret){
    println!("A! [{}]", line.contents);
}

pub fn do_ab(line: &Commandline, caret:&mut Caret){
    println!("AB! [{}]", line.contents);
    caret.next = "ND_cde";
}

pub fn do_abc(line: &Commandline, _caret:&mut Caret){
    println!("ABC! [{}]", line.contents);
}

pub fn do_cde(line: &Commandline, _caret:&mut Caret){
    println!("CDE! [{}]", line.contents);
}

pub fn do_end(line: &Commandline, caret:&mut Caret){
    caret.done_line = true;
    println!("End! [{}]", line.contents);
}

pub fn do_num(_line: &Commandline, caret:&mut Caret){
    let cap = &caret.groups[0];
    println!("Number! [{}]", cap);
}

pub fn do_other(line: &Commandline, caret:&mut Caret){
    println!("Not match. line.contents=[{}], caret.starts={}", line.contents, caret.starts);
}

pub fn do_quit(line: &Commandline, caret:&mut Caret){
    println!("Quit. [{}]", line.contents);
    caret.quits = true;
}
