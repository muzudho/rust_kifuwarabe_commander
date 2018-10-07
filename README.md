# What is kifuwarabe shell?

## Overview.

プロトコルや、コマンドライン　のパーサーのフレームワークだぜ☆（＾～＾）  

理屈でいうと、  

```
abc 123 def
```

というコマンドがあるとき、

```
pub fn do_abc() {

}

pub fn do_num() {

}

pub fn do_def() {

}
```

という関数を呼んでくれたら楽だろ。このライブラリは それをやってくれる。  
diagram.json という設定ファイルに  

```
### これは要所を抜粋して見せてるだけ
{
    "token": "abc",
    "fn": "do_abc"
},
{
    "regex": "(\\d+)",
    "fn": "do_num"
},
{
    "token": "def",
    "fn": "do_def"
},
```

といった風に書いておけば、コールバック関数 do_abc(), do_num(), do_def() とかが呼ばれる仕組み。  
実際は コールバック関数の引数や、 JSONファイルの中身は もっと ごつく なる。  
コールバック関数は あらかじめ登録しておく☆（＾～＾）  
詳しくは diagram.json、 examples/main.rs を読めだぜ☆（＾～＾）  

# Instalation.

## Cargo.toml の例。

```
[dependencies]
serde_json = "1.0"

[dependencies.kifuwarabe_shell]
git = "https://github.com/muzudho/rust_kifuwarabe_shell.git"
rev = "7462977... Please get new rev from git hub."
```

rev は Git hub を見て新しいのを入れろだぜ☆（＾～＾）

# How to use kifuwarabe shell?

## ファイルの冒頭の例。

```
extern crate serde_json;
extern crate kifuwarabe_shell;
use kifuwarabe_shell::diagram::*;
use kifuwarabe_shell::diagram::ResponseOption;
use kifuwarabe_shell::shell::*;
```

## diagram.json のファイル名。

```
const DIAGRAM_JSON_FILE : &'static str = "diagram.json";
```

なんでもいい。定数にしておけだぜ。

## 任意の struct を1個持てる。

```
// 任意のオブジェクト。
pub struct ShellVar {
    pub count: i32,
}
impl ShellVar {
    fn new() -> ShellVar {
        ShellVar { count: 0 }
    }
}
```

コールバック関数の引数として渡される。１個作っておけだぜ。

```
fn main() {
    // グラフ作成し、コントローラー関数の登録。
    let mut diagram = Diagram::new();
    diagram.insert_fn("do_abc", do_abc);
    diagram.insert_fn("do_num", do_num);
    diagram.insert_fn("do_def", do_def);

    // ファイル読取。
    diagram.read_file(DIAGRAM_JSON_FILE.to_string());

    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();

    let mut shell = Shell::new();
    println!("Please enter command.");

    // 実行。グラフと 任意のオブジェクトを渡す。
    shell.run(&mut diagram, &mut shell_var);

    // 一行だけ実行するだけでいいなら、こっち
    // shell.execute_line(&mut diagram, &mut shell_var, "abc 123 def");
}
```

main 関数はこんなもん。 run の中で標準入力を勝手に拾う。
標準入出力じゃなくてもいい。詳しくはソース読め。

コールバック関数は こんなふうに書くぜ☆（＾～＾）

```
pub fn do_abc(
    shell_var: &mut ShellVar,
    _req: &Request,
    res: &mut dyn Response,
) {
    shell_var.count += 1;
    println!("I am abc!");
    // res.forward("#next"); デフォルトなんで書かなくてもいい。
}

pub fn do_num(
    shell_var: &mut ShellVar,
    req: &Request,
    res: &mut dyn Response,
) {
    // 正規表現は () 1個で全体を囲んだグループ1個 のものにだけ対応。
    let num = req.get_groups()[0];
    println!("I am {}!", num);
    // res.forward("#next"); デフォルトなんで書かなくてもいい。
}

pub fn do_def(
    _shell_var: &mut ShellVar,
    _req: &Request,
    res: &mut dyn Response,
) {
    println!("I am def!");
    // res.forward("#next"); デフォルトなんで書かなくてもいい。
}
```

request とか、 response とか、 forward というのは Webサーバーのフレームワークを真似ている☆（＾～＾）
じゃあ次は diagram.json の書き方だぜ。

## diagram.json の書き方。

```
{
    "entry_point": "HEAD.neutral",
    "nodes" : [
        {
            "label": "HEAD.neutral",
            "exit": {
                "#next": [
                    "TK.a",
                    "TK.c"
                ]
            }
        },
        {
            "label": "TK.a",
            "token": "abc",
            "fn": "do_abc",
            "exit": {
                "#next": [
                    "TK.b"
                ]
            }
        },
        {
            "label": "TK.b",
            "regex": "(\\d+)",
            "fn": "do_num",
            "exit": {
                "#next": [
                    "TK.c"
                ]
            }
        },
        {
            "label": "TK.c",
            "token": "def",
            "fn": "do_def"
            "exit": {
                "#next": [
                    "HEAD.neutral"
                ]
            }
        }
    ]
}
```

```entry_point``` というのは 入り口で 1個だけ 「ノードのラベル」というものを指定している。
ノードというのは　迷路の中にある 部屋 ぐらいに思えだぜ。
ここで ```HEAD.neutral``` とか ```TK.a``` みたいな記号が 「ノードのラベル」だが、
初学者用に目印っぽく書いただけなんで 特に書き方に決まりはない。

じゃあ 2行目の エントリーポイントに ```HEAD.neutral``` と書いてるんで、 5行目ぐらいを見ろだぜ。
今は コマンドラインの行頭 にカーソルがあると思えだぜ。ヘッドな。
その下の ```exit``` の下あたりに ```#next``` というのがあるが、これは 次はどのノードに行くかな、というのが書いている。

ネクストには また ```TK.a``` とか ```TK.c``` とか 「ノードのラベル」が複数個書いてあるが、
「文字列のマッチング」で マッチしたやつ が選ばれる。  

じゃあ次。

ラベルの下には、 (1) ```token```, (2) ```regex```, (3) 無記入　が選べるぜ。
これが 「文字列のマッチング」になっている。例えば

```
### abc にマッチする。
"token": "abc"

### 123 とかにマッチする。正規表現はたいして使えず、全体を丸かっこで囲んで１トークンとする必要がある。
"regex": "(\\d+)"

### token と regex のどちらも無記入の場合は特殊な使い方をする。
```

```fn``` というのは ```diagram.insert_fn("名前", 関数名);``` で登録したやつだ。

```exit``` は少し複雑だ。

```
"exit": {
    "#next": [
        "TK.b"
    ],
    "jump": [
        "TK.x",
        "TK.y",
        "TK.z"
    ],
    "kick": [
        "TK.w"
    ]
}
```

上のように書いたら、コールバック関数では

```
    if a == 1 {
        res.forward("jump");
    } else if a == 2 {
        res.forward("kick");
    } else {
        // res.forward("#next"); デフォルトなんで書かなくてもいい。
    }
```

このように飛び先を変えることができる。
ノードの名前を書くのではなく、 ```exit``` オブジェクトのキー名を書けだぜ。
こうすることで diagram.json で遷移図が できあがるようにしている。

特別な意味を持ったラベルは ```#next``` のように頭に ```#``` が付いている。
自分で ラベル の名前を作るときは頭に ```#``` を付けるなだぜ。それを守れば 任意だぜ。

## 特殊なケース: 改行

改行 をうまく拾えなかったので ```#newline``` という組込みラベル を用意した。

例えば、

```
jikan 500
jikan 500 byoyomi 100
jikan 500 byoyomi 100 black
```

のような３つのコマンドがあって、いずれも改行で ```TK.newline``` ノードに飛んで欲しいとする。
そんなときは

```
    "token": "jikan",
    "exit": {
        "#next": [
            "TK.byoyomi"
        ],
        "#newline": [
            "TK.newline"
        ]
    }
```

8文字で長いが ```#newline``` を書けだぜ。
で、いちいち 改行していい トークン全部に ```#newline``` 付けるの嫌なんで、
改行するか ```#newline``` を再設定するまで 以降のトークンにこの設定は有効。
どこで改行したか分からないが、分からなくていい作りにしろだぜ。

## 特殊なケース: なにとも一致しなかったとき。

とりあえず ```#else``` という組込みノード名 を用意した。

```
        {
            "name": "#else",
            "fn": "do_other"
        },
```

コントローラーを１個対応させることができる。  
一致するトークンが無かった時点で、行のそこから後ろは パースされず、次の行の先頭に移る。  

- 「それ以外なら」の意味で使うなら、exitのラベルとして ```#next``` の方を使う。デフォルト値なので「それ以外なら」のケースに該当する。
- ```#else``` は、想定していない入力をキャッチして異常終了するときに使うことになると思う。

## グラフとシェルの関係は、音楽データと音楽プレイヤーの関係。

response.set_option を使って、シェルに指示を出すことができる。１度に１つだけ。

```
### シェル終われ。
res.set_option(ResponseOption::Quits);

### diagram.json ファイルを読み込み直せ。
res.set_option(ResponseOption::Reloads(DIAGRAM_JSON_FILE.to_string()));

### diagram.json ファイルを保存しろ。
res.set_option(ResponseOption::Saves(DIAGRAM_JSON_FILE.to_string()));
```

diagram.json ファイルを編集するツールは、 rust_kifuwarabe_shell_visualizer として作成中だぜ☆（＾ｑ＾）

# その他

## Examples.

```
### 以下のコマンドでサンプルを実行。 
cargo run --example main
```

## Reference implementation.

実際使っている例は きふわらべ のソースを読めだぜ☆（＾～＾）

[Kifuwarabe_Shogi2018](https://github.com/muzudho/Kifuwarabe_Shogi2018)


## Visualizer

CUI だが、グラフを可視化するツールも作成中だぜ☆（*＾～＾*）

https://github.com/muzudho/rust_kifuwarabe_shell_visualizer
