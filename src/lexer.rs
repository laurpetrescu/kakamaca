//use lazy_static::lazy_static;
//use std::collections::HashMap;

#[allow(dead_code)]
pub mod lexer {
// 	const ILLEGAL: &str		= "ILLEGAL";
// 	const IDENTIFIER: &str 	= "IDENTIFIER";
// 	const INTEGER: &str		= "INTEGER";
	
	const FUNCTION: &str	= "fn";
	const LET: &str 		= "let";
	const TRUE: &str 		= "true";
	const FALSE: &str 		= "false";
	const IF: &str	 		= "if";
	const ELSE: &str 		= "else";
	const RETURN: &str 		= "return";
	const KAKA: &str 		= "kaka";
	const MACA: &str 		= "maca";
	
	const EQUAL: &str		= "==";
	const NOT_EQUAL: &str	= "!=";
	
	const EOF: char			= '\0';
	const ASSIGN: char 		= '=';
	const PLUS: char 		= '+';
	const MINUS: char 		= '-';
	const BANG: char 		= '!';
	const ASTERIX: char		= '*';
	const SLASH: char 		= '/';
	const LT: char 			= '<';
	const GT: char	 		= '>';
	
	const COMMA: char 		= ',';
	const SEMICOLON: char	= ';';
	const LPAREN: char 		= '(';
	const RPAREN: char 		= ')';
	const LBRACE: char 		= '{';
	const RBRACE: char 		= '}';
	
	#[derive(Debug, Clone)]
	pub struct Token {
		pub token_type: TokenType,
		pub literal: String
	}

	#[derive(Debug, Clone, PartialEq, Eq)]
	pub enum TokenType {
		Invalid,
		Eof,
		Kaka,
		Maca,
		
		Identifier,
		Integer,
	
		Equal,
		NotEqual,
		Assign,
		Plus,
		Minus,
		Bang,
		Asterix,
		Slash,
		Lt,
		Gt,
		
		Comma,
		Semicolon,
		Lparen,
		Rparen,
		Lbrace,
		Rbrace,
	
		Function,
		Let,
		True,
		False,
		If,
		Else,
		Return
	}
	
	lazy_static::lazy_static! {
	static ref KEYWORDS: std::collections::HashMap<&'static str, TokenType> = {
		let mut map = std::collections::HashMap::new();
		map.insert(FUNCTION, TokenType::Function);
		map.insert(LET, TokenType::Let);
		map.insert(TRUE, TokenType::True);
		map.insert(FALSE, TokenType::False);
		map.insert(IF, TokenType::If);
		map.insert(ELSE, TokenType::Else);
		map.insert(RETURN, TokenType::Return);
		map.insert(KAKA, TokenType::Kaka);
		map.insert(MACA, TokenType::Maca);
		
		map
	};
}
	
	fn lookup_keyword(keyword: &String) -> TokenType {
		match KEYWORDS.get(&keyword[..]) {
			Some(key) => {
				//println!("deb key word: {}", keyword);
				key.clone()
			},
			None => {
				//println!("deb indentifier: {}", keyword);
				TokenType::Identifier
			}
		}
	}
	
	#[allow(dead_code)]
	pub struct Lexer {
		input: String,
		position: usize,
		read_position: usize,
		pub ch: char
	}
	
	fn is_valid_identifier(ch: char) -> bool {
		ch.is_ascii_alphabetic() || ch == '_'

	}
	
	fn is_digit(ch: char) -> bool {
		'0' <= ch && ch <= '9'
	}
	
	fn is_whitespace(ch: char) -> bool {
		ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
	}
	
	fn skip_whitespace(lex: &mut Lexer) {
		while is_whitespace(lex.ch) {
			lex.read_char();
		}
	}
	
	#[allow(dead_code)]
	impl Lexer {
		/*
		 * Constructor
		 */
		pub fn new(input: String) -> Lexer {
			let mut lex = Lexer{input: input, position: 0, read_position: 0, ch: '\0'};
			lex.read_char();
			return lex;
		}
		
		/* TODO: read/test for UTF-8 chars
		 * Read char
		 */
		pub fn read_char(&mut self) {
			if self.read_position >= self.input.len() {
				self.ch = '\0';
			} else {
				self.ch = self.input.chars().nth(self.read_position).unwrap();
			}
			
			self.position = self.read_position;
			self.read_position += 1;
		}
		
		/*
		 * Peek one char without moving the position marker
		 */
		pub fn peek_char(&self) -> char {
			if self.read_position >= self.input.len() {
				'\0'
			} else {
				self.input.chars().nth(self.read_position).unwrap()
			}
		}
		
		/* 
		 * Read identifier
		 */
		pub fn read_identifier(&mut self) -> String {
			let pos = self.position;
			while is_valid_identifier(self.peek_char()) {
				self.read_char();
			}
			
			self.input[pos..self.position+1].to_string()
		}
		
		/*
		 * Read number
		 */
		pub fn read_number(&mut self) -> String {
			let pos = self.position;
			
			while is_digit(self.peek_char()) {
				self.read_char();
			}
			
			self.input[pos..self.position+1].to_string()
		}
		
		/*
		 * Match next token
		 */
		pub fn next_token(&mut self) -> Token {
			skip_whitespace(self);
			let mut lit: String = self.ch.to_string();
			
			let tok = match self.ch {
				ASSIGN => {
					if self.peek_char() == ASSIGN {
						self.read_char();
						lit = String::from(EQUAL);
						TokenType::Equal
					} else {
						TokenType::Assign
					}
				},
				BANG => {
					if self.peek_char() == ASSIGN {
						self.read_char();
						lit = String::from(NOT_EQUAL);
						TokenType::NotEqual
					} else {
						TokenType::Bang
					}
				},
				PLUS => TokenType::Plus,
				MINUS => TokenType::Minus,
				SLASH => TokenType::Slash,
				ASTERIX => TokenType::Asterix,
				LT => TokenType::Lt,
				GT => TokenType::Gt,
				
				COMMA => TokenType::Comma,
				SEMICOLON => TokenType::Semicolon,
				LPAREN => TokenType::Lparen,
				RPAREN => TokenType::Rparen,
				LBRACE => TokenType::Lbrace,
				RBRACE => TokenType::Rbrace,
				EOF => TokenType::Eof,
				_ => {
					//println!("deb {}", self.ch);
					if is_valid_identifier(self.ch) {
						lit = self.read_identifier();
						lookup_keyword(&lit)
					} else if is_digit(self.ch) {
						lit = self.read_number();
						TokenType::Integer
					} else {
						TokenType::Invalid
					}
				}
			};
			
			let token = Token{token_type: tok, literal: lit};
			self.read_char();
			return token;
		}
	}

	
/////////////////////////////////////////////////////////////////////////////////

// 	trait NodeTrait {
// 		fn token_literal(&self) -> String;
// 	}
	
	trait StatementTrait {
		fn token_literal(&self) -> &str;
	}
	
// 	trait ExpressionTrait {
// 		fn expression_node(&self);
// 	}
	
	struct Node;
	struct Expression;
	
	struct Identifier {
		token: Token,
		value: String
	}
	
	impl StatementTrait for Identifier {
		fn token_literal(&self) -> &str {
			self.token.literal.as_str()
		}
	}
	
	struct LetStatement {
		token: Token,
		name: String,
		value: Expression
	}
	
	impl StatementTrait for LetStatement {
		fn token_literal(&self) -> &str {
			self.token.literal.as_str()
		}
	}
	
	impl LetStatement {
		pub fn new() -> Box<LetStatement> {
			Box::new(LetStatement{
				token: Token{token_type: TokenType::Let, literal: LET.to_string()},
				name: String::from(""),
				value: Expression{}
			})
		}
	}
	
	struct ReturnStatement {
		token: Token
		//return_value: ReturnValue
	}
	
	impl StatementTrait for ReturnStatement {
		fn token_literal(&self) -> &str {
			&self.token.literal
		}
	}
	
	
	impl ReturnStatement {
		pub fn new() -> Box<ReturnStatement> {
			Box::new(ReturnStatement{
				token: Token{token_type: TokenType::Return, literal: RETURN.to_string()}
				//return_value: 
			})
		}
	}
	
	struct Program {
		statements: Vec<Box<dyn StatementTrait>>
	}
	
	impl Program {
		fn token_literal(&self) -> &str {
			if self.statements.len() > 0 {
				self.statements[0].token_literal()
			} else {
				""
			}
		}
	}
	
	struct Parser {
		lexer: Lexer,
		errors: Vec<String>,
		current_token: Token,
		peek_token: Token
	}
	
	impl Parser {
		pub fn new(lex: Lexer) -> Parser {
			let mut parser = Parser{lexer: lex,
				errors: vec![],
				current_token: Token{token_type: TokenType::Eof, literal: String::from("")},
				peek_token: Token{token_type: TokenType::Eof, literal: String::from("")}};
			
			// populate current and peek TokenType
			parser.next_token();
			parser.next_token();
			
			return parser;
		}
		
		pub fn errors(&self) -> &Vec<String> {
			&self.errors
		}
		
		pub fn peek_error(&mut self, tok: TokenType) {
			self.errors.push(format!("expected next token to be {:?}, got {:?} instead",
				tok, self.peek_token.token_type));
		}
		
		pub fn next_token(&mut self) {
			self.current_token = self.peek_token.clone();
			self.peek_token = self.lexer.next_token();
		}
		
		pub fn parse_program(&mut self) -> Program {
			let mut program = Program{statements: vec![]};
			
			loop {
				match self.current_token.token_type {
					TokenType::Eof => break,
					_ => {
						if let Some(statement) = self.parse_statement() {
							program.statements.push(statement);
						}
						
						self.next_token();
					}
				}
			}
			
			return program;
		}
		
		pub fn parse_statement(&mut self) -> Option<Box<dyn StatementTrait>> {
			match self.current_token.token_type {
				TokenType::Let => self.parse_let_statement(),
				TokenType::Return => self.parse_return_statement(),
				_ => None
			}
		}
		
		pub fn parse_let_statement(&mut self) -> Option<Box<dyn StatementTrait>> {
			let stmt = LetStatement::new();
			
			if !self.expect_peek(TokenType::Identifier) {
				return None;
			}
			
			while !self.is_current_token(TokenType::Semicolon) {
				self.next_token();
			}
			
			return Some(stmt);
		}
		
		pub fn parse_return_statement(&mut self) -> Option<Box<dyn StatementTrait>> {
			let stmt = ReturnStatement::new();
			
			self.next_token();
			
			while !self.is_current_token(TokenType::Semicolon) {
				self.next_token();
			}
			
			return Some(stmt);
		}
		
		fn is_current_token(&self, tok: TokenType) -> bool {
			tok == self.current_token.token_type
		}
		
		fn is_peek_token(&self, tok: TokenType) -> bool {
			tok == self.peek_token.token_type
		}
		
		fn expect_peek(&mut self, tok: TokenType) -> bool {
			if self.is_peek_token(tok.clone()) {
				self.next_token();
				return true;
			} else {
				self.peek_error(tok);
				return false;
			}
		}
	}
	
	
}

// #[cfg(test)]
// mod tests {
// 	#[test]
// 	fn it_works() {
// 		assert_eq!(2+2, 4);
// 	}
// 	
// 	#[test]
// 	fn call_lexer() {
// 		let mut l: lexer::lexer::Lexer = lexer::lexer::Lexer::new(String::from("let abc=,{}() 123"));
// 		let mut tok = l.next_token();
// 	
// 		match tok {
//  			lexer::lexer::TokenType::Identifier(s) => {
// 				println!("{}", s);
// 			},
// 			_ => {
// 				println!("somesome");
// 			}
// 		}
// 	}
// }
