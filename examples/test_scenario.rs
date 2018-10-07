use kifuwarabe_shell::diagram::ResponseOption;
use kifuwarabe_shell::diagram::{Diagram, Request, Response};

const DIAGRAM_JSON_FILE: &str = "diagram.json";

// 任意のオブジェクト。
pub struct ShellVar {
    pub count: i32,
}
impl ShellVar {
    pub fn new() -> ShellVar {
        ShellVar { count: 0 }
    }
}

pub fn setup_diagram(diagram: &mut Diagram<ShellVar>) {
    // コントローラーを登録。
    diagram.insert_fn("do_a", do_a);
    diagram.insert_fn("do_ab", do_ab);
    diagram.insert_fn("do_abc", do_abc);
    diagram.insert_fn("do_cde", do_cde);
    diagram.insert_fn("do_edit_save", do_edit_save);
    diagram.insert_fn("do_end", do_end);
    diagram.insert_fn("do_numvar", do_numvar);
    diagram.insert_fn("do_quit", do_quit);
    diagram.insert_fn("do_wordvar", do_wordvar);
    diagram.insert_fn("do_ab_newline", do_ab_newline);
    diagram.insert_fn("do_other", do_other);
    diagram.insert_fn("do_reload", do_reload);

    // ファイルからグラフのノード構成を読取。
    diagram.read_file(&DIAGRAM_JSON_FILE);
}

pub fn do_a(shell_var: &mut ShellVar, _req: &dyn Request, _res: &mut dyn Response) {
    shell_var.count += 1;
    println!("A.");
}

pub fn do_ab(shell_var: &mut ShellVar, _req: &dyn Request, res: &mut dyn Response) {
    shell_var.count += 1;
    println!("Ab.");
    // res.forward("#next"); デフォルトなんで書かなくてもいい。
}

pub fn do_ab_newline(shell_var: &mut ShellVar, _req: &dyn Request, _res: &mut dyn Response) {
    shell_var.count += 1;
    println!("Ab-NewLine.");
}

pub fn do_abc(shell_var: &mut ShellVar, _req: &dyn Request, _res: &mut dyn Response) {
    shell_var.count += 1;
    println!("Abc.");
}

pub fn do_cde(shell_var: &mut ShellVar, _req: &dyn Request, res: &mut dyn Response) {
    shell_var.count += 1;
    println!("Cde.");
    // res.forward("#next"); デフォルトなんで書かなくてもいい。
}

/// グラフファイルを上書き保存する。
pub fn do_edit_save(_shell_var: &mut ShellVar, _req: &dyn Request, res: &mut dyn Response) {
    println!("!Save. {}", DIAGRAM_JSON_FILE);
    res.set_option(ResponseOption::Saves(DIAGRAM_JSON_FILE.to_string()));
}

pub fn do_end(shell_var: &mut ShellVar, _req: &dyn Request, res: &mut dyn Response) {
    shell_var.count += 1;
    res.set_done_line(true);
    println!("End.");
}

pub fn do_numvar(shell_var: &mut ShellVar, req: &dyn Request, _res: &mut dyn Response) {
    shell_var.count += 1;
    let cap = &req.get_groups()[0];
    println!("Number({}).", cap);
}

pub fn do_other(shell_var: &mut ShellVar, req: &dyn Request, _res: &mut dyn Response) {
    shell_var.count += 1;
    println!(
        "do_other: Not match. req.line=[{}], req.caret={}",
        req.get_line(),
        req.get_caret()
    );
}

pub fn do_quit(shell_var: &mut ShellVar, _req: &dyn Request, res: &mut dyn Response) {
    shell_var.count += 1;
    println!("Quit.");
    res.set_option(ResponseOption::Quits);
}

pub fn do_reload(_shell_var: &mut ShellVar, _req: &dyn Request, res: &mut dyn Response) {
    println!("Reload. {}", DIAGRAM_JSON_FILE);
    res.set_option(ResponseOption::Reloads(DIAGRAM_JSON_FILE.to_string()));
}

pub fn do_wordvar(shell_var: &mut ShellVar, req: &dyn Request, _res: &mut dyn Response) {
    shell_var.count += 1;
    let cap = &req.get_groups()[0];
    println!("Word({}).", cap);
}
