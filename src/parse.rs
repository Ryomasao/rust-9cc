use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub enum NodeKind {
	Num(i32),
	BinOp(TokenKind, Box<Node>, Box<Node>),
	// BinOpとは区別することにした
	Assign(Box<Node>, Box<Node>),
	Lvar(char, usize), // 左辺値 変数名 offsett
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

	fn new_ident(c: char) -> Self {
		// 変数名は1文字で、RBPからのオフセットを文字に応じて固定にしとく
		let offset = (c as usize - 'a' as usize + 1) * 8;
		Self::new(NodeKind::Lvar(c, offset))
	}

	fn new_assign(lhs: Node, rhs: Node) -> Self {
		Self::new(NodeKind::Assign(Box::new(lhs), Box::new(rhs)))
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

	fn stmt(&mut self) -> Node {
		let node = self.expr();
		self.expect(TokenKind::SemiColon);
		node
	}

	fn expr(&mut self) -> Node {
		self.assign()
	}

	fn assign(&mut self) -> Node {
		let node = self.equality();
		if self.consume(TokenKind::Assign) {
			return Node::new_assign(node, self.assign());
		}
		node
	}

	fn equality(&mut self) -> Node {
		let mut node = self.relational();
		loop {
			if self.consume(TokenKind::EQ) {
				node = Node::new_binop(TokenKind::EQ, node, self.relational())
			} else if self.consume(TokenKind::NEQ) {
				node = Node::new_binop(TokenKind::NEQ, node, self.relational())
			} else {
				return node;
			}
		}
	}

	fn relational(&mut self) -> Node {
		let mut node = self.add();
		loop {
			if self.consume(TokenKind::LE) {
				node = Node::new_binop(TokenKind::LE, node, self.add())
			} else if self.consume(TokenKind::LeftAngleBracket) {
				node = Node::new_binop(TokenKind::LeftAngleBracket, node, self.add())
			} else if self.consume(TokenKind::RE) {
				// > → <
				node = Node::new_binop(TokenKind::LE, self.add(), node)
			} else if self.consume(TokenKind::RightAngleBracket) {
				// >= → <=
				node = Node::new_binop(TokenKind::LeftAngleBracket, self.add(), node)
			} else {
				return node;
			}
		}
	}

	fn add(&mut self) -> Node {
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
		let mut node = self.unary();

		loop {
			if self.consume(TokenKind::Mul) {
				node = Node::new_binop(TokenKind::Mul, node, self.unary());
			} else if self.consume(TokenKind::Div) {
				node = Node::new_binop(TokenKind::Div, node, self.unary());
			} else {
				return node;
			}
		}
	}

	// 単項目
	fn unary(&mut self) -> Node {
		// +xの場合は、ただのxにする
		if self.consume(TokenKind::Plus) {
			return self.primary();
		}
		// -xの場合は、0 - xにする
		if self.consume(TokenKind::Minus) {
			return Node::new_binop(TokenKind::Minus, Node::new_num(0), self.primary());
		}

		self.primary()
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
			// https://doc.rust-jp.rs/book-ja/ch18-03-pattern-syntax.html?highlight=ref#ref%E3%81%A8ref-mut%E3%81%A7%E3%83%91%E3%82%BF%E3%83%BC%E3%83%B3%E3%81%AB%E5%8F%82%E7%85%A7%E3%82%92%E7%94%9F%E6%88%90%E3%81%99%E3%82%8B
			// Stringの場合、matchした値の所有権が移動しないようにrefを利用する
			// とりあえず変数名はcharなのでrefは不要
			TokenKind::Ident(c) => Node::new_ident(c),
			TokenKind::Num(v) => Node::new_num(v),
			_ => current_token.bad_token(&format!("number expected, but actual: {:?}", current_token)),
		}
	}
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
	let mut nodes = Vec::new();
	let mut parser = Parser::new(tokens);

	// parser内のtokenを走査してく
	// parser.posは0からはじまるので補正
	while (parser.tokens.len() - 1) != parser.pos {
		nodes.push(parser.stmt());
	}

	nodes
}
