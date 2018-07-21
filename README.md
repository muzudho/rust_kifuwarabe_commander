# kifuwarabe_shell

## Overview.

例えば

```
C:\Users\Muzudho>usi
```

とコマンドを打ち込めば、 ```usi``` という文字列に対応した do_usi(...) コールバック関数を  
呼び出してくれるフレームワークだぜ☆（＾～＾）  
コールバック関数は あらかじめ登録しておく☆（＾～＾）  

```
extern crate kifuwarabe_shell;
use kifuwarabe_shell::TokenMapping;
use kifuwarabe_shell::Shell;

fn main() {
    
    let mut shell = Shell::new();
    // 追加順に先頭一致検索
    shell.push_token_mapping(TokenMapping { token: "isready".to_string(), callback: do_isready});
    shell.push_token_mapping(TokenMapping { token: "position".to_string(), callback: do_position});
    shell.push_token_mapping(TokenMapping { token: "quit".to_string(), callback: do_quit});
    shell.push_token_mapping(TokenMapping { token: "usinewgame".to_string(), callback: do_usinewgame});
    shell.push_token_mapping(TokenMapping { token: "usi".to_string(), callback: do_usi});    
    shell.set_other_callback(do_other);

    // [Ctrl]+[C] で強制終了
    shell.run();
}
```

コールバック関数は こんなふうに書くぜ☆（＾～＾）

```
use kifuwarabe_shell::Response;

/// USIプロトコル参照。
pub fn do_usi(_row: &String, _starts:&mut usize, _res:&mut Response) {
    // 省略
}
```

## Instalation.

Example.

Cargo.toml

```
[dependencies.kifuwarabe_shell]
git = "https://github.com/muzudho/rust_kifuwarabe_shell.git"
rev = "fb4e862195e29a60cfc3d3a9bc2f98db6586acf6"
```

```
C:\Users\Muzudho\example>cargo build
```

## Example.

[Kifuwarabe_Shogi2018](https://github.com/muzudho/Kifuwarabe_Shogi2018)
