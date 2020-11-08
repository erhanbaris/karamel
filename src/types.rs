use std::default::Default;
use std::vec::Vec;

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
    Or
}

pub static KEYWORDS: [(&str, BramaKeywordType); 18] = [
    ("true",  BramaKeywordType::True),
    ("false", BramaKeywordType::False),
    ("use",   BramaKeywordType::Use),
    ("until", BramaKeywordType::Until),
    ("loop",  BramaKeywordType::Loop),
    ("if",    BramaKeywordType::If),
    ("else",  BramaKeywordType::Else),
    ("and",   BramaKeywordType::And),
    ("or",    BramaKeywordType::Or),

    ("doğru",  BramaKeywordType::True),
    ("yanlış", BramaKeywordType::False),
    ("kullan", BramaKeywordType::Use),
    ("kadar",  BramaKeywordType::Until),
    ("döngü",  BramaKeywordType::Loop),
    ("eğer",   BramaKeywordType::If),
    ("yada",   BramaKeywordType::Else),
    ("ve",     BramaKeywordType::And),
    ("veya",   BramaKeywordType::Or)
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
    AssignModulus,
    Equal,
    EqualValue,
    NotEqual,
    NotEqualValue,
    Not,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    BitwiseUnsignedRightShift,
    GreaterThan,
    LessThan,
    GreaterEqualThan,
    LessEqualThan,
    QuestionMark,
    ColonMark,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
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
    None,
    Integer(i64),
    Double(f64),
    Symbol(String),
    Operator(BramaOperatorType),
    Text(String),
    Keyword(BramaKeywordType),
    WhiteSpace(u8),
    NewLine(u8)
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaNumberSystem {
    None        = 0,
    Binary      = 1,
    Octal       = 2,
    Decimal     = 3,
    Hexadecimal = 4
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaStatus {
    Ok,
    Error(String, u32, u32),
}

pub struct Token {
    pub line      : u32,
    pub column    : u32,
    pub token_type: BramaTokenType
}

#[derive(Default)]
pub struct Tokinizer {
    pub data  : &'static str,
    pub length: u32,
    pub line  : u32,
    pub column: u32,
    pub index : u32,
    pub tokens: Vec<Token>
}


#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaPrimative {
    None,
    Integer(i64),
    Double(f64),
    Bool(bool),
    List(Vec<i32>),
    Atom(String),
    String(String)
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaAstType {
    None,
    Primative(BramaPrimative),
    Binary,
    Control,
    Unary,
    Assign,
    Loop,
    IfStatement,
    Symbol
}


pub struct BramaAst {
    ast_type: BramaAstType
}

impl Tokinizer {
    pub fn is_end(&self) -> bool {
        self.length <= self.index
    }

    pub fn get_char(&self) -> char {
        if !self.is_end() {
            return self.data.chars().nth(self.index as usize).unwrap_or('\0');
        }
        return '\0';
    }

    pub fn get_next_char(&self) -> char {
        if self.length > self.index + 1 {
            return self.data.chars().nth((self.index + 1) as usize).unwrap_or('\0');
        }
        return '\0';
    }

    pub fn get_third_char(&self) -> char {
        if self.length > self.index + 2 {
            return self.data.chars().nth((self.index + 2) as usize).unwrap_or('\0');
        }
        return '\0';
    }

    pub fn add_token(&mut self, token: Token) {
        self.column = 0;
        self.tokens.push(token);
    }

    pub fn increase_index(&mut self) {
        self.index += 1;
    }

    pub fn increate_line(& mut self) {
        self.line += 1;
    }

    pub fn reset_column(& mut self) {
        self.column = 0;
    }
}

pub trait TokenParser {
    fn check(&self, tokinizer: &Tokinizer) -> bool;
    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<BramaTokenType, (String, u32, u32)>;
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
        return self.is_alphabetic() || *self == '_' ||  *self == '$';
    }

    fn is_integer(&self) -> bool {
        match *self {
            '0'..='9' => true,
            _ => false,
        }
    }
}

impl Token {
    pub fn is_primative(&self) -> bool {
        return match &self.token_type {
            BramaTokenType::Integer(num) => true,
            BramaTokenType::Double(num) => true,
            BramaTokenType::Text(s) => true,
            BramaTokenType::Keyword(keyword) => {
                *keyword == BramaKeywordType::True || *keyword == BramaKeywordType::False
            },
            BramaTokenType::WhiteSpace(c) => false,
            BramaTokenType::NewLine(c) => false,
            BramaTokenType::Symbol(sym) => false,
            BramaTokenType::Operator(opt) => false,
            BramaTokenType::None => false
        };
    }
}