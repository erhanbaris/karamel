use std::vec::Vec;
use std::str::Chars;
use std::iter::Peekable;
use std::result::Result;
use std::hash::Hash;
use std::rc::Rc;
use crate::{compiler::ast::BramaAstType, error::BramaError};
use crate::error::BramaErrorType;

pub type ParseResult        = Result<(), BramaError>;
pub type AstResult          = Result<BramaAstType, BramaErrorType>;
pub type CompilerResult     = Result<(), &'static str>;

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
#[repr(transparent)]
pub struct VmObject(pub u64);


#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum BramaKeywordType {
    None=0,
    True,
    False,
    Use,
    Until,
    Loop,
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
    WhileStartPart,
    WhileEndPart
}

impl BramaKeywordType {
    pub fn to_operator(&self) -> BramaOperatorType {
        match &self {
            BramaKeywordType::And              => BramaOperatorType::And,
            BramaKeywordType::Or               => BramaOperatorType::Or,
            BramaKeywordType::Modulo           => BramaOperatorType::Modulo,
            BramaKeywordType::Not              => BramaOperatorType::Not,
            BramaKeywordType::Equal            => BramaOperatorType::Equal,
            BramaKeywordType::NotEqual         => BramaOperatorType::NotEqual,
            BramaKeywordType::GreaterThan      => BramaOperatorType::GreaterThan,
            BramaKeywordType::GreaterEqualThan => BramaOperatorType::GreaterEqualThan,
            BramaKeywordType::LessThan         => BramaOperatorType::LessThan,
            BramaKeywordType::LessEqualThan    => BramaOperatorType::LessEqualThan,
            _                                  => BramaOperatorType::None
        }
    }
}

pub static KEYWORDS: &[(&str, BramaKeywordType)] = &[
    ("doğru",  BramaKeywordType::True),
    ("dogru",  BramaKeywordType::True),
    ("yanlış", BramaKeywordType::False),
    ("yanlis", BramaKeywordType::False),
    ("kullan", BramaKeywordType::Use),
    ("kadar",  BramaKeywordType::Until),
    ("döngü",  BramaKeywordType::Loop),
    ("dongu",  BramaKeywordType::Loop),
    ("sonsuz", BramaKeywordType::Endless),
    ("ise",    BramaKeywordType::If),
    ("yoksa",   BramaKeywordType::Else),
    ("ve",     BramaKeywordType::And),
    ("veya",   BramaKeywordType::Or),
    ("yok",    BramaKeywordType::Empty),
    ("mod",    BramaKeywordType::Modulo),
    ("eşittir",       BramaKeywordType::Equal),
    ("esittir",       BramaKeywordType::Equal),
    ("eşitdeğildir",  BramaKeywordType::NotEqual),
    ("esitdegildir",  BramaKeywordType::NotEqual),
    ("büyüktür",      BramaKeywordType::GreaterThan),
    ("buyuktur",      BramaKeywordType::GreaterThan),
    ("büyükeşittir",  BramaKeywordType::GreaterEqualThan),
    ("buyukesittir",  BramaKeywordType::GreaterEqualThan),
    ("küçüktür",      BramaKeywordType::LessThan),
    ("kucuktur",      BramaKeywordType::LessThan),
    ("küçükeşittir",  BramaKeywordType::LessEqualThan),
    ("kucukesittir",  BramaKeywordType::LessEqualThan),
    ("değil",         BramaKeywordType::Not),
    ("degil",         BramaKeywordType::Not),
    ("fonk",            BramaKeywordType::Fn),
    ("döndür",        BramaKeywordType::Return),
    ("dondur",        BramaKeywordType::Return),
    ("kır",           BramaKeywordType::Break),
    ("kir",           BramaKeywordType::Break),
    ("devam",       BramaKeywordType::Continue),
    ("döngü",         BramaKeywordType::WhileStartPart),
    ("dongu",         BramaKeywordType::WhileStartPart),
    ("iken",          BramaKeywordType::WhileEndPart)
];

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaOperatorType {
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

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaTokenType {
    Integer(i64),
    Double(f64),
    Symbol(Rc<String>),
    Operator(BramaOperatorType),
    Text(Rc<String>),
    Keyword(BramaKeywordType),
    WhiteSpace(u8),
    NewLine(u8)
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaNumberSystem {
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
    pub token_type: BramaTokenType
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

    pub fn add_token(&mut self, start: u32, token_type: BramaTokenType) {
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
    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<(), BramaErrorType>;
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

impl BramaTokenType {

    pub fn is_symbol(&self) -> bool {
        match self {
            BramaTokenType::Symbol(_) => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn is_keyword(&self) -> bool {
        match self {
            BramaTokenType::Keyword(_) => true,
            _ => false
        }
    }

    pub fn get_symbol(&self) -> String {
        match self {
            BramaTokenType::Symbol(string) => string.to_string(),
            _ => String::from("")
        }
    }

    pub fn get_keyword(&self) -> BramaKeywordType {
        match self {
            BramaTokenType::Keyword(keyword) => *keyword,
            _ => BramaKeywordType::None
        }
    }
}
