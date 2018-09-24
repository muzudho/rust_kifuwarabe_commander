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
use kifuwarabe_shell::graph::ResponseOption;
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

const GRAPH_JSON_FILE : &str = "graph.json";
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
    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();
    // シェルの作成。
    let mut shell = Shell::new();

    // グラフの作成。
    let mut graph = Graph::new();
    // コントローラーを登録。
    graph.insert_fn("do_a", do_a);
    graph.insert_fn("do_ab", do_ab);
    graph.insert_fn("do_abc", do_abc);
    graph.insert_fn("do_cde", do_cde);
    graph.insert_fn("do_edit_save", do_edit_save);
    graph.insert_fn("do_end", do_end);
    graph.insert_fn("do_numvar", do_numvar);
    graph.insert_fn("do_quit", do_quit);
    graph.insert_fn("do_wordvar", do_wordvar);
    graph.insert_fn("do_ab_newline", do_ab_newline);
    graph.insert_fn("do_other", do_other);
    graph.insert_fn("do_reload", do_reload);

    // ファイルからグラフのノード構成を読取。
    graph.read_graph_file(&GRAPH_JSON_FILE);

    // 内容確認出力。
    {
        println!("entrance");
        for node in graph.get_entrance_vec().iter() {
            println!("  - {}", node);
        }

        println!("nodes");
        for (node_label, node) in graph.get_node_map().iter() {
            println!("  - {} {}", node_label, node.get_token());
            for (exits_label, exits_vec) in node.get_exits_map().iter() {
                println!("    - {}", exits_label);
                for exits_item in exits_vec.iter() {
                    println!("      - {}", exits_item);
                }
            }
        }
    }


    // 実行。
    println!("Please enter command.");
    shell.run(&mut graph, &mut shell_var);
    println!("Finished. shell_var.count: {}.", shell_var.count);
}

pub fn do_a(
    shell_var: &mut ShellVar,
    _req: &dyn Request,
    _res: &mut dyn Response,
) {
    shell_var.count += 1;
    println!("A.");
}

pub fn do_ab(
    shell_var: &mut ShellVar,
    _req: &dyn Request,
    res: &mut dyn Response,
) {
    shell_var.count += 1;
    println!("Ab.");
    res.forward("next");
}

pub fn do_ab_newline(
    shell_var: &mut ShellVar,
    _req: &dyn Request,
    _res: &mut dyn Response,
) {
    shell_var.count += 1;
    println!("Ab-NewLine.");
}

pub fn do_abc(
    shell_var: &mut ShellVar,
    _req: &dyn Request,
    _res: &mut dyn Response,
) {
    shell_var.count += 1;
    println!("Abc.");
}

pub fn do_cde(
    shell_var: &mut ShellVar,
    _req: &dyn Request,
    res: &mut dyn Response,
) {
    shell_var.count += 1;
    println!("Cde.");
    res.forward("next");
}

/// グラフファイルを上書き保存する。
pub fn do_edit_save(
    _shell_var: &mut ShellVar,
    _req: &dyn Request,
    res: &mut dyn Response,
) {
    println!("!Save. {}", GRAPH_JSON_FILE);
    res.set_option(ResponseOption::Saves(GRAPH_JSON_FILE.to_string()));
}

pub fn do_end(
    shell_var: &mut ShellVar,
    _req: &dyn Request,
    res: &mut dyn Response,
) {
    shell_var.count += 1;
    res.set_done_line(true);
    println!("End.");
}

pub fn do_numvar(
    shell_var: &mut ShellVar,
    req: &dyn Request,
    _res: &mut dyn Response,
) {
    shell_var.count += 1;
    let cap = &req.get_groups()[0];
    println!("Number({}).", cap);
}

pub fn do_other(
    shell_var: &mut ShellVar,
    req: &dyn Request,
    _res: &mut dyn Response,
) {
    shell_var.count += 1;
    println!(
        "Not match. req.line=[{}], req.caret={}",
        req.get_line(),
        req.get_caret()
    );
}

pub fn do_quit(
    shell_var: &mut ShellVar,
    _req: &dyn Request,
    res: &mut dyn Response,
) {
    shell_var.count += 1;
    println!("Quit.");
    res.set_option(ResponseOption::Quits);
}

pub fn do_reload(
    _shell_var: &mut ShellVar,
    _req: &dyn Request,
    res: &mut dyn Response,
) {
    println!("Reload. {}", GRAPH_JSON_FILE);
    res.set_option(ResponseOption::Reloads(GRAPH_JSON_FILE.to_string()));
}

pub fn do_wordvar(
    shell_var: &mut ShellVar,
    req: &dyn Request,
    _res: &mut dyn Response,
) {
    shell_var.count += 1;
    let cap = &req.get_groups()[0];
    println!("Word({}).", cap);
}
