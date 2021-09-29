use std::env;
use std::error;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
	pub entry: String,
	pub dist: String,
}

// 組み込みのエラーはいろいろ存在していて、1関数内に複数エラーの型が存在していると
// 返り値の型をどうしていいのかわからなくなる。
// これの対応として返り値はErrorトレイトを実装している型っていうふうに表現することができる。
// が、Rustは返り値の型がトレイトだとHeapに値を保存するしかないので、Box化してheapに保存することを明示するdynをつけるんだって
// https://doc.rust-jp.rs/rust-by-example-ja/trait/dyn.html
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// 今回のエラー処理だと、出力されるエラー情報がうすいので、どっかでこちらのパッケージを利用した方法を参考にさせていただこう。
// https://cha-shu00.hatenablog.com/entry/2020/12/08/060000#f-243e672f
// 尚、序盤の独自エラー型は、公式exampleに掲載されてる
// https://doc.rust-jp.rs/rust-by-example-ja/error/multiple_error_types/wrap_error.html
// 組み込みErrorを独自エラー型に変換するのがめんどくさそうね

impl<'a> Config {
	// @MEMO デフォルトパラメータみたいなことができれば
	// @MEMO const値なんだけど、ライフライムを指定する意味はあるのかな
	const DEFAULT_ENTRY: &'a str = "tmp.c";
	const DEFAULT_DIST: &'a str = "tmp.s";

	pub fn new(mut args: env::Args) -> Config {
		args.next();
		// @MEMO lenってnextイテレータすすめると減るのかな
		//if args.len() != 2 {
		//	return Err("引数の数が間違ってるよ".into());
		//}

		let entry = match args.next() {
			Some(args) => args,
			// @MEMO Copyになるけどいいかな
			None => Self::DEFAULT_ENTRY.to_string(),
		};

		let dist = match args.next() {
			Some(args) => args,
			None => Self::DEFAULT_DIST.to_string(),
		};

		Config { entry, dist }
	}
}

pub fn run(config: &Config) -> Result<()> {
	//
	// ファイル読み込み
	//

	//let mut f = match File::open(config.filename) {
	//	Ok(file) => file,
	//	Err(e) => return Err(e),
	//};
	// 上記のショートカット
	// 厳密にはエラー時に返す型がちょっと違うみたい
	// https://doc.rust-jp.rs/book-ja/ch09-02-recoverable-errors-with-result.html#%E3%82%A8%E3%83%A9%E3%83%BC%E5%A7%94%E8%AD%B2%E3%81%AE%E3%82%B7%E3%83%A7%E3%83%BC%E3%83%88%E3%82%AB%E3%83%83%E3%83%88-%E6%BC%94%E7%AE%97%E5%AD%90
	let mut f = File::open(&config.entry)?;
	let mut contents = String::new();
	f.read_to_string(&mut contents)?;
	println!("contents:{}", contents);

	//
	// コンパイル処理
	//
	let mut compiled = String::new();
	compiled = builder(compiled)?;
	println!("compiled:\n{}", compiled);

	//
	// コンパイル結果を出力
	//
	output(&compiled, &config)?;
	Ok(())
}

fn builder(mut s: String) -> Result<String> {
	s.push_str(".intel_syntax noprefix\n");
	s.push_str(".globl main\n");

	s.push_str("main:\n");
	// https://doc.rust-lang.org/std/macro.write.html
	// format!マクロで文字列を生成すると、生成した文字列をヒープに書き込んだ後
	// sのヒープにコピーして、生成した文字列をdropすることになる
	// writeを使えば、sのヒープに直接formatした文字列を書き込めるってことかな
	// https://users.rust-lang.org/t/how-do-i-push-str-the-contents-of-a-variable/45594/6
	write!(&mut s, "  mov rax, {}\n", 12)?;
	s.push_str("  ret\n");
	Ok(s)
}

fn output(s: &str, config: &Config) -> Result<()> {
	let path = Path::new(&config.dist);
	let mut file = File::create(&path)?;
	file.write_all(s.as_bytes())?;
	Ok(())
}
