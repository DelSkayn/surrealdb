use crate::syn::lexer::{CharError, Lexer};
use crate::syn::token::{t, Token, TokenKind};

use super::Error;

impl<'a> Lexer<'a> {
	pub fn lex_char(&mut self, byte: u8) -> Token {
		let c = match self.reader.complete_char(byte) {
			Ok(x) => x,
			Err(CharError::Eof) => return self.eof_token(),
			Err(CharError::Unicode) => return self.finish_token(TokenKind::Invalid, None),
		};
		let kind = match c {
			'⟨' => return self.lex_surrounded_ident(false),
			'∋' => t!("∋"),
			'∌' => t!("∌"),
			'∈' => t!("∈"),
			'∉' => t!("∉"),
			'⊇' => t!("⊇"),
			'⊃' => t!("⊃"),
			'⊅' => t!("⊅"),
			'⊆' => t!("⊆"),
			'⊂' => t!("⊂"),
			'⊄' => t!("⊄"),
			'×' => t!("×"),
			'÷' => t!("÷"),
			x => return self.invalid_token(Error::UnexpectedCharacter(x)),
		};
		self.finish_token(kind, None)
	}
}