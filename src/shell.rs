use diagram::ResponseOption;
use diagram::*;
/// クライアント１つにつき、１つのシェルを与えます。
/// 行単位です。
///
/// コマンド例
///
/// ```
/// cls
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shell
/// cargo clippy
/// ```
use diagram_player::*;
use line_parser::*;
use std::any::Any; // https://stackoverflow.com/questions/33687447/how-to-get-a-struct-reference-from-a-boxed-trait
use std::io;

/// 不具合を取りたいときに真にする。
const VERBOSE: bool = false;

pub const NEXT_EXIT_LABEL: &str = "#next";
/// デフォルトのラベル。
pub const NEWLINE_EXIT_LABEL: &str = "#newline";
pub const ELSE_NODE_LABEL: &str = "#else";

/// コマンドライン文字列。
///
/// # Members
///
/// * `line` - コマンドライン文字列の1行全体です。
/// * `line_len` - コマンドライン文字列の1行全体の文字数です。
/// * `groups` - あれば、正規表現の結果を入れておく。
pub struct RequestStruct {
    pub line: Box<String>, // String型は長さが可変なので、固定長のBoxでラップする。
    pub line_len: usize,
    pub caret: usize,
    pub groups: Vec<String>,
}
impl RequestStruct {
    fn new(line2: Box<String>) -> RequestStruct {
        let len = line2.chars().count();
        RequestStruct {
            line: line2,
            line_len: len,
            caret: 0,
            groups: Vec::new(),
        }
    }
}
impl Request for RequestStruct {
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
    fn get_line(&self) -> &String {
        &self.line
    }
    fn get_line_len(&self) -> usize {
        self.line_len
    }
    fn get_caret(&self) -> usize {
        self.caret
    }
    fn get_groups(&self) -> &Vec<String> {
        &self.groups
    }
}

/// キャレット。本来、文字列解析のカーソル位置だが、ほかの機能も持たされている。
/// - シェルを終了するなど、シェルに対して指示することができる。
/// - また、字句解析の次のノードを指定する役割も持つ。
///
/// # Members
///
/// * `starts` - コマンドライン文字列の次のトークンの先頭位置です。
/// * `done_line` - 行の解析を中断するなら真にします。
/// * `option` - シェルに指示を出す。アプリケーション終了、ファイル再読込など。
/// * `exit_label` - 次のノード ラベルです。
pub struct ResponseStruct {
    pub caret: usize,
    pub done_line: bool,
    pub option: ResponseOption,
    pub exit_label: String,
}
impl ResponseStruct {
    fn new() -> ResponseStruct {
        ResponseStruct {
            caret: 0,
            done_line: false,
            option: ResponseOption::None,
            exit_label: "".to_string(),
        }
    }
    pub fn reset(&mut self) {
        self.set_caret(0);
        self.set_done_line(false);
        self.set_option(ResponseOption::None);
        self.forward("");
    }
}

impl Response for ResponseStruct {
    fn as_any(&self) -> &dyn Any {
        self
    }
    /// トレイトを実装している方を返すのに使う。
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
    // .rs にハードコーディングして使う。
    fn forward(&mut self, exit_label2: &'static str) {
        self.exit_label = exit_label2.to_string();
    }
    fn set_caret(&mut self, caret2: usize) {
        self.caret = caret2
    }
    fn set_done_line(&mut self, done_line2: bool) {
        self.done_line = done_line2
    }
    fn set_option(&mut self, value: ResponseOption) {
        self.option = value;
    }
}

pub type Reader<T> = fn(t: &mut T) -> String;

pub fn standard_input_reader<T>(_t: &mut T) -> String {
    let mut line_string = String::new();
    // コマンド プロンプトからの入力があるまで待機します。
    io::stdin()
        .read_line(&mut line_string)
        .expect("info Failed to read_line"); // OKでなかった場合のエラーメッセージ。

    // 末尾の 改行 を除きます。前後の空白も消えます。
    line_string.trim().parse().expect("info Failed to parse")
}

/// シェル。
///
/// # Arguments
///
/// * `vec_row` - コマンドを複数行 溜めておくバッファーです。
pub struct Shell<T: 'static> {
    diagram_player: DiagramPlayer,
    vec_row: Vec<String>,
    reader: Reader<T>,
}
impl<T> Default for Shell<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: 'static> Shell<T> {
    pub fn new() -> Shell<T> {
        Shell {
            diagram_player: DiagramPlayer::new(),
            vec_row: Vec::new(),
            reader: standard_input_reader,
        }
    }

    /// シェルの使い方より、ダイアグラムの使い方の方が簡単なので、
    /// ダイアグラムを返す。
    pub fn get_diagram_player(&self) -> &DiagramPlayer {
        &self.diagram_player
    }
    /// パーサーを使わず状態遷移したいときに使う。
    pub fn get_mut_diagram_player(&mut self) -> &mut DiagramPlayer {
        &mut self.diagram_player
    }

    /// 現在ノードのラベル。
    pub fn get_current(&self) -> String {
        self.diagram_player.get_current().to_string()
    }

    /// 現在地が遷移図の外か。
    pub fn is_out(&self) -> bool {
        self.diagram_player.is_out()
    }

    pub fn set_reader(&mut self, reader2: Reader<T>) {
        self.reader = reader2;
    }
    /// コマンドを1行も入力していなければ真を返します。
    pub fn is_empty(&self) -> bool {
        self.vec_row.len() == 0
    }
    /// コンソール入力以外の方法で、コマンド1行を追加したいときに使います。
    /// 行の末尾に改行は付けないでください。
    pub fn push_row(&mut self, row: &str) {
        self.vec_row.push(format!("{}\n", row));
    }
    /// 先頭のコマンド1行をキューから削除して返します。
    pub fn pop_row(&mut self) -> Box<String> {
        Box::new(self.vec_row.pop().unwrap())
    }

    /// コマンドラインの入力受付、および コールバック関数呼出を行います。
    /// スレッドはブロックします。
    /// 強制終了する場合は、 [Ctrl]+[C] を入力してください。
    pub fn run(&mut self, diagram: &mut DiagramEx<T>, t: &mut T) {
        loop {
            // リクエストは、キャレットを更新するのでミュータブル。
            let mut req = if self.is_empty() {
                let line_string = (self.reader)(t);
                RequestStruct::new(Box::new(line_string))
            } else {
                // バッファーの先頭行です。
                RequestStruct::new(self.pop_row())
            };

            use diagram::ResponseOption::*;
            let res: &mut dyn Response = &mut ResponseStruct::new();

            LineParser::run(&mut self.diagram_player, diagram, t, &mut req, res);

            if let Some(res_struct) = &mut res.as_mut_any().downcast_mut::<ResponseStruct>() {
                match res_struct.option {
                    None => {}
                    Quits => break, // response.quits したとき run ループを抜ける。
                    Reloads(ref file) => {
                        // ファイルからグラフのノード構成を読取。
                        diagram.get_mut_diagram().read_file(&file);
                    }
                    Saves(ref file) => {
                        // ファイルを上書き。
                        diagram.get_mut_diagram().write_file(&file);
                    }
                }
            } else {
                panic!("Downcast fail.");
            }
        }
    }

    /// 1行 処理するだけでいいとき。
    ///
    /// - Quits は無効になる。
    ///
    /// # Arguments.
    ///
    /// * 'diagram' - パースの状態遷移図。
    /// * 't' - 任意のオブジェクト。
    /// * 'line' - コマンドライン文字列。
    pub fn execute_line(&mut self, diagram: &mut DiagramEx<T>, t: &mut T, line: &str) {
        // リクエストは、キャレットを更新するのでミュータブル。
        let mut req = RequestStruct::new(Box::new(line.to_string()));

        use diagram::ResponseOption::*;
        let res: &mut dyn Response = &mut ResponseStruct::new();

        LineParser::run(&mut self.diagram_player, diagram, t, &mut req, res);

        if let Some(res_struct) = &mut res.as_mut_any().downcast_mut::<ResponseStruct>() {
            match res_struct.option {
                None => {}
                Quits => {} // ループの中ではないので無効。
                Reloads(ref file) => {
                    // ファイルからグラフのノード構成を読取。
                    diagram.get_mut_diagram().read_file(&file);
                }
                Saves(ref file) => {
                    // ファイルを上書き。
                    diagram.get_mut_diagram().write_file(&file);
                }
            }
        } else {
            panic!("Downcast fail.");
        }
    }
}
