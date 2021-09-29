use std::fmt::Write as FmtWrite;

use crate::parse::Node;
use crate::Result;

pub fn codegen(nodes: Vec<Node>) -> Result<String> {
	let mut s = String::new();
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
