use std::fmt;

use logos::{Lexer, Logos};

use crate::keyword::{self, Keyword};
use crate::{Joined, LexError};

fn whitespace_callback(lexer: &mut Lexer<BaseTokenKind>) {
	lexer.extras = Joined::Seperated;
}

#[derive(Logos, Clone, Copy, PartialEq, Eq, Debug)]
#[logos(extras = Joined)]
#[logos(error(LexError, LexError::from_lexer))]
#[logos(subpattern duration_part = r"[0-9]+(y|w|d|h|m|s|ms|us|µs|ns)")]
#[logos(subpattern backtick_ident = r"`([^`\\]|\\.)*`")]
#[logos(subpattern bracket_ident = r"⟨([^⟩\\]|\\.)*⟩")]
#[logos(subpattern whitespace = r"[ \t\n\r\u{0085}\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200A}\u{2028}\u{2029}\u{202F}\u{205F}\u{3000}]+")]
#[logos(subpattern multi_line_comment = r"/\*([^*]|\*[^/])*\*/")]
#[logos(subpattern line_comment = r"(//|#|--)[^\n\r\u{2028}\u{2029}]*")]
#[logos(skip(r"((?&whitespace)|(?&line_comment)|(?&multi_line_comment))+", whitespace_callback))]
pub enum BaseTokenKind {
	#[token("{")]
	/// `{`
	OpenBrace,
	#[token("}")]
	/// `}`
	CloseBrace,
	#[token("[")]
	/// `[`
	OpenBracket,
	#[token("]")]
	/// `]`
	CloseBracket,
	#[token("(")]
	/// `(`
	OpenParen,
	#[token(")")]
	/// `)`
	CloseParen,

	#[token(";")]
	SemiColon,
	#[token(",")]
	Comma,
	#[token("@")]
	At,
	#[token("/")]
	Slash,
	#[token("%")]
	Percent,

	#[token("|")]
	HLine,
	#[token("||")]
	HLineHLine,
	#[token("|>")]
	HLineRightShevron,

	#[token("&&")]
	AndAnd,

	#[token(".")]
	Dot,
	#[token("..")]
	DotDot,
	#[token("...")]
	#[token("…")]
	DotDotDot,

	#[token("!")]
	Exclaim,
	#[token("!=")]
	ExclaimEq,

	#[token("?")]
	Question,
	#[token("??")]
	QuestionQuestion,
	#[token("?=")]
	QuestionEqual,
	#[token("?:")]
	QuestionColon,

	#[token("<")]
	LeftShevron,
	#[token("<~")]
	LeftShevronTilde,
	#[token("<=")]
	LeftShevronEqual,
	#[token("<|")]
	LeftShevronHLine,

	#[token(">")]
	RightShevron,
	#[token(">=")]
	RightShevronEqual,

	#[token("-")]
	Dash,
	#[token("-=")]
	DashEqual,
	#[token("->")]
	DashRightShevron,

	#[token("+")]
	Plus,
	#[token("+=")]
	PlusEqual,
	#[token("+?=")]
	PlusQuestionEqual,

	#[token("*")]
	Star,
	#[token("*=")]
	StarEqual,
	#[token("**")]
	StarStar,

	#[token("=")]
	Equal,
	#[token("==")]
	EqualEqual,

	#[token(":")]
	Colon,
	#[token("::")]
	ColonColon,

	#[token("$")]
	Dollar,

	#[token("×")]
	Times,
	#[token("÷")]
	Divide,
	#[token("∋")]
	Contains,
	#[token("∌")]
	NotContains,
	#[token("∈")]
	Inside,
	#[token("∉")]
	NotInside,
	#[token("⊇")]
	ContainsAll,
	#[token("⊃")]
	ContainsAny,
	#[token("⊅")]
	ContainsNone,
	#[token("⊆")]
	AllInside,
	#[token("⊂")]
	AnyInside,
	#[token("⊄")]
	NoneInside,

	#[regex(r#"(s)?"([^"\\]|\\.)*""#)]
	#[regex(r#"(s)?'([^'\\]|\\.)*'"#)]
	String,
	#[regex(r#"r"([^"\\]|\\.)*""#)]
	#[regex(r#"r'([^'\\]|\\.)*'"#)]
	RecordIdString,
	#[regex(r#"u"([^"\\]|\\.)*""#)]
	#[regex(r#"u'([^'\\]|\\.)*'"#)]
	UuidString,
	#[regex(r#"d"([^"\\]|\\.)*""#)]
	#[regex(r#"d'([^'\\]|\\.)*'"#)]
	DateTimeString,
	#[regex(r#"f"([^"\\]|\\.)*""#)]
	#[regex(r#"f'([^'\\]|\\.)*'"#)]
	FileString,
	#[regex(r#"b"([^"\\]|\\.)*""#)]
	#[regex(r#"b'([^'\\]|\\.)*'"#)]
	ByteString,

	#[regex(r"\$(?&backtick_ident)")]
	#[regex(r"\$(?&bracket_ident)")]
	#[regex(r"\$\p{XID_Continue}+", priority = 3)]
	Param,
	#[regex(r"(?&backtick_ident)", |_| Keyword::None)]
	#[regex(r"(?&bracket_ident)", |_| Keyword::None)]
	#[regex(r"[_\p{XID_Start}]\p{XID_Continue}*", |x| keyword::refine_keyword(x.slice()))]
	Ident(Keyword),

	#[token("NaN")]
	NaN,
	#[token(r"Infinity")]
	Infinity,
	#[token(r"+Infinity")]
	PosInfinity,
	#[token(r"-Infinity")]
	NegInfinity,
	#[regex(r"[0-9][0-9_]*f")]
	#[regex(r"[0-9][0-9_]*(\.[0-9][0-9_]*)?([eE][-+]?[0-9][0-9_]*)?(f)?")]
	Float,
	#[regex(r"[0-9][0-9_]*(\.[0-9][0-9_]*)?([eE][-+]?[0-9][0-9_]*)?dec")]
	Decimal,
	#[regex(r"[0-9][0-9_]*", priority = 3)]
	Int,
	#[regex(r"(?&duration_part)+")]
	Duration,
}

impl BaseTokenKind {
	/// Returns a description of the token, used for generating an error when expecting a specific
	/// token.
	pub fn description(&self) -> &'static str {
		match self {
			BaseTokenKind::OpenParen => "`(`",
			BaseTokenKind::CloseParen => "`)`",
			BaseTokenKind::OpenBrace => "`{`",
			BaseTokenKind::CloseBrace => "`}`",
			BaseTokenKind::OpenBracket => "`[`",
			BaseTokenKind::CloseBracket => "`]`",
			BaseTokenKind::SemiColon => "`;`",
			BaseTokenKind::Comma => "`,`",
			BaseTokenKind::At => "`@`",
			BaseTokenKind::Slash => "`/`",
			BaseTokenKind::Percent => "`%`",
			BaseTokenKind::HLine => "`|`",
			BaseTokenKind::HLineHLine => "`||`",
			BaseTokenKind::HLineRightShevron => "`|>`",
			BaseTokenKind::AndAnd => "`&&`",
			BaseTokenKind::Dot => "`.`",
			BaseTokenKind::DotDot => "`..`",
			BaseTokenKind::DotDotDot => "`...`",
			BaseTokenKind::Exclaim => "`!`",
			BaseTokenKind::ExclaimEq => "`!=`",
			BaseTokenKind::Question => "`?`",
			BaseTokenKind::QuestionQuestion => "`??`",
			BaseTokenKind::QuestionEqual => "`?=`",
			BaseTokenKind::QuestionColon => "`?:`",
			BaseTokenKind::LeftShevron => "`<`",
			BaseTokenKind::LeftShevronTilde => "`<~`",
			BaseTokenKind::LeftShevronEqual => "`<=`",
			BaseTokenKind::LeftShevronHLine => "`<|`",
			BaseTokenKind::RightShevron => "`>`",
			BaseTokenKind::RightShevronEqual => "`>=`",
			BaseTokenKind::Dash => "`-`",
			BaseTokenKind::DashEqual => "`-=`",
			BaseTokenKind::DashRightShevron => "`->`",
			BaseTokenKind::Plus => "`+`",
			BaseTokenKind::PlusEqual => "`+=`",
			BaseTokenKind::PlusQuestionEqual => "`+?=`",
			BaseTokenKind::Star => "`*`",
			BaseTokenKind::StarEqual => "`*=`",
			BaseTokenKind::StarStar => "`**`",
			BaseTokenKind::Equal => "`=`",
			BaseTokenKind::EqualEqual => "`==`",
			BaseTokenKind::Colon => "`:`",
			BaseTokenKind::ColonColon => "`::`",
			BaseTokenKind::Dollar => "`$`",
			BaseTokenKind::Times => "`×`",
			BaseTokenKind::Divide => "`÷`",
			BaseTokenKind::Contains => "`∋`",
			BaseTokenKind::NotContains => "`∌`",
			BaseTokenKind::Inside => "`∈`",
			BaseTokenKind::NotInside => "`∉`",
			BaseTokenKind::ContainsAll => "`⊇`",
			BaseTokenKind::ContainsAny => "`⊃`",
			BaseTokenKind::ContainsNone => "`⊅`",
			BaseTokenKind::AllInside => "`⊆`",
			BaseTokenKind::AnyInside => "`⊂`",
			BaseTokenKind::NoneInside => "`⊄`",
			BaseTokenKind::Ident(x) => x.description(),
			BaseTokenKind::String => "a string",
			BaseTokenKind::RecordIdString => "a record-id string",
			BaseTokenKind::UuidString => "a uuid",
			BaseTokenKind::DateTimeString => "a datetime",
			BaseTokenKind::FileString => "a file path",
			BaseTokenKind::ByteString => "a byte string",
			BaseTokenKind::Param => "a parameter",
			BaseTokenKind::NaN => "`NaN`",
			BaseTokenKind::Infinity => "infinity",
			BaseTokenKind::PosInfinity => "infinity",
			BaseTokenKind::NegInfinity => "negative infinity",
			BaseTokenKind::Float => "a float",
			BaseTokenKind::Decimal => "a decimal",
			BaseTokenKind::Int => "an integer",
			BaseTokenKind::Duration => "a duration",
		}
	}
}

impl fmt::Display for BaseTokenKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.description())
	}
}
