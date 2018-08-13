# kifuwarabe_shell

## Examples.

```
### 以下のコマンドでサンプルを実行。 
cargo run --example main
```


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
use kifuwarabe_shell::*;

fn main() {
    
    let mut shell = Shell::new();
    // 追加順に先頭一致検索
    shell.insert_node("ND_isready", "isready", do_isready);
    shell.insert_node("ND_position", "position", do_position);
    shell.insert_node("ND_quit", "quit", do_quit);
    shell.insert_node("ND_usinewgame", "usinewgame", do_usinewgame);
    shell.insert_node("ND_usi", "usi", do_usi);
    
    // 該当なしの場合のコールバック関数を登録する。
    shell.set_complementary_controller(do_other);

    // 開始ノードを選択する。
    shell.set_next("ND_isready, ND_position,
        ND_quit, ND_usinewgame, ND_usi");

    // [Ctrl]+[C] で強制終了
    shell.run();
}
```

コールバック関数は こんなふうに書くぜ☆（＾～＾）

```
/// USIプロトコル参照。
pub fn do_usi(_request: &Request, _response:&mut Response) {
    // 省略
}
```

## Instalation.

Example.

Cargo.toml

```
[dependencies.kifuwarabe_shell]
git = "https://github.com/muzudho/rust_kifuwarabe_shell.git"
rev = "6deac338e5ad49992f2f7bfe94c9415bf8382a26"
```

```
C:\Users\Muzudho\example>cargo build
```

## Reference implementation.

[Kifuwarabe_Shogi2018](https://github.com/muzudho/Kifuwarabe_Shogi2018)
