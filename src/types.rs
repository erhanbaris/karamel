use std::default::Default;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum BramaKeywordType {
    None=0,
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Int,
    Long,
    Register,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}

pub static KEYWORDS: [(&str, BramaKeywordType); 32] = [
    ("auto", BramaKeywordType::Auto),
    ("break", BramaKeywordType::Break),
    ("case", BramaKeywordType::Case),
    ("char", BramaKeywordType::Char),
    ("const", BramaKeywordType::Const),
    ("continue", BramaKeywordType::Continue),
    ("default", BramaKeywordType::Default),
    ("do", BramaKeywordType::Do),
    ("double", BramaKeywordType::Double),
    ("else", BramaKeywordType::Else),
    ("enum", BramaKeywordType::Enum),
    ("extern", BramaKeywordType::Extern),
    ("float", BramaKeywordType::Float),
    ("for", BramaKeywordType::For),
    ("goto", BramaKeywordType::Goto),
    ("if", BramaKeywordType::If),
    ("int", BramaKeywordType::Int),
    ("long", BramaKeywordType::Long),
    ("register", BramaKeywordType::Register),
    ("return", BramaKeywordType::Return),
    ("short", BramaKeywordType::Short),
    ("signed", BramaKeywordType::Signed),
    ("sizeof", BramaKeywordType::Sizeof),
    ("static", BramaKeywordType::Static),
    ("struct", BramaKeywordType::Struct),
    ("switch", BramaKeywordType::Switch),
    ("typedef", BramaKeywordType::Typedef),
    ("union", BramaKeywordType::Union),
    ("unsigned", BramaKeywordType::Unsigned),
    ("void", BramaKeywordType::Void),
    ("volatile", BramaKeywordType::Volatile),
    ("while", BramaKeywordType::While)
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
    And,
    Or,
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
    EndOfFile,
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
pub struct Tokinizer<'a> {
    pub data  : &'a str,
    pub length: u32,
    pub line  : u32,
    pub column: u32,
    pub index : u32,
    pub tokens: Vec<Token>
}

impl<'a> Tokinizer<'a> {
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
    fn check(&self, tokinizer: &Tokinizer<'_>) -> bool;
    fn parse(&self, tokinizer: &mut Tokinizer<'_>) -> Result<BramaTokenType, (String, u32, u32)>;
    fn validate(&self, tokinizer: &mut Tokinizer<'_>) -> BramaStatus;
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
        match *self {
            'a'..='z' | 'A'..='Z' | '_' => true,
            '$' => true,
            _ => false,
        }
    }

    fn is_integer(&self) -> bool {
        match *self {
            '0'..='9' => true,
            _ => false,
        }
    }
}