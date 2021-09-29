use std::env;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod codegen;
mod parse;

// 組み込みのエラーはいろいろ存在していて、1関数内に複数エラーの型が存在していると
// 返り値の型をどうしていいのかわからなくなる。
// これの対応として返り値はErrorトレイトを実装している型っていうふうに表現することができる。
// が、Rustは返り値の型がトレイトだとHeapに値を保存するしかないので、Box化してheapに保存することを明示するdynをつけるんだって
// https://doc.rust-jp.rs/rust-by-example-ja/trait/dyn.html
pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct Config {
	pub entry: String,
	pub dist: String,
}

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

	let tokens = tokenize(&contents);

	// 構文木作成
	let nodes = parse::parse(tokens);

	//
	// コンパイル処理
	//
	let result = codegen::codegen(nodes)?;
	println!("compiled:\n{}", result);

	//
	// コンパイル結果を出力
	//
	output(&result, &config)?;
	Ok(())
}

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

#[derive(Debug)]
enum TokenKind {
	Plus,     // +
	Minus,    // +
	Num(i32), // 整数
}

impl TokenKind {
	fn new_single_letter(c: char) -> Option<Self> {
		match c {
			'+' => Some(TokenKind::Plus),
			'-' => Some(TokenKind::Minus),
			_ => None,
		}
	}
}

#[derive(Debug)]
pub struct Token {
	kind: TokenKind,
}

impl Token {
	fn new(kind: TokenKind) -> Self {
		Token { kind }
	}
}

fn tokenize(s: &String) -> Vec<Token> {
	let mut tokens = Vec::new();
	for c in s.chars() {
		let c = CharType::new(c);
		match c {
			CharType::Whitespace => {
				continue;
			}
			//CharType::Alphabetic(c) => {
			//}
			CharType::Num(c) => {
				// これでいいのかふあん
				// char to i32
				let val = c.to_digit(10).unwrap() as i32;
				let token = Token::new(TokenKind::Num(val));
				tokens.push(token);
			}
			CharType::NonAlphabetic(c) => {
				if let Some(token_kind) = TokenKind::new_single_letter(c) {
					let token = Token::new(token_kind);
					tokens.push(token);
					continue;
				}
				// 存在しない記号
				panic!("知らない記号:{}", c);
			}
		}
	}
	tokens
}

fn output(s: &str, config: &Config) -> Result<()> {
	let path = Path::new(&config.dist);
	let mut file = File::create(&path)?;
	file.write_all(s.as_bytes())?;
	Ok(())
}
