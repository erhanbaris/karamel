use crate::types::*;
use crate::error::KaramelErrorType;

pub struct OperatorParser;

impl TokenParser for OperatorParser {
    fn check(&self, _tokinizer: &mut Tokinizer) -> bool {
        true
    }

    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<(), KaramelErrorType> {
        let ch       = tokinizer.get_char();
        let ch_next  = tokinizer.get_next_char();
        let start= tokinizer.column;
        
        tokinizer.increase_index();

        let mut operator_type = match (ch, ch_next) {
            ('!', '=') => KaramelOperatorType::NotEqual,
            ('/', '=') => KaramelOperatorType::AssignDivision,
            ('/', '/') => KaramelOperatorType::CommentLine,
            ('/', '*') => KaramelOperatorType::CommentMultilineStart,
            ('+', '+') => KaramelOperatorType::Increment,
            ('+', '=') => KaramelOperatorType::AssignAddition,
            ('-', '-') => KaramelOperatorType::Deccrement,
            ('-', '=') => KaramelOperatorType::AssignSubtraction,
            ('<', '=') => KaramelOperatorType::LessEqualThan,
            ('>', '=') => KaramelOperatorType::GreaterEqualThan,
            ('*', '=') => KaramelOperatorType::AssignMultiplication,
            ('*', '/') => KaramelOperatorType::CommentMultilineEnd,
            ('=', '=') => KaramelOperatorType::Equal,
            _ =>  KaramelOperatorType::None
        };

        if operator_type != KaramelOperatorType::None {
            tokinizer.increase_index();
        }
        else {
            operator_type = match ch {
                '=' => KaramelOperatorType::Assign,
                '*' => KaramelOperatorType::Multiplication,
                '<' => KaramelOperatorType::LessThan,
                '>' => KaramelOperatorType::GreaterThan,
                '-' => KaramelOperatorType::Subtraction,
                '+' => KaramelOperatorType::Addition,
                '/' => KaramelOperatorType::Division,
                '?' => KaramelOperatorType::QuestionMark,
                ':' => KaramelOperatorType::ColonMark,
                '(' => KaramelOperatorType::LeftParentheses,
                ')' => KaramelOperatorType::RightParentheses,
                '[' => KaramelOperatorType::SquareBracketStart,
                ']' => KaramelOperatorType::SquareBracketEnd,
                '{' => KaramelOperatorType::CurveBracketStart,
                '}' => KaramelOperatorType::CurveBracketEnd,
                ',' => KaramelOperatorType::Comma,
                ';' => KaramelOperatorType::Semicolon,
                '.' => KaramelOperatorType::Dot,
                '!' => KaramelOperatorType::Not,
                _ => KaramelOperatorType::None
            };
        }

        if ch == '\r' {
            return Ok(());
        }

        if operator_type == KaramelOperatorType::None {
            log::debug!("'{}' not found", ch as usize);
            return Err(KaramelErrorType::CharNotValid);
        }
        
        tokinizer.add_token(start, KaramelTokenType::Operator(operator_type));
        return Ok(());
    }
}