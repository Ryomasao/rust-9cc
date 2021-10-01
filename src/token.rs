use crate::CharType;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
	Plus,              // +
	Minus,             // +
	Mul,               // *
	Div,               // /
	Num(i32),          // 整数
	LeftParen,         // (
	RightParen,        // )
	LeftAngleBracket,  // <
	RightAngleBracket, // >
	EQ,                // ==
	NEQ,               // !=
	LE,                // <=
	RE,                // >=
	EOF,               // トークンの終端
}

struct Symbol {
	name: &'static str,
	kind: TokenKind,
}

const SYMBOL_LIST: [Symbol; 4] = [
	Symbol {
		name: "==",
		kind: TokenKind::EQ,
	},
	Symbol {
		name: "!=",
		kind: TokenKind::NEQ,
	},
	Symbol {
		name: "<=",
		kind: TokenKind::LE,
	},
	Symbol {
		name: ">=",
		kind: TokenKind::RE,
	},
];

impl TokenKind {
	fn new_single_letter(c: char) -> Option<Self> {
		match c {
			'+' => Some(TokenKind::Plus),
			'-' => Some(TokenKind::Minus),
			'*' => Some(TokenKind::Mul),
			'/' => Some(TokenKind::Div),
			'(' => Some(TokenKind::LeftParen),
			')' => Some(TokenKind::RightParen),
			'<' => Some(TokenKind::LeftAngleBracket),
			'>' => Some(TokenKind::RightAngleBracket),
			_ => None,
		}
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
		'outer: while let Some(c) = self.get_by_pos(self.pos) {
			match c {
				CharType::Whitespace => self.pos += 1,
				//CharType::Alphabetic(c) => {
				//}
				CharType::Num(c) => {
					// これでいいのかふあん
					// char to i32
					let val = c.to_digit(10).unwrap() as i32;
					let token = Token::new(TokenKind::Num(val));
					tokens.push(token);
					self.pos += 1
				}
				CharType::NonAlphabetic(c) => {
					// multi char
					for symbol in SYMBOL_LIST.iter() {
						let len = symbol.name.len();
						if self.pos + len > self.chars.len() {
							continue;
						}
						let key = &self.chars[self.pos..self.pos + len];
						// https://doc.rust-lang.org/nightly/std/iter/trait.Iterator.html#method.collect
						// charのスライスから、Stringに変換してる
						let key = key.iter().collect::<String>();

						if symbol.name == key {
							tokens.push(Token::new(symbol.kind.clone()));
							self.pos += symbol.name.len();
							// loopがネストしてるので
							continue 'outer;
						}
					}

					// single char
					if let Some(token_kind) = TokenKind::new_single_letter(c) {
						let token = Token::new(token_kind);
						tokens.push(token);
						self.pos += 1
					} else {
						// 存在しない記号
						panic!("知らない記号:{}", c);
					}
				}
			}
		}

		tokens.push(Token::new_eof());
		tokens
	}
}
