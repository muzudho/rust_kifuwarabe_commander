#![allow(unused_variables)]

use std::io;

type Callback = fn(len:usize, line: &String, starts:&mut usize, quits:&mut bool);

/// [2016-12-10 Idiomatic callbacks in Rust](https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust)
pub struct Command {
    pub keyword: String,
    pub callback: Callback,
}
impl Command {

    pub fn is_matched(&self, len:usize, line: &String, starts:&usize) -> bool {
        return self.keyword.len()<=len && &line[*starts..self.keyword.len()] == self.keyword
    }

    pub fn move_caret_and_go(&self, len:usize, line: &String, starts:&mut usize, quits:&mut bool) {
        *starts += self.keyword.len();
        // 続きにスペース「 」が１つあれば読み飛ばす
        if 0<(len-*starts) && &line[*starts..(*starts+1)]==" " {
            *starts+=1;
        }            

        (self.callback)(len, line, starts, quits);
    }
}

pub fn none_callback(len:usize, line: &String, starts:&mut usize, quits:&mut bool){

}

pub struct Commander{
    // コマンドを溜めておくバッファー
    pub vec_line : Vec<String>,
    pub action_len_zero: Command,
    pub command_array: Vec<Command>,
}
impl Commander {
    pub fn new()->Commander{
        Commander{
            vec_line : Vec::new(),
            action_len_zero: Command { keyword: "".to_string(), callback: none_callback },
            command_array: Vec::new(),
        }
    }
    pub fn is_empty_command(&mut self) -> bool {
        self.vec_line.len()==0
    }
    pub fn push_command(&mut self, line:&String) {
        self.vec_line.push( format!("{}\n", line ) );
    }
    pub fn pop_command(&mut self) -> String {
        self.vec_line.pop().unwrap()
    }

    pub fn run(&mut self) {
        // [Ctrl]+[C] で強制終了
        loop{

            let mut line : String;
            if self.is_empty_command() {
                line = String::new();
            } else {
                // バッファーに溜まっていれば☆（＾～＾）
                line = self.pop_command();
            }

            // まず最初に、コマンドライン入力を待機しろだぜ☆（＾～＾）
            io::stdin().read_line(&mut line)
                .ok()// read_lineの返り値オブジェクトResult の okメソッド
                .expect("info Failed to read line");// OKで無かった場合のエラーメッセージ

            // 末尾の改行を除こうぜ☆（＾～＾）
            // trim すると空白も消えるぜ☆（＾～＾）
            let line : String = line.trim().parse().ok().expect("info Failed to parse");

            // 文字数を調べようぜ☆（＾～＾）
            let len = line.chars().count();
            let mut starts = 0;

            let mut is_done = false;
            // アプリケーション終了
            let mut quits = false;

            for element in self.command_array.iter() {
                if element.is_matched(len, &line, &starts) {
                    element.move_caret_and_go(len, &line, &mut starts, &mut quits);
                    is_done = true;
                    break;
                }
            }

            // 何とも一致しなかったら実行する。
            if !is_done {
                self.action_len_zero.move_caret_and_go(len, &line, &mut starts, &mut quits);
            }

            if quits {
                // ループを抜けて終了
                break;
            }
        }//loop
    }
}


