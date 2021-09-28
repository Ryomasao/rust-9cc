use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

#[derive(Debug)]
pub struct Config {
	pub filename: String,
}

impl Config {
	pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
		args.next();
		// lenってnextイテレータすすめると減るのかな
		if args.len() != 1 {
			return Err("引数の数が間違ってるよ");
		}

		let filename = match args.next() {
			Some(args) => args,
			None => return Err("コンパイル対象のファイル名を入力してね"),
		};

		Ok(Config { filename })
	}
}

pub fn run(config: Config) -> Result<(), Error> {
	//let mut f = match File::open(config.filename) {
	//	Ok(file) => file,
	//	Err(e) => return Err(e),
	//};

	// 上記のショートカット
	// 厳密にはエラー時に返す型がちょっと違うみたい
	// https://doc.rust-jp.rs/book-ja/ch09-02-recoverable-errors-with-result.html#%E3%82%A8%E3%83%A9%E3%83%BC%E5%A7%94%E8%AD%B2%E3%81%AE%E3%82%B7%E3%83%A7%E3%83%BC%E3%83%88%E3%82%AB%E3%83%83%E3%83%88-%E6%BC%94%E7%AE%97%E5%AD%90
	let mut f = File::open(config.filename)?;
	let mut contents = String::new();
	f.read_to_string(&mut contents)?;

	println!("contents:{}", contents);

	Ok(())
}
