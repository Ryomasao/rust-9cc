use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// modとuseの違いがわからなくなったでござる
// 利用するときはuse。子は親をしってるけど、親は子を知らないってことかな。
// 親から利用する場合はmodで読み込んでuseかしら。
// codegenの中でmod fooとした場合、rustはcodegen/foo.rsを期待してるっぽい
// main.rsとlib.rsの場合は同じ階層のファイルをmodで参照するっぽ

mod codegen;
mod config;
mod parse;
mod token;

// pubをつけるとreexport的なかんじ
pub use config::Config;
use token::Tokenizer;

// 組み込みのエラーはいろいろ存在していて、1関数内に複数エラーの型が存在していると
// 返り値の型をどうしていいのかわからなくなる。
// これの対応として返り値はErrorトレイトを実装している型っていうふうに表現することができる。
// が、Rustは返り値の型がトレイトだとHeapに値を保存するしかないので、Box化してheapに保存することを明示するdynをつけるんだって
// https://doc.rust-jp.rs/rust-by-example-ja/trait/dyn.html
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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
	//println!("contents:{}", contents);

	let mut tokenizer = Tokenizer::new(&contents);
	let tokens = tokenizer.generate();

	//
	// 構文木作成
	//
	let nodes = parse::parse(tokens);

	//
	// アセンブリに変換
	//
	let result = codegen::codegen(nodes);
	//println!("compiled:\n{}", result);

	//
	// 出力
	//
	output(&result, &config)?;
	Ok(())
}

#[derive(Debug)]
enum CharType {
	Whitespace,
	Num(char),
	//Alphabetic(char),
	NonAlphabetic(char),
}

impl CharType {
	fn new(c: char) -> CharType {
		if c.is_ascii_whitespace() {
			return CharType::Whitespace;
		}

		if c.is_ascii_digit() {
			return CharType::Num(c);
		}

		//if c.is_ascii_alphabetic() {
		//	return CharType::Alphabetic(c);
		//}

		CharType::NonAlphabetic(c)
	}
}

fn output(s: &str, config: &Config) -> Result<()> {
	let path = Path::new(&config.dist);
	let mut file = File::create(&path)?;
	file.write_all(s.as_bytes())?;
	Ok(())
}
