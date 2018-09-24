# kifuwarabe_shell

## Overview.

コマンドラインのパーサーのフレームワークだぜ☆（＾～＾）  

理屈でいうと、  

```
abc def ghi
```

というコマンドがあるとき、 graph.json という設定ファイルに  

```
### 省略した書き方
{
    "token": "abc",
    "fn": "do_abc"
},
{
    "token": "def",
    "fn": "do_def"
},
{
    "token": "ghi",
    "fn": "do_ghi"
},
```

といった風に書いておけば、コールバック関数 do_abc(), do_def(), do_ghi() とかが呼ばれる仕組み。  
実際は JSONファイルの中身は ごつく なる。  
コールバック関数は あらかじめ登録しておく☆（＾～＾）  
詳しくは graph.json、 examples/main.rs を読めだぜ☆（＾～＾）  

## Cargo.toml の例。

```
[dependencies.kifuwarabe_shell]
git = "https://github.com/muzudho/rust_kifuwarabe_shell.git"
rev = "7462977... Please get new rev from git hub."
```

rev は Git hub を見て新しいのを入れろだぜ☆（＾～＾）

## ファイルの冒頭の例。

```
extern crate serde_json;
extern crate kifuwarabe_shell;
use kifuwarabe_shell::graph::*;
use kifuwarabe_shell::graph::ResponseOption;
use kifuwarabe_shell::shell::*;
```

## graph.json のファイル名。

```
const GRAPH_JSON_FILE : &'static str = "graph.json";
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
    // グラフ作成し、コントローラー登録。
    let mut graph = Graph::new();
    graph.insert_fn("do_abc", do_abc);
    graph.insert_fn("do_def", do_def);
    graph.insert_fn("do_ghi", do_ghi);

    // ファイル読取。
    graph.read_graph_file(GRAPH_JSON_FILE.to_string());

    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();

    // 実行。グラフと 任意のオブジェクトを渡す。
    let mut shell = Shell::new();
    println!("Please enter command.");
    shell.run(&mut graph, &mut shell_var);
}
```

main 関数はこんなもん。 run の中で標準入力を勝手に拾う。
標準入出力じゃなくてもいい。詳しくはソース読め。

コールバック関数は こんなふうに書くぜ☆（＾～＾）

```
pub fn fn_abc(
    shell_var: &mut ShellVar,
    _req: &Request,
    res: &mut dyn Response,
) {
    shell_var.count += 1;
    println!("I am abc!");
    res.forward("next");
}

pub fn fn_def(
    shell_var: &mut ShellVar,
    _req: &Request,
    res: &mut dyn Response,
) {
    shell_var.count += 1;
    println!("I am def!");
    res.forward("next");
}

pub fn fn_ghi(
    shell_var: &mut ShellVar,
    _req: &Request,
    res: &mut dyn Response,
) {
    shell_var.count += 1;
    println!("I am ghi!");
    res.forward("next");
}
```

request とか、 response とか、 forward というのは Webサーバーのフレームワークを真似ている☆（＾～＾）
じゃあ次は graph.json の書き方だぜ。

## graph.json の書き方。

```
{
	"entrance": [
		"ND.a"
	],
	"nodes" : [
		{
			"label": "ND.a",
			"token": "abc",
			"fn": "do_abc",
			"exit": {
				"next": [
					"ND.b"
				]
			}
		},
		{
			"label": "ND.b",
			"token": "def",
			"fn": "do_def",
			"exit": {
				"next": [
					"ND.c"
				]
			}
		},
		{
			"label": "ND.c",
			"token": "ghi",
			"fn": "do_ghi"
		}
    ]
}
```

ここで ```ND.a``` みたいなやつは ノードの名前 ぐらいの意味でなんでもいい。ただの Go to 用のラベルだぜ。
```entrance``` というのは コマンドラインの行頭 ぐらいの意味だぜ。複数書けばマッチしたやつが選ばれる。

```token```, ```regex```, 無記入が選べ、例えば

```
### abc にマッチする。
"token": "abc"

### 123 とかにマッチする。正規表現はたいして使えず、全体を丸かっこで囲んで１トークンとする必要がある。
"regex": "(\\d+)"

### token と regex のどちらも無記入の場合は特殊な使い方をする。
```

```fn``` というのは ```graph.insert_fn("名前", 関数名);``` で登録したやつだ。

```exit``` は少し複雑だ。

```
"exit": {
    "next": [
        "ND.b"
    ],
    "jump": [
        "ND.x",
        "ND.y",
        "ND.z"
    ],
    "kick": [
        "ND.w"
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
        res.forward("next");
    }
```

このように飛び先を変えることができる。
ノードの名前を書くのではなく、 ```exit``` オブジェクトのキー名を書けだぜ。
こうすることで graph.json で遷移図が できあがるようにしている。

## 特殊なケース: 改行

改行 をうまく拾えなかったので ```#newline``` という組込みラベル を用意した。

例えば、

```
jikan 500
jikan 500 byoyomi 100
jikan 500 byoyomi 100 black
```

のような３つのコマンドがあって、いずれも改行で ```ND.newline``` ノードに飛んで欲しいとする。
そんなときは

```
    "token": "jikan",
    "exit": {
        "next": [
            "ND.byoyomi"
        ],
        "#newline": [
            "ND.newline"
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

## グラフとシェルの関係は、音楽データと音楽プレイヤーの関係。

response.set_option を使って、シェルに指示を出すことができる。１度に１つだけ。

```
### シェル終われ。
res.set_option(ResponseOption::Quits);

### graph.json ファイルを読み込み直せ。
res.set_option(ResponseOption::Reloads(GRAPH_JSON_FILE.to_string()));

### graph.json ファイルを保存しろ。
res.set_option(ResponseOption::Saves(GRAPH_JSON_FILE.to_string()));
```

graph.json ファイルを編集するツールは、 rust_kifuwarabe_shell_visualizer として作成中だぜ☆（＾ｑ＾）

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
