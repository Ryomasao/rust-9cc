use crate::{Token, TokenKind};

#[derive(Debug)]
pub enum NodeKind {
	Num(i32),
	BinOp(TokenKind, Box<Node>, Box<Node>),
}

#[derive(Debug)]
pub struct Node {
	pub kind: NodeKind,
}

impl Node {
	fn new(kind: NodeKind) -> Self {
		Self { kind }
	}

	fn new_num(v: i32) -> Self {
		Self::new(NodeKind::Num(v))
	}

	fn new_binop(token_kind: TokenKind, lhs: Node, rhs: Node) -> Self {
		Self::new(NodeKind::BinOp(token_kind, Box::new(lhs), Box::new(rhs)))
	}
}

struct Parser {
	// TODO
	// 参照のほうがよさげだけどひとまず
	tokens: Vec<Token>,
	// 参照するtokenの現在位置
	pos: usize,
}

impl Parser {
	fn new(tokens: Vec<Token>) -> Parser {
		Parser { tokens, pos: 0 }
	}

	fn consume(&mut self, expect_token_kind: TokenKind) -> bool {
		let current_token = &self.tokens[self.pos];
		if current_token.kind != expect_token_kind {
			return false;
		}

		self.pos += 1;
		true
	}

	// トークンが期待するkindだったらposをすすめる。
	// そうでない場合、panic。
	fn expect(&mut self, expect_token_kind: TokenKind) {
		let current_token = &self.tokens[self.pos];
		if current_token.kind != expect_token_kind {
			current_token.bad_token(&format!("{:?} を想定してました。", expect_token_kind));
		}
		self.pos += 1;
	}

	fn expr(&mut self) -> Node {
		let mut node = self.mul();

		loop {
			if self.consume(TokenKind::Plus) {
				node = Node::new_binop(TokenKind::Plus, node, self.mul())
			} else if self.consume(TokenKind::Minus) {
				node = Node::new_binop(TokenKind::Minus, node, self.mul())
			} else {
				return node;
			}
		}
	}

	fn mul(&mut self) -> Node {
		let mut node = self.primary();

		loop {
			if self.consume(TokenKind::Mul) {
				node = Node::new_binop(TokenKind::Mul, node, self.primary());
			} else if self.consume(TokenKind::Div) {
				node = Node::new_binop(TokenKind::Div, node, self.primary());
			} else {
				return node;
			}
		}
	}

	fn primary(&mut self) -> Node {
		let current_token = &self.tokens[self.pos];
		self.pos += 1;
		match current_token.kind {
			// ( がくるのであれば、その後はexprがくるはず
			TokenKind::LeftParen => {
				let node = self.expr();
				// exprの後は )
				self.expect(TokenKind::RightParen);
				node
			}
			TokenKind::Num(v) => Node::new_num(v),
			_ => current_token.bad_token("number expected"),
		}
	}
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
	let mut nodes = Vec::new();
	let mut parser = Parser::new(tokens);

	// parser内のtokenを走査してく
	// parser.posは0からはじまるので補正
	while (parser.tokens.len() - 1) != parser.pos {
		nodes.push(parser.expr());
	}

	//println!("{:#?}", nodes);

	nodes
}
