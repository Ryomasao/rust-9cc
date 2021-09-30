use std::fmt::Write as FmtWrite;

use crate::parse::{Node, NodeKind};
use crate::TokenKind;

pub fn codegen(nodes: Vec<Node>) -> String {
	let mut s = String::new();
	s.push_str(".intel_syntax noprefix\n");
	s.push_str(".globl main\n");

	s.push_str("main:\n");
	for node in nodes {
		gen(&mut s, node);
	}
	s.push_str("  pop rax\n");
	s.push_str("  ret\n");
	s
}

pub fn gen(code: &mut String, node: Node) -> &String {
	match node.kind {
		NodeKind::BinOp(token_kind, lhs, rhs) => {
			// Box外し
			gen(code, *lhs);
			gen(code, *rhs);

			write!(code, "  pop rdi\n").unwrap();
			write!(code, "  pop rax\n").unwrap();

			match token_kind {
				TokenKind::Plus => {
					write!(code, "  add rax, rdi\n").unwrap();
				}
				TokenKind::Minus => {
					write!(code, "  sub rax, rdi\n").unwrap();
				}
				TokenKind::Mul => {
					write!(code, "  imul rax, rdi\n").unwrap();
				}
				TokenKind::Div => {
					write!(code, "  cqo\n").unwrap();
					write!(code, "  idiv rdi\n").unwrap()
				}
				_ => panic!("unexpected token kind"),
			}
		}
		NodeKind::Num(v) => {
			// https://doc.rust-lang.org/std/macro.write.html
			// format!マクロで文字列を生成すると、生成した文字列をヒープに書き込んだ後
			// sのヒープにコピーして、生成した文字列をdropすることになる
			// writeを使えば、sのヒープに直接formatした文字列を書き込めるってことかな
			// https://users.rust-lang.org/t/how-do-i-push-str-the-contents-of-a-variable/45594/6
			write!(code, "  push {}\n", v).unwrap();
			return code;
		}
	}

	// このコードはNumのときには実行してない
	write!(code, "  push rax\n").unwrap();

	code
}
