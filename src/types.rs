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
    NewLine
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
    MissingStringDemininator(i32, i32),
    MultipleDotOnDouble(i32, i32)
}

pub struct Token {
    pub line      : i32,
    pub column    : i32,
    pub token_type: BramaTokenType
}

#[derive(Default)]
pub struct Tokinizer<'a> {
    pub data  : &'a str,
    pub length: i32,
    pub line  : i32,
    pub column: i32,
    pub index : i32,
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_new_line() {
        assert_eq!(true, '\n'.is_new_line());
        assert_eq!(false, ' '.is_new_line());
    }

    #[test]
    fn is_integer() {
        for ch in '0'..'9' {
            assert_eq!(true, ch.is_integer());
        }
        assert_eq!(false, 'a'.is_integer());
    }

    #[test]
    fn is_symbol() {
        assert_eq!(true, '_'.is_symbol());
        for ch in 'a'..'z' {
            assert_eq!(true, ch.is_symbol());
        }
        for ch in 'A'..'Z' {
            assert_eq!(true, ch.is_symbol());
        }
    }

    #[test]
    fn is_whitespace() {
        assert_eq!(true, ' '.is_whitespace());
        assert_eq!(true, '\r'.is_whitespace());
        assert_eq!(true, '\t'.is_whitespace());
        assert_eq!(false, '2'.is_whitespace());
    }
}