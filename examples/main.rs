/// ```
/// ### 以下のコマンドで実行。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shell
/// cargo run --example main
/// ```
// #[macro_use(hashmap)]
extern crate kifuwarabe_shell;

use kifuwarabe_shell::graph::*;
use kifuwarabe_shell::shell::*;
use std::collections::HashMap;

// 参考:
// https://github.com/serde-rs/json
extern crate serde_json;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

// 任意のオブジェクト。
pub struct ShellVar {
    pub count: i32,
}
impl ShellVar {
    fn new() -> ShellVar {
        ShellVar { count: 0 }
    }
}

/// JSON配列を、文字列の配列に変換。
///
/// # Arguments.
///
/// * 'v' - Json array.
/// * 'str_vec' - let str_vec = Vec::new();
fn array_to_str_vec(v: &Value, str_vec: &mut Vec<String>) {
    let value_vec: Vec<Value> = v.as_array().unwrap().to_vec();
    for node_name in value_vec {
        str_vec.push(node_name.as_str().unwrap().to_string());
    }
}
/// JSONオブジェクトを、文字列のハッシュマップに変換。
///
/// # Arguments.
///
/// * 'v' - Json object.
/// * 'str_vec' - let str_vec = Vec::new();
fn object_to_map(obj: &Value, map0: &mut HashMap<String, Vec<String>>) {
    println!("Parse object: begin.");
    if !obj.is_null() {
        for (name1,array1) in obj.as_object().unwrap().iter() {
            println!("  Array: begin.");
            let mut array2: Vec<String> = Vec::new();
            for item1 in array1.as_array().unwrap().iter() {
                println!("    Item: begin.");
                array2.push(item1.as_str().unwrap().to_string());
                println!("    Item: end.");
            }
            map0.insert(name1.to_string(), array2);
            println!("  Array: end.");
        }
    }
    println!("Parse object: end.");
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
    // グラフの作成。
    let mut graph = Graph::new();
    // コントローラーを登録。
    graph.insert_controller("do_a", do_a);
    graph.insert_controller("do_ab", do_ab);
    graph.insert_controller("do_abc", do_abc);
    graph.insert_controller("do_cde", do_cde);
    graph.insert_controller("do_end", do_end);
    graph.insert_controller("do_numvar", do_numvar);
    graph.insert_controller("do_quit", do_quit);
    graph.insert_controller("do_wordvar", do_wordvar);
    graph.insert_controller("do_ab_linebreak", do_ab_linebreak);
    graph.insert_controller("do_other", do_other);

    // グラフのノード構成。
    {
        println!("Test json: begin.");
        let mut file = File::open("graph.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        // https://docs.serde.rs/serde_json/value/enum.Value.html
        let v: Value = serde_json::from_str(&data).unwrap();

        // 文字列に変換する。
        println!("Parse entrance: begin.");
        let mut entrance_vec : Vec<String> = Vec::new();
        array_to_str_vec(&v["entrance"], &mut entrance_vec);
        println!("Parse entrance: parsed.");
        graph.set_entrance(entrance_vec);
        println!("Parse entrance: end.");

        for node in v["nodes"].as_array().unwrap().iter() {
            /* デバッグ出力。
            println!("  name: {}", node["name"]);
            println!("  token: {}", node["token"]);
            println!("  regex: {}", node["regex"]);
            println!("  controller: {}", node["controller"]);
            if !node["next"].is_null() {
                // println!("  next: {}", node["next"]);
                for (next_key, next_value) in node["next"].as_object().unwrap().iter() {
                    println!("    next: {}, {}", next_key, next_value);
                }
            }
            */

            if !node["token"].is_null() {
                let mut entrance_map : HashMap<String, Vec<String>> = HashMap::new();
                if !node["exit"].is_null() {
                    object_to_map(&v["exit"], &mut entrance_map);
                }
                graph.insert_node(
                    node["name"].as_str().unwrap().to_string(),
                    node["token"].as_str().unwrap().to_string(),
                    node["controller"].as_str().unwrap().to_string(),
                    entrance_map,
                );
            } else if !node["regex"].is_null() {
                let mut entrance_map : HashMap<String, Vec<String>> = HashMap::new();
                if !node["exit"].is_null() {
                    object_to_map(&v["exit"], &mut entrance_map);
                    // for (exits_key, exits_node_names) in node["exit"].as_object().unwrap().iter() {
                    //     // 変換する。
                    //     entrance_map = object_to_map(v["exit"], &HashMap::new());
                    // }
                }
                graph.insert_node_reg(
                    node["name"].as_str().unwrap().to_string(),
                    node["regex"].as_str().unwrap().to_string(),
                    node["controller"].as_str().unwrap().to_string(),
                    entrance_map,
                );
            } else {
                graph.insert_node_single(
                    node["name"].as_str().unwrap().to_string(),
                    node["controller"].as_str().unwrap().to_string(),
                );
            }
        }
        // println!("Test json: end.");
    }
    /*
    graph.insert_node("ND_a".to_string(), "a".to_string(), "do_a".to_string(), hashmap![]);
    graph.insert_node(
        "ND_ab".to_string(),
        "ab".to_string(),
        "do_ab".to_string(),
        hashmap!["next".to_string() => "ND_cde".to_string(), "#linebreak".to_string() => "ND_ab_linebreak".to_string()],
    ); // #linebreak コールバック関数は行終了時に実行される。
    graph.insert_node("ND_abc".to_string(), "abc".to_string(), "do_abc".to_string(), hashmap![]);
    graph.insert_node("ND_cde".to_string(), "cde".to_string(), "do_cde".to_string(), hashmap!["next".to_string() => "ND_wordvar".to_string()]);
    graph.insert_node("ND_end".to_string(), "end".to_string(), "do_end".to_string(), hashmap![]);
    graph.insert_node_reg("ND_numvar".to_string(), r"(\d+)".to_string(), "do_numvar".to_string(), hashmap![]);
    graph.insert_node("ND_quit".to_string(), "quit".to_string(), "do_quit".to_string(), hashmap![]);
    graph.insert_node_reg("ND_wordvar".to_string(), r"(\w+)".to_string(), "do_wordvar".to_string(), hashmap![]);
    graph.insert_node_single("ND_ab_linebreak".to_string(), "do_ab_linebreak".to_string());
    graph.insert_node_single("#ND_complementary".to_string(), "do_other".to_string()); // 該当なしの場合のコールバック関数を登録する。
                                                             // 正規表現は、うまく作れていない。全体を丸括弧で囲む。1個だけ。
                                                             // 開始ノードを選択する。
    graph.set_entrance(
        "ND_a,ND_ab,ND_abc,ND_end,ND_numvar,
        ND_quit,ND_wordvar".to_string(),
    );
    */

    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();
    // シェルの作成。
    let mut shell = Shell::new();

    // 実行。
    println!("Please enter command.");
    shell.run(&mut graph, &mut shell_var);

    println!("shell_var.count: {}", shell_var.count);
}

pub fn do_a(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("A.");
}

pub fn do_ab(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("Ab.");
    response.forward("next".to_string());
}

pub fn do_ab_linebreak(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("Ab-LineBreak.");
}

pub fn do_abc(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("Abc.");
}

pub fn do_cde(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("Cde.");
    response.forward("next".to_string());
}

pub fn do_end(
    shell_var: &mut ShellVar,
    _request: &RequestAccessor,
    response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    response.set_done_line(true);
    println!("End.");
}

pub fn do_numvar(
    shell_var: &mut ShellVar,
    request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    let cap = &request.get_groups()[0];
    println!("Number({}).", cap);
}

pub fn do_other(
    shell_var: &mut ShellVar,
    request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
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
    _request: &RequestAccessor,
    response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    println!("Quit.");
    response.set_quits(true);
}

pub fn do_wordvar(
    shell_var: &mut ShellVar,
    request: &RequestAccessor,
    _response: &mut dyn ResponseAccessor,
) {
    shell_var.count += 1;
    let cap = &request.get_groups()[0];
    println!("Word({}).", cap);
}
