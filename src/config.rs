use std::env;

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
	// MEMO デフォルトパラメータみたいなことができれば
	// MEMO const値なんだけど、ライフライムを指定する意味はあるのかな
	const DEFAULT_ENTRY: &'a str = "tmp.c";
	const DEFAULT_DIST: &'a str = "tmp.s";

	pub fn new(mut args: env::Args) -> Config {
		args.next();
		// MEMO lenってnextイテレータすすめると減るのかな
		//if args.len() != 2 {
		//	return Err("引数の数が間違ってるよ".into());
		//}

		let entry = match args.next() {
			Some(args) => args,
			// MEMO Copyになるけどいいかな
			None => Self::DEFAULT_ENTRY.to_string(),
		};

		let dist = match args.next() {
			Some(args) => args,
			None => Self::DEFAULT_DIST.to_string(),
		};

		Config { entry, dist }
	}
}
