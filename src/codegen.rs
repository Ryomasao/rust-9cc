use std::fmt::Write as FmtWrite;

use crate::parse::{Node, NodeKind};
use crate::token::TokenKind;

pub fn codegen(nodes: Vec<Node>) -> String {
	let mut s = String::new();
	s.push_str(".intel_syntax noprefix\n");
	s.push_str(".globl main\n");

	s.push_str("main:\n");

	// プロローグ
	// 変数割当に関しての過去の記憶
	// https://github.com/Ryomasao/9cc/blob/master/src/lvar.s
	s.push_str("  # prologue start\n");
	s.push_str("  push rbp\n");
	s.push_str("  mov rbp, rsp\n");
	s.push_str("  sub rsp, 208\n");
	s.push_str("  # prologue end\n");

	for node in nodes {
		gen(&mut s, node);
		s.push_str("  # 式の評価結果がスタックから溢れないようにする\n");
		s.push_str("  pop rax\n");
	}

	// エピローグ
	s.push_str("  # epilogue\n");
	s.push_str("  mov rsp, rbp\n");
	s.push_str("  pop rbp\n");
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
				TokenKind::EQ => {
					write!(code, "  cmp rax, rdi\n").unwrap();
					write!(code, "  sete al\n").unwrap();
					write!(code, "  movzb rax, al\n").unwrap();
				}
				TokenKind::LE => {
					write!(code, "  cmp rax, rdi\n").unwrap();
					write!(code, "  setle al\n").unwrap();
					write!(code, "  movzb rax, al\n").unwrap();
				}
				TokenKind::LeftAngleBracket => {
					write!(code, "  cmp rax, rdi\n").unwrap();
					write!(code, "  setle al\n").unwrap();
					write!(code, "  movzb rax, al\n").unwrap();
				}
				TokenKind::NEQ => {
					write!(code, "  cmp rax, rdi\n").unwrap();
					write!(code, "  setne al\n").unwrap();
					write!(code, "  movzb rax, al\n").unwrap();
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
		NodeKind::Assign(lhs, rhs) => {
			// =の場合、左辺値は必ず変数
			if let NodeKind::Lvar(_, _) = lhs.kind {
				gen(code, *lhs);
			} else {
				panic!("unexpected node: {:?}", lhs)
			}

			gen(code, *rhs);

			write!(code, "  pop rdi\n").unwrap();
			write!(code, "  pop rax\n").unwrap();
			write!(code, "  mov [rax], rdi\n").unwrap();
			write!(code, "  push rdi\n").unwrap();
			return code;
		}
		NodeKind::Lvar(_, offset) => {
			write!(code, "  mov rax, rbp\n").unwrap();
			write!(code, "  sub rax, {}\n", offset).unwrap();
			write!(code, "  push rax\n").unwrap();
			return code;
		}
	}

	// このコードは今の所BinOpときだけ実行してる
	write!(code, "  push rax\n").unwrap();

	code
}
