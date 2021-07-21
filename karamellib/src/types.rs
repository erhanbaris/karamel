use std::borrow::Borrow;
use std::mem::{self, discriminant};
use std::vec::Vec;
use std::str::Chars;
use std::iter::Peekable;
use std::result::Result;
use std::hash::Hash;
use std::rc::Rc;
use crate::{compiler::ast::KaramelAstType, error::KaramelError};
use crate::error::KaramelErrorType;

pub type ParseResult        = Result<(), KaramelError>;
pub type AstResult<'a>      = Result<KaramelAstType<'a>, KaramelErrorType>;
pub type CompilerResult     = Result<(), KaramelErrorType>;

pub const TAG_NULL        : u64 = 0;
pub const TAG_FALSE       : u64 = 1;
pub const TAG_TRUE        : u64 = 2;

pub const QNAN:         u64 = 0x7ffc_0000_0000_0000;
pub const POINTER_FLAG: u64 = 0x8000_0000_0000_0000;
pub const POINTER_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;
pub const FALSE_FLAG:   u64 = QNAN | TAG_FALSE;
pub const TRUE_FLAG:    u64 = QNAN | TAG_TRUE;
pub const EMPTY_FLAG:   u64 = QNAN | TAG_NULL;

#[derive(PartialEq, Hash, Clone, Copy)]
#[derive(Default)]
#[repr(transparent)]
pub struct VmObject(pub u64);


#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum KaramelKeywordType {
    None=0,
    True,
    False,
    Use,
    Until,
    If,
    Else,
    And,
    Or,
    Empty,
    Modulo,
    Not,
    GreaterThan,
    LessThan,
    GreaterEqualThan,
    LessEqualThan,
    Equal,
    NotEqual,
    Fn,
    Return,
    Endless,
    Break,
    Continue,
    While,
    Load
}

impl KaramelKeywordType {
    pub fn to_operator(&self) -> KaramelOperatorType {
        match &self {
            KaramelKeywordType::And              => KaramelOperatorType::And,
            KaramelKeywordType::Or               => KaramelOperatorType::Or,
            KaramelKeywordType::Modulo           => KaramelOperatorType::Modulo,
            KaramelKeywordType::Not              => KaramelOperatorType::Not,
            KaramelKeywordType::Equal            => KaramelOperatorType::Equal,
            KaramelKeywordType::NotEqual         => KaramelOperatorType::NotEqual,
            KaramelKeywordType::GreaterThan      => KaramelOperatorType::GreaterThan,
            KaramelKeywordType::GreaterEqualThan => KaramelOperatorType::GreaterEqualThan,
            _                                  => KaramelOperatorType::None
        }
    }
}

pub static KEYWORDS: &[(&str, KaramelKeywordType)] = &[
    ("doğru",  KaramelKeywordType::True),
    ("dogru",  KaramelKeywordType::True),
    ("yanlış", KaramelKeywordType::False),
    ("yanlis", KaramelKeywordType::False),
    ("kullan", KaramelKeywordType::Use),
    ("kadar",  KaramelKeywordType::Until),
    ("sonsuz", KaramelKeywordType::Endless),
    ("ise",    KaramelKeywordType::If),
    ("yoksa",   KaramelKeywordType::Else),
    ("ve",     KaramelKeywordType::And),
    ("veya",   KaramelKeywordType::Or),
    ("yok",    KaramelKeywordType::Empty),
    ("mod",    KaramelKeywordType::Modulo),
    ("değil",         KaramelKeywordType::Not),
    ("degil",         KaramelKeywordType::Not),
    ("fonk",            KaramelKeywordType::Fn),
    ("döndür",        KaramelKeywordType::Return),
    ("dondur",        KaramelKeywordType::Return),
    ("kır",           KaramelKeywordType::Break),
    ("kir",           KaramelKeywordType::Break),
    ("devam",       KaramelKeywordType::Continue),
    ("döngü",         KaramelKeywordType::While),
    ("dongu",         KaramelKeywordType::While),
    ("yükle",          KaramelKeywordType::Load),
    ("yukle",          KaramelKeywordType::Load)
];

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum KaramelOperatorType {
    None,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Increment,
    Deccrement,
    Assign,
    AssignAddition,
    AssignSubtraction,
    AssignMultiplication,
    AssignDivision,
    Equal,
    NotEqual,
    Not,
    And,
    Or,
    GreaterThan,
    LessThan,
    GreaterEqualThan,
    LessEqualThan,
    QuestionMark,
    ColonMark,
    LeftParentheses,
    RightParentheses,
    SquareBracketStart,
    SquareBracketEnd,
    Comma,
    Semicolon,
    Dot,
    CommentLine,
    CommentMultilineStart,
    CommentMultilineEnd,
    CurveBracketStart,
    CurveBracketEnd
}

 impl KaramelOperatorType {
     pub fn is_same<T: Borrow<Self>>(&self, other: T) -> bool {
        discriminant(self) == discriminant(other.borrow())
     }
 }

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum KaramelTokenType {
    Integer(i64),
    Double(f64),
    Symbol(Rc<String>),
    Operator(KaramelOperatorType),
    Text(Rc<String>),
    Keyword(KaramelKeywordType),
    WhiteSpace(u8),
    NewLine(u8)
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum KaramelNumberSystem {
    Binary      = 0,
    Octal       = 1,
    Decimal     = 2,
    Hexadecimal = 3
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line      : u32,
    pub start    : u32,
    pub end    : u32,
    pub token_type: KaramelTokenType
}

pub struct Tokinizer<'a> {
    pub line  : u32,
    pub column: u32,
    pub tokens: Vec<Token>,
    pub iter: Peekable<Chars<'a>>,
    pub iter_second: Peekable<Chars<'a>>,
    pub iter_third: Peekable<Chars<'a>>,
    pub data: String,
    pub index: u32
}

impl Tokinizer<'_> {
    pub fn is_end(&mut self) -> bool {
        return match self.iter.peek() {
            Some(_) => false,
            None => true
        };
    }

    pub fn get_char(&mut self) -> char {
        return match self.iter.peek() {
            Some(&c) => c,
            None => '\0'
        };
    }

    pub fn get_next_char(&mut self) -> char {
        return match self.iter_second.peek() {
            Some(&c) => c,
            None => '\0'
        };
    }

    pub fn add_token(&mut self, start: u32, token_type: KaramelTokenType) {
        let token = Token {
            line: self.line,
            start,
            end: self.column,
            token_type
        };
        self.tokens.push(token);
    }

    pub fn increase_index(&mut self) {
        self.index  += self.get_char().len_utf8() as u32;
        self.column += 1;
        self.iter.next();
        self.iter_second.next();
        self.iter_third.next();
    }

    pub fn increate_line(& mut self) {
        self.line += 1;
        self.reset_column();
    }

    pub fn reset_column(& mut self) {
        self.column = 0;
    }
}

pub trait TokenParser {
    fn check(&self, tokinizer: &mut Tokinizer) -> bool;
    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<(), KaramelErrorType>;
}

pub trait CharTraits {
    fn is_new_line(&self) -> bool;
    fn is_whitespace(&self) -> bool;
    fn is_symbol(&self) -> bool;
    fn is_integer(&self) -> bool;
}

impl CharTraits for char {
    fn is_new_line(&self) -> bool {
        *self == '\n'
    }

    fn is_whitespace(&self) -> bool {
        match *self {
            ' ' | '\r' | '\t' => true,
            _ => false
        }
    }

    fn is_symbol(&self) -> bool {
        self.is_alphabetic() || *self == '_' ||  *self == '$'
    }

    fn is_integer(&self) -> bool {
        match *self {
            '0'..='9' => true,
            _ => false,
        }
    }
}

impl KaramelTokenType {

    pub fn is_symbol(&self) -> bool {
        match self {
            KaramelTokenType::Symbol(_) => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn is_keyword(&self) -> bool {
        match self {
            KaramelTokenType::Keyword(_) => true,
            _ => false
        }
    }

    pub fn get_symbol(&self) -> String {
        match self {
            KaramelTokenType::Symbol(string) => string.to_string(),
            _ => String::from("")
        }
    }

    pub fn get_keyword(&self) -> KaramelKeywordType {
        match self {
            KaramelTokenType::Keyword(keyword) => *keyword,
            _ => KaramelKeywordType::None
        }
    }
}
