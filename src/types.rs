use std::vec::{Vec};
use std::str::Chars;
use std::iter::Peekable;
use std::cell::Cell;
use std::result::Result;
use std::string::String;
use std::collections::HashMap;

pub type ParseResult        = Result<(), (&'static str, u32, u32)>;
pub type AstResult          = Result<BramaAstType, (&'static str, u32, u32)>;
pub type CompilerResult     = Result<(), (&'static str, u32, u32)>;
pub type ParseType          = fn(parser: &SyntaxParser) -> AstResult;

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
    Not
}

impl BramaKeywordType {
    pub fn to_operator(&self) -> BramaOperatorType {
        match &self {
            BramaKeywordType::And => BramaOperatorType::And,
            BramaKeywordType::Or => BramaOperatorType::Or,
            BramaKeywordType::Modulo => BramaOperatorType::Modulo,
            BramaKeywordType::Not => BramaOperatorType::Not,
            _ => BramaOperatorType::None
        }
    }
}

pub static KEYWORDS: [(&str, BramaKeywordType); 23] = [
    ("true",   BramaKeywordType::True),
    ("false",  BramaKeywordType::False),
    ("use",    BramaKeywordType::Use),
    ("until",  BramaKeywordType::Until),
    ("loop",   BramaKeywordType::Loop),
    ("if",     BramaKeywordType::If),
    ("else",   BramaKeywordType::Else),
    ("and",    BramaKeywordType::And),
    ("or",     BramaKeywordType::Or),
    ("empty",  BramaKeywordType::Empty),
    ("not",    BramaKeywordType::Not),

    ("doğru",  BramaKeywordType::True),
    ("yanlış", BramaKeywordType::False),
    ("kullan", BramaKeywordType::Use),
    ("kadar",  BramaKeywordType::Until),
    ("döngü",  BramaKeywordType::Loop),
    ("eğer",   BramaKeywordType::If),
    ("yada",   BramaKeywordType::Else),
    ("ve",     BramaKeywordType::And),
    ("veya",   BramaKeywordType::Or),
    ("yok",    BramaKeywordType::Empty),
    ("mod",    BramaKeywordType::Modulo),
    ("değil",  BramaKeywordType::Not)
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
    Binary      = 0,
    Octal       = 1,
    Decimal     = 2,
    Hexadecimal = 3
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line      : u32,
    pub column    : u32,
    pub token_type: BramaTokenType
}

pub struct Tokinizer {
    pub line  : u32,
    pub column: u32,
    pub tokens: Vec<Token>,
    pub iter: Peekable<Chars<'static>>,
    pub iter_second: Peekable<Chars<'static>>,
    pub iter_third: Peekable<Chars<'static>>,
}

pub struct SyntaxParser {
    pub tokens: Box<Vec<Token>>,
    pub index: Cell<usize>,
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaPrimative {
    Empty,
    Integer(i64),
    Double(f64),
    Bool(bool),
    List(Vec<Box<BramaAstType>>),
    Atom(String),
    Text(String)
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaAstType {
    None,
    Primative(BramaPrimative),
    Binary {
        left: Box<BramaAstType>, 
        operator: BramaOperatorType, 
        right: Box<BramaAstType>
    },
    Control {
        left: Box<BramaAstType>, 
        operator: BramaOperatorType, 
        right: Box<BramaAstType>
    },
    /*Control,*/
    PrefixUnary(BramaOperatorType, Box<BramaAstType>),
    SuffixUnary(BramaOperatorType, Box<BramaAstType>),
    /*
    Assign,
    Loop,
    IfStatement,*/
    Symbol(String)
}

impl Tokinizer {
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

    pub fn get_third_char(&mut self) -> char {
        return match self.iter_third.peek() {
            Some(&c) => c,
            None => '\0'
        };
    }

    pub fn add_token(&mut self, token: Token) {
        self.column = 0;
        self.tokens.push(token);
    }

    pub fn increase_index(&mut self) {
        self.iter.next();
        self.iter_second.next();
        self.iter_third.next();
    }

    pub fn increate_line(& mut self) {
        self.line += 1;
    }

    pub fn reset_column(& mut self) {
        self.column = 0;
    }
}

pub trait TokenParser {
    fn check(&self, tokinizer: &mut Tokinizer) -> bool;
    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<BramaTokenType, (&'static str, u32, u32)>;
}

pub trait SyntaxParserTrait {
    type Item;
    fn parse(parser: &SyntaxParser) -> AstResult;
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

impl BramaTokenType {
    pub fn is_integer(&self) -> bool {
        return match self {
            BramaTokenType::Integer(_) => true,
            _ => false
        }
    }

    pub fn is_double(&self) -> bool {
        return match self {
            BramaTokenType::Double(_) => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn is_operator(&self) -> bool {
        return match self {
            BramaTokenType::Operator(_) => true,
            _ => false
        }
    }

    pub fn is_bool(&self) -> bool {
        if self.is_keyword() {
            return match self {
                BramaTokenType::Keyword(BramaKeywordType::True) => true,
                BramaTokenType::Keyword(BramaKeywordType::False) => true,
                _ => false
            }
        }
        return false;        
    }

    pub fn is_symbol(&self) -> bool {
        return match self {
            BramaTokenType::Symbol(_) => true,
            _ => false
        }
    }

    pub fn is_keyword(&self) -> bool {
        return match self {
            BramaTokenType::Keyword(_) => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn is_text(&self) -> bool {
        return match self {
            BramaTokenType::Text(_) => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn is_whitespace(&self) -> bool {
        return match self {
            BramaTokenType::WhiteSpace(_) => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn is_newline(&self) -> bool {
        return match self {
            BramaTokenType::NewLine(_) => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn get_integer(&self) -> i64 {
        return match self {
            BramaTokenType::Integer(integer) => *integer,
            _ => 0
        }
    }

    #[allow(dead_code)]
    pub fn get_double(&self) -> f64 {
        return match self {
            BramaTokenType::Double(double) => *double,
            _ => 0.0
        }
    }

    #[allow(dead_code)]
    pub fn get_operator(&self) -> BramaOperatorType {
        return match self {
            BramaTokenType::Operator(operator) => *operator,
            _ => BramaOperatorType::None
        }
    }

    pub fn get_symbol(&self) -> &str {
        return match self {
            BramaTokenType::Symbol(string) => string,
            _ => ""
        }
    }

    #[allow(dead_code)]
    pub fn get_keyword(&self) -> BramaKeywordType {
        return match self {
            BramaTokenType::Keyword(keyword) => *keyword,
            _ => BramaKeywordType::None
        }
    }

    #[allow(dead_code)]
    pub fn get_text(&self) -> &str {
        return match self {
            BramaTokenType::Text(string) => string,
            _ => ""
        }
    }

    #[allow(dead_code)]
    pub fn get_whitespace(&self) -> u8 {
        return match self {
            BramaTokenType::WhiteSpace(count) => *count,
            _ => 0
        }
    }

    #[allow(dead_code)]
    pub fn get_newline(&self) -> u8 {
        return match self {
            BramaTokenType::NewLine(count) => *count,
            _ => 0
        }
    }

    #[allow(dead_code)]
    pub fn get_bool(&self) -> bool {
        if self.is_keyword() {
            return match self {
                BramaTokenType::Keyword(BramaKeywordType::True) => true,
                BramaTokenType::Keyword(BramaKeywordType::False) => false,
                _ => false
            }
        }
        return false; 
    }
}

#[derive(PartialEq, Debug)]
pub enum VmObjectType {
    Empty,
    Atom(u64),
    Integer(i64),
    Double(f64),
    Text(String),
    Bool(bool),
    List(Vec<Box<BramaAstType>>)
}

#[derive(PartialEq, Debug)]
pub struct VmObject {
    pub marked: bool,
    pub data  : VmObjectType
}

#[repr(C)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaVmOpCode {
    None,
    Halt,
    Addition   {target: i16, left: i16, right: i16},
    Subraction {target: i16, left: i16, right: i16},
    Multiply   {target: i16, left: i16, right: i16},
    Divition   {target: i16, left: i16, right: i16},
    Modulo     {target: i16, left: i16, right: i16},
    And        {target: i16, left: i16, right: i16},
    Or         {target: i16, left: i16, right: i16}
}

#[derive(PartialEq, Debug)]
pub struct InnerStrorage {
    pub return_back_address   : u32,
    pub return_back_variable  : u32,
    pub const_variables       : Vec<VmObjectType>,
    pub temporary_variables   : u16,
    pub variables             : HashMap<String, VmObjectType>,
    pub memory                : Vec<VmObjectType>,
    pub temp_counter          : i16,
    pub total_const_variables : i16
}

pub struct BramaCompilerOption {
    pub opcodes : Vec<BramaVmOpCode>,
    pub storages: Vec<InnerStrorage>
}