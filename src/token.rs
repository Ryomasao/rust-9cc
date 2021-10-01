use crate::CharType;

#[derive(PartialEq, Debug)]
pub enum TokenKind {
	Plus,       // +
	Minus,      // +
	Mul,        // *
	Div,        // /
	Num(i32),   // 整数
	LeftParen,  // (
	RightParen, // )
	EQ,         // ==
	EOF,        // トークンの終端
}

impl TokenKind {
	fn new_single_letter(c: char) -> Option<Self> {
		match c {
			'+' => Some(TokenKind::Plus),
			'-' => Some(TokenKind::Minus),
			'*' => Some(TokenKind::Mul),
			'/' => Some(TokenKind::Div),
			'(' => Some(TokenKind::LeftParen),
			')' => Some(TokenKind::RightParen),
			_ => None,
		}
	}

	fn new_multi_letter(c: char) -> Option<Self> {
		// TODO
	}
}

#[derive(Debug)]
pub struct Token {
	pub kind: TokenKind,
}

impl Token {
	pub fn new(kind: TokenKind) -> Self {
		Token { kind }
	}

	fn new_eof() -> Self {
		Token {
			kind: TokenKind::EOF,
		}
	}

	pub fn bad_token(&self, msg: &str) -> ! {
		panic!("{}", msg);
	}
}

pub struct Tokenizer {
	chars: Vec<char>,
	pos: usize,
}

impl Tokenizer {
	pub fn new(s: &String) -> Self {
		let chars = s.chars().collect();
		Tokenizer { chars, pos: 0 }
	}

	fn get_by_pos(&self, pos: usize) -> Option<CharType> {
		self.chars.get(pos).map(|c| CharType::new(*c))
	}

	pub fn generate(&mut self) -> Vec<Token> {
		let mut tokens = Vec::new();

		// 当初 s.chars()をforで回すだけだったんだけど、
		// イテレータを移動させなきゃいけない処理が頻発するので
		// parserとおなじposによるindexアクセスがいいんだね
		while let Some(c) = self.get_by_pos(self.pos) {
			match c {
				CharType::Whitespace => {}
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
					} else {
						// 存在しない記号
						panic!("知らない記号:{}", c);
					}
				}
			}
			self.pos += 1
		}

		tokens.push(Token::new_eof());
		tokens
	}
}
