use std::vec::{Vec};
use std::str::Chars;
use std::iter::Peekable;
use std::cell::Cell;
use std::result::Result;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::rc::{Rc, Weak};

pub type ParseResult        = Result<(), (&'static str, u32, u32)>;
pub type AstResult          = Result<BramaAstType, (&'static str, u32, u32)>;
pub type CompilerResult     = Result<(), (&'static str, u32, u32)>;
pub type ParseType          = fn(parser: &SyntaxParser) -> AstResult;

pub const TAG_NULL        : u64 = 0;
pub const TAG_FALSE       : u64 = 1;
pub const TAG_TRUE        : u64 = 2;

pub const QNAN:         u64 = 0x7ffc_0000_0000_0000;
pub const POINTER_FLAG: u64 = 0x8000_0000_0000_0000;
pub const POINTER_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;
pub const FALSE_FLAG:   u64 = QNAN | TAG_FALSE;
pub const TRUE_FLAG:    u64 = QNAN | TAG_TRUE;
pub const EMPTY_FLAG:   u64 = QNAN | TAG_NULL;

#[derive(PartialEq, Debug, Hash)]
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
    NotEqual
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

pub static KEYWORDS: &'static [(&str, BramaKeywordType)] = &[
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
    ("equal",       BramaKeywordType::Equal),
    ("notequal",  BramaKeywordType::NotEqual),
    ("greater",      BramaKeywordType::GreaterThan),
    ("greaterequal",  BramaKeywordType::GreaterEqualThan),
    ("less",      BramaKeywordType::LessThan),
    ("lessequal",  BramaKeywordType::LessEqualThan),

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
    ("eşittir",       BramaKeywordType::Equal),
    ("eşitdeğildir",  BramaKeywordType::NotEqual),
    ("büyüktür",      BramaKeywordType::GreaterThan),
    ("büyükeşittir",  BramaKeywordType::GreaterEqualThan),
    ("küçüktür",      BramaKeywordType::LessThan),
    ("küçükeşittir",  BramaKeywordType::LessEqualThan),
    ("değil",         BramaKeywordType::Not)
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
    Symbol(&'static str),
    Operator(BramaOperatorType),
    Text(&'static str),
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
    pub data: &'static str,
    pub index: u32
}

pub struct SyntaxParser {
    pub tokens: Box<Vec<Token>>,
    pub index: Cell<usize>,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum BramaPrimative {
    Empty,
    Number(f64),
    Bool(bool),
    List(Vec<Box<BramaAstType>>),
    Atom(u64),
    Text(String)
}

impl Drop for BramaPrimative {
    fn drop(&mut self) {
        println!("Dropping BramaPrimative!, {:?}", self);
    }
}

impl VmObject {
    pub fn convert(primative: &BramaPrimative) -> VmObject {
        match primative {
            BramaPrimative::Empty            => VmObject(QNAN | EMPTY_FLAG),
            BramaPrimative::Number(number)   => VmObject(number.to_bits()),
            BramaPrimative::Bool(boolean)    => VmObject(QNAN | if *boolean { TRUE_FLAG } else { FALSE_FLAG }),
            _                                => VmObject(QNAN | POINTER_FLAG | (
                POINTER_MASK & (Box::into_raw(Box::new(primative))) as u64
            ))
        }
    }

    pub fn deref(&self) -> BramaPrimative {
        match self.0 {
            n if (n & QNAN) != QNAN       => BramaPrimative::Number(f64::from_bits(n)),
            e if e == (QNAN | EMPTY_FLAG) => BramaPrimative::Empty,
            f if f == (QNAN | FALSE_FLAG) => BramaPrimative::Bool(false),
            t if t == (QNAN | TRUE_FLAG)  => BramaPrimative::Bool(true),
            p if (p & POINTER_FLAG) == POINTER_FLAG => {
                let pointer = (self.0 & POINTER_MASK) as *mut &BramaPrimative;
                Box::leak(unsafe { Box::from_raw(pointer) }).clone()
            },
            _ => BramaPrimative::Empty
        }
    }
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
    Symbol(&'static str)
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

    pub fn add_token(&mut self, token: Token) {
        self.column = 0;
        self.tokens.push(token);
    }

    pub fn increase_index(&mut self) {
        self.iter.next();
        self.iter_second.next();
        self.iter_third.next();
        self.index += self.get_char().len_utf8() as u32;
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

    pub fn get_symbol(&self) -> &'static str {
        return match self {
            BramaTokenType::Symbol(string) => string,
            _ => ""
        }
    }
}

#[repr(C)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaVmOpCode {
    None,
    Addition         {target: i16, left: i16, right: i16},
    Subraction       {target: i16, left: i16, right: i16},
    Multiply         {target: i16, left: i16, right: i16},
    Division         {target: i16, left: i16, right: i16},
    And              {target: i16, left: i16, right: i16},
    Or               {target: i16, left: i16, right: i16},
    Equal            {target: i16, left: i16, right: i16},
    NotEqual         {target: i16, left: i16, right: i16},
    GreaterThan      {target: i16, left: i16, right: i16},
    LessThan         {target: i16, left: i16, right: i16},
    GreaterEqualThan {target: i16, left: i16, right: i16},
    LessEqualThan    {target: i16, left: i16, right: i16}
}

#[derive(PartialEq, Debug)]
pub struct InnerStorage {
    pub constants             : Vec<VmObject>,
    pub constant_size         : u16,
    pub temp_size             : u16,
    pub temp_counter          : u16,
    pub variables             : HashMap<&'static str, u16>,
    pub memory                : Vec<VmObject>,
    pub total_const_variables : u16
}

pub trait Storage {
    /// Build memory block with temporary, constant and variable definitions
    fn build(&mut self);
    fn get_memory(&self) -> &Vec<VmObject>;
    fn get_constant_size(&self) -> u16;
    fn get_variable_size(&self) -> u16;
    fn get_temp_size(&self) -> u16;
    fn get_free_temp_slot(&mut self) -> u16;
    fn set_temp_size(&mut self, value: u16);

    fn get_temp_counter(&self) -> u16;
    fn inc_temp_counter(&mut self);
    fn reset_temp_counter(&mut self);

    fn add_variable(&mut self, name: &'static str);
    fn set_variable_value(&mut self, name: &'static str, object: VmObject);
    fn add_constant(&mut self, object: &BramaPrimative);

    fn get_variable_location(&mut self, name: &'static str) -> Option<u16>;
    fn get_constant_location(&mut self, object: &BramaPrimative) -> Option<u16>;

    fn dump(&self);
}

pub struct BramaCompilerOption {
    pub opcodes : Vec<BramaVmOpCode>,
    pub storages: Vec<InnerStorage>
}

pub trait StrTrait {
    fn atom(&self) -> u64;
}

impl StrTrait for str {
    fn atom(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        return hasher.finish();
    }
}