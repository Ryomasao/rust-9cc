use crate::{Token, TokenKind};

enum NodeKind {
	Num(i32),
	BinOp(TokenKind, Box<Node>, Box<Node>),
}

pub struct Node {
	kind: NodeKind,
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
	let nodes = Vec::new();

	for token in tokens {
		println!("token:{:#?}", token);
	}

	nodes
}
