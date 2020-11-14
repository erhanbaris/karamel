use crate::types::*;

pub struct OperatorParser;

impl TokenParser for OperatorParser {
    fn check(&self, _tokinizer: &mut Tokinizer) -> bool {
        true
    }

    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<BramaTokenType, (&'static str, u32, u32)> {
        let ch       = tokinizer.get_char();
        let ch_next  = tokinizer.get_next_char();
        let ch_third = tokinizer.get_third_char();

        tokinizer.increase_index();

        let mut operator_type = match (ch, ch_next) {
            ('!', '=') => BramaOperatorType::NotEqual,
            ('/', '=') => BramaOperatorType::AssignDivision,
            ('/', '/') => BramaOperatorType::CommentLine,
            ('/', '*') => BramaOperatorType::CommentMultilineStart,
            ('+', '+') => BramaOperatorType::Increment,
            ('+', '=') => BramaOperatorType::AssignAddition,
            ('-', '-') => BramaOperatorType::Deccrement,
            ('-', '=') => BramaOperatorType::AssignSubtraction,
            ('<', '=') => BramaOperatorType::LessEqualThan,
            ('>', '=') => BramaOperatorType::GreaterEqualThan,
            ('*', '=') => BramaOperatorType::AssignMultiplication,
            ('*', '/') => BramaOperatorType::CommentMultilineEnd,
            ('=', '=') => BramaOperatorType::Equal,
            _ =>  BramaOperatorType::None
        };

        if operator_type != BramaOperatorType::None {
            tokinizer.increase_index();
        }
        else {
            operator_type = match ch {
                '=' => BramaOperatorType::Assign,
                '*' => BramaOperatorType::Multiplication,
                '<' => BramaOperatorType::LessThan,
                '>' => BramaOperatorType::GreaterThan,
                '-' => BramaOperatorType::Subtraction,
                '+' => BramaOperatorType::Addition,
                '/' => BramaOperatorType::Division,
                '?' => BramaOperatorType::QuestionMark,
                ':' => BramaOperatorType::ColonMark,
                '(' => BramaOperatorType::LeftParentheses,
                ')' => BramaOperatorType::RightParentheses,
                '[' => BramaOperatorType::SquareBracketStart,
                ']' => BramaOperatorType::SquareBracketEnd,
                '{' => BramaOperatorType::CurveBracketStart,
                '}' => BramaOperatorType::CurveBracketEnd,
                ',' => BramaOperatorType::Comma,
                ';' => BramaOperatorType::Semicolon,
                '.' => BramaOperatorType::Dot,
                _ => BramaOperatorType::None
            };
        }

        if operator_type == BramaOperatorType::None {
            return Err(("Char not valid", tokinizer.line, tokinizer.column));
        }

        return Ok(BramaTokenType::Operator(operator_type));
    }
}