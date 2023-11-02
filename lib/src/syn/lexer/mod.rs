use crate::{
	sql::{Datetime, Number, Uuid},
	syn::token::{DataIndex, Span, Token, TokenKind},
};

mod byte;
mod char;
mod duration;
mod ident;
mod keywords;
mod number;
mod reader;
mod unicode;

mod datetime;
mod strand;
#[cfg(test)]
mod test;
mod uuid;

pub use reader::{BytesReader, CharError};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Lexer encountered unexpected character {0:?}")]
	UnexpectedCharacter(char),
	#[error("invalid escape character {0:?}")]
	InvalidEscapeCharacter(char),
	#[error("Lexer encountered unexpected end of source characters")]
	UnexpectedEof,
	#[error("source was not valid utf-8")]
	InvalidUtf8,
	#[error("expected token to end with '{0}'")]
	ExpectedEnd(char),
	#[error("failed to lex date-time, {0}")]
	DateTime(#[from] datetime::Error),
	#[error("failed to lex uuid, {0}")]
	Uuid(#[from] uuid::Error),
	#[error("failed to lex duration, {0}")]
	Duration(#[from] duration::Error),
	#[error("failed to lex number, {0}")]
	Number(#[from] number::Error),
}

impl From<CharError> for Error {
	fn from(value: CharError) -> Self {
		match value {
			CharError::Eof => Self::UnexpectedEof,
			CharError::Unicode => Self::InvalidUtf8,
		}
	}
}

/// The SurrealQL lexer.
/// Takes a slice of bytes and turns it into tokens.
///
/// The lexer generates tokens lazily. each time next is called on the lexer it will try to lex the
/// next bytes in the give source as a token.
pub struct Lexer<'a> {
	/// The reader for reading the source bytes.
	pub reader: BytesReader<'a>,
	/// The one past the last character of the previous token.
	last_offset: u32,
	/// A buffer used to build the value of tokens which can't be read straight from the source.
	/// like for example strings with escape characters.
	scratch: String,

	// below are a collection of buffers with values produced by tokens.
	// For performance reasons we wan't to keep the tokens as small as possible.
	// As only some tokens have an additional value associated with them we don't store that value
	// in the token itself but, instead, in the lexer ensureing a smaller size for each individual
	// token.
	pub numbers: Vec<Number>,
	/// Strings build from the source.
	pub strings: Vec<String>,
	pub durations: Vec<Duration>,
	pub datetime: Vec<Datetime>,
	pub uuid: Vec<Uuid>,
	pub error: Option<Error>,
}

impl<'a> Lexer<'a> {
	/// Create a new lexer.
	///
	/// # Panic
	///
	/// Will panic if the size of the provided slice exceeds `u32::MAX`.
	pub fn new(source: &'a str) -> Lexer<'a> {
		let reader = BytesReader::new(source.as_bytes());
		assert!(reader.len() <= u32::MAX as usize, "source code exceeded maximum size");
		Lexer {
			reader,
			last_offset: 0,
			scratch: String::new(),
			numbers: Vec::new(),
			strings: Vec::new(),
			datetime: Vec::new(),
			durations: Vec::new(),
			uuid: Vec::new(),
			error: None,
		}
	}

	/// Returns the next token, driving the lexer forward.
	///
	/// If the lexer is at the end the source it will always return the Eof token.
	pub fn next_token(&mut self) -> Token {
		let Some(byte) = self.reader.next() else {
			return self.eof_token();
		};
		if byte.is_ascii() {
			self.lex_ascii(byte)
		} else {
			self.lex_char(byte)
		}
	}

	/// Creates the eof token.
	///
	/// An eof token has tokenkind Eof and an span which points to the last character of the
	/// source.
	fn eof_token(&mut self) -> Token {
		Token {
			kind: TokenKind::Eof,
			span: Span {
				offset: self.last_offset.saturating_sub(1),
				len: 1,
			},
			data_index: None,
		}
	}

	/// Skip the last consumed bytes in the reader.
	///
	/// The bytes consumed before this point won't be part of the span.
	fn skip_offset(&mut self) {
		self.last_offset = self.reader.offset() as u32;
	}

	/// Return an invalid token.
	fn invalid_token(&mut self, error: Error) -> Token {
		self.error = Some(error);
		self.finish_token(TokenKind::Invalid, None)
	}

	// Returns the span for the current token being lexed.
	fn current_span(&self) -> Span {
		// We make sure that the source is no longer then u32::MAX so this can't overflow.
		let new_offset = self.reader.offset() as u32;
		let len = new_offset - self.last_offset;
		Span {
			offset: self.last_offset,
			len,
		}
	}

	/// Builds a token from an TokenKind.
	///
	/// Attaches a span to the token and returns, updates the new offset.
	fn finish_token(&mut self, kind: TokenKind, data_index: Option<DataIndex>) -> Token {
		let span = self.current_span();
		// We make sure that the source is no longer then u32::MAX so this can't overflow.
		self.last_offset = self.reader.offset() as u32;
		Token {
			kind,
			span,
			data_index,
		}
	}

	/// Finish a token which contains a string like value.
	///
	/// Copies out all of the values in scratch and pushes into the data array.
	/// Attaching it to the token.
	fn finish_string_token(&mut self, kind: TokenKind) -> Token {
		let id = self.strings.len() as u32;
		let string = self.scratch.clone();
		self.scratch.clear();
		self.strings.push(string);
		self.finish_token(kind, Some(id.into()))
	}

	fn finish_number_token(&mut self, number: Number) -> Token {
		let id = self.numbers.len() as u32;
		self.numbers.push(number);
		self.finish_token(TokenKind::Number, Some(id.into()))
	}

	/// Moves the lexer state back to before the give span.
	///
	/// # Warning
	/// Moving the lexer into a state where the next byte is within a multibyte character will
	/// result in spurious errors.
	pub fn backup_before(&mut self, span: Span) {
		self.reader.backup(span.offset as usize);
		self.last_offset = span.offset;
	}

	/// Moves the lexer state to after the give span.
	///
	/// # Warning
	/// Moving the lexer into a state where the next byte is within a multibyte character will
	/// result in spurious errors.
	pub fn backup_after(&mut self, span: Span) {
		let offset = span.offset + span.len;
		self.reader.backup(offset as usize);
		self.last_offset = offset;
	}

	/// Checks if the next byte is the give byte, if it is it consumes the byte and returns true.
	/// Otherwise returns false.
	///
	/// Also returns false if there is no next character.
	pub fn eat(&mut self, byte: u8) -> bool {
		if self.reader.peek() == Some(byte) {
			self.reader.next();
			true
		} else {
			false
		}
	}

	/// Checks if the closure returns true when given the next byte, if it is it consumes the byte
	/// and returns true. Otherwise returns false.
	///
	/// Also returns false if there is no next character.
	pub fn eat_when<F: FnOnce(u8) -> bool>(&mut self, f: F) -> bool {
		let Some(x) = self.reader.peek() else {
			return false;
		};
		if f(x) {
			self.reader.next();
			true
		} else {
			false
		}
	}
}

impl Iterator for Lexer<'_> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let token = self.next_token();
		if token.is_eof() {
			return None;
		}
		Some(token)
	}
}