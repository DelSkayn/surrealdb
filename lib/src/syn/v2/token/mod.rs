//! Module specifying the token representation of the parser.

use std::{hash::Hash, num::NonZeroU32};

mod keyword;
pub(crate) use keyword::keyword_t;
pub use keyword::Keyword;
mod mac;
pub(crate) use mac::t;

use crate::sql::{language::Language, Algorithm};

/// A location in the source passed to the lexer.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Span {
	/// Offset in bytes.
	pub offset: u32,
	/// The amount of bytes this location encompasses.
	pub len: u32,
}

impl Span {
	/// Create a new empty span.
	pub const fn empty() -> Self {
		Span {
			offset: 0,
			len: 0,
		}
	}

	pub fn is_empty(&self) -> bool {
		self.len == 0
	}
}

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Operator {
	/// `!`
	Not,
	/// `+`
	Add,
	/// `-`
	Subtract,
	/// `÷`
	Divide,
	/// `×` or `∙`
	Mult,
	/// `||`
	Or,
	/// `&&`
	And,
	/// `<=`
	LessEqual,
	/// `>=`
	GreaterEqual,
	/// `*`
	Star,
	/// `**`
	Power,
	/// `=`
	Equal,
	/// `==`
	Exact,
	/// `!=`
	NotEqual,
	/// `*=`
	AllEqual,
	/// `?=`
	AnyEqual,
	/// `~`
	Like,
	/// `!~`
	NotLike,
	/// `*~`
	AllLike,
	/// `?~`
	AnyLike,
	/// `∋`
	Contains,
	/// `∌`
	NotContains,
	/// `⊇`
	ContainsAll,
	/// `⊃`
	ContainsAny,
	/// `⊅`
	ContainsNone,
	/// `∈`
	Inside,
	/// `∉`
	NotInside,
	/// `⊆`
	AllInside,
	/// `⊂`
	AnyInside,
	/// `⊄`
	NoneInside,
	/// `@123@`
	Matches,
	/// `+=`
	Inc,
	/// `-=`
	Dec,
	/// `+?=`
	Ext,
	/// `??`
	Tco,
	/// `?:`
	Nco,
}

impl Operator {
	fn as_str(&self) -> &'static str {
		match self {
			Operator::Not => "'!'",
			Operator::Add => "'+'",
			Operator::Subtract => "'-'",
			Operator::Divide => "'÷'",
			Operator::Or => "'||'",
			Operator::And => "'&&'",
			Operator::Mult => "'×'",
			Operator::LessEqual => "'<='",
			Operator::GreaterEqual => "'>='",
			Operator::Star => "'*'",
			Operator::Power => "'**'",
			Operator::Equal => "'='",
			Operator::Exact => "'=='",
			Operator::NotEqual => "'!='",
			Operator::AllEqual => "'*='",
			Operator::AnyEqual => "'?='",
			Operator::Like => "'~'",
			Operator::NotLike => "'!~'",
			Operator::AllLike => "'*~'",
			Operator::AnyLike => "'?~'",
			Operator::Contains => "'∋'",
			Operator::NotContains => "'∌'",
			Operator::ContainsAll => "'⊇'",
			Operator::ContainsAny => "'⊃'",
			Operator::ContainsNone => "'⊅'",
			Operator::Inside => "'∈'",
			Operator::NotInside => "'∉'",
			Operator::AllInside => "'⊆'",
			Operator::AnyInside => "'⊂'",
			Operator::NoneInside => "'⊄'",
			Operator::Matches => "'@@'",
			Operator::Inc => "'+='",
			Operator::Dec => "'-='",
			Operator::Ext => "'+?='",
			Operator::Tco => "'??'",
			Operator::Nco => "'?:'",
		}
	}
}

/// A delimiting token, denoting the start or end of a certain production.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Delim {
	/// `()`
	Paren,
	/// `[]`
	Bracket,
	/// `{}`
	Brace,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum GeometryName {
	Feature,
	Point,
	Line,
	Polygon,
	Multipoint,
	Multiline,
	Multipolygon,
	Collection,
}

impl GeometryName {
	pub fn as_str(&self) -> &'static str {
		match self {
			GeometryName::Feature => "feature",
			GeometryName::Point => "point",
			GeometryName::Line => "line",
			GeometryName::Polygon => "polygon",
			GeometryName::Multipoint => "multipoint",
			GeometryName::Multiline => "multiline",
			GeometryName::Multipolygon => "multipolygon",
			GeometryName::Collection => "collection",
		}
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum DistanceKind {
	Euclidean,
	Manhattan,
	Cosine,
	Hamming,
	Minkowski,
}

impl DistanceKind {
	pub fn as_str(&self) -> &'static str {
		match self {
			DistanceKind::Euclidean => "EUCLIDEAN",
			DistanceKind::Manhattan => "MANHATTAN",
			DistanceKind::Cosine => "COSINE",
			DistanceKind::Hamming => "HAMMING",
			DistanceKind::Minkowski => "MINKOWSKI",
		}
	}
}

/// The type of token
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum TokenKind {
	Keyword(Keyword),
	Algorithm(Algorithm),
	Language(Language),
	Geometry(GeometryName),
	Distance(DistanceKind),
	Operator(Operator),
	OpenDelim(Delim),
	CloseDelim(Delim),
	Regex,
	Uuid,
	DateTime,
	Strand,
	/// A parameter like `$name`.
	Parameter,
	/// A duration.
	Duration,
	Number,
	Identifier,
	/// `<`
	LeftChefron,
	/// `>`
	RightChefron,
	/// `*`
	Star,
	/// `?`
	Question,
	/// `$`
	Dollar,
	/// `->`
	ArrowRight,
	/// `<-`
	ArrowLeft,
	/// `<->`
	BiArrow,
	/// '/'
	ForwardSlash,
	/// `.`
	Dot,
	/// `..`
	DotDot,
	/// `...` or `…`
	DotDotDot,
	/// `;`
	SemiColon,
	/// `::`
	PathSeperator,
	/// `:`
	Colon,
	/// `,`
	Comma,
	/// `|`
	Vert,
	/// `@`
	At,
	/// A token which could not be properly lexed.
	Invalid,
	/// A token which indicates the end of the file.
	Eof,
}

impl TokenKind {
	pub fn can_be_identifier(&self) -> bool {
		matches!(
			self,
			TokenKind::Identifier
				| TokenKind::Keyword(_)
				| TokenKind::Language(_)
				| TokenKind::Algorithm(_)
		)
	}

	pub fn as_str(&self) -> &'static str {
		match *self {
			TokenKind::Keyword(x) => x.as_str(),
			TokenKind::Operator(x) => x.as_str(),
			TokenKind::Algorithm(_) => todo!(),
			TokenKind::Language(_) => todo!(),
			TokenKind::Geometry(x) => x.as_str(),
			TokenKind::Distance(x) => x.as_str(),
			TokenKind::OpenDelim(Delim::Paren) => "(",
			TokenKind::OpenDelim(Delim::Brace) => "{",
			TokenKind::OpenDelim(Delim::Bracket) => "[",
			TokenKind::CloseDelim(Delim::Paren) => ")",
			TokenKind::CloseDelim(Delim::Brace) => "}",
			TokenKind::CloseDelim(Delim::Bracket) => "]",
			TokenKind::Uuid => "a uuid",
			TokenKind::DateTime => "a date-time",
			TokenKind::Strand => "a strand",
			TokenKind::Parameter => "a parameter",
			TokenKind::Duration => "a duration",
			TokenKind::Number => "a number",
			TokenKind::Identifier => "an identifier",
			TokenKind::Regex => "a regex",
			TokenKind::LeftChefron => "'<'",
			TokenKind::RightChefron => "'>'",
			TokenKind::Star => "'*'",
			TokenKind::Dollar => "'$'",
			TokenKind::Question => "'?'",
			TokenKind::ArrowRight => "'->'",
			TokenKind::ArrowLeft => "'<-'",
			TokenKind::BiArrow => "'<->'",
			TokenKind::ForwardSlash => "'/'",
			TokenKind::Dot => "'.'",
			TokenKind::DotDot => "'..'",
			TokenKind::DotDotDot => "'...'",
			TokenKind::SemiColon => "';'",
			TokenKind::PathSeperator => "'::'",
			TokenKind::Colon => "':'",
			TokenKind::Comma => "','",
			TokenKind::Vert => "'|'",
			TokenKind::At => "'@'",
			TokenKind::Invalid => "Invalid",
			TokenKind::Eof => "Eof",
		}
	}
}

/// A index for extra data associated with the token.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct DataIndex(NonZeroU32);

impl From<u32> for DataIndex {
	fn from(value: u32) -> Self {
		let idx = NonZeroU32::new(value.checked_add(1).unwrap()).unwrap();
		DataIndex(idx)
	}
}
impl From<DataIndex> for u32 {
	fn from(value: DataIndex) -> Self {
		u32::from(value.0) - 1
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Token {
	pub kind: TokenKind,
	pub span: Span,
	pub data_index: Option<DataIndex>,
}

impl Token {
	pub const fn invalid() -> Token {
		Token {
			kind: TokenKind::Invalid,
			span: Span::empty(),
			data_index: None,
		}
	}

	/// Returns if the token is invalid.
	pub fn is_invalid(&self) -> bool {
		matches!(self.kind, TokenKind::Invalid)
	}

	/// Returns if the token is `end of file`.
	pub fn is_eof(&self) -> bool {
		matches!(self.kind, TokenKind::Eof)
	}
}
