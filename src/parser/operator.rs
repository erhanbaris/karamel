use std::collections::HashMap;
use crate::types::*;

pub struct OperatorParser;

impl TokenParser for OperatorParser {
    fn check(&self, tokinizer: &Tokinizer<'_>) -> bool {
        true
    }

    fn parse(&self, tokinizer: &mut Tokinizer<'_>) -> Result<BramaTokenType, (String, u32, u32)> {
        let ch       = tokinizer.get_char();
        let ch_next  = tokinizer.get_next_char();
        let ch_third = tokinizer.get_third_char();

        tokinizer.increase_index();

        let mut operator_type = match (ch, ch_next, ch_third) {
            ('=', '=', '=') => BramaOperatorType::EqualValue,
            ('!', '=', '=') => BramaOperatorType::NotEqualValue,
            _ =>  BramaOperatorType::None
        };

        if operator_type != BramaOperatorType::None {
            tokinizer.increase_index();
            tokinizer.increase_index();
        }
        else {
            operator_type = match (ch, ch_next) {
                ('!', '=') => BramaOperatorType::NotEqual,
                ('/', '=') => BramaOperatorType::AssignDivision,
                ('/', '/') => BramaOperatorType::CommentLine,
                ('/', '*') => BramaOperatorType::CommentMultilineStart,
                ('+', '+') => BramaOperatorType::Increment,
                ('+', '=') => BramaOperatorType::AssignAddition,
                ('-', '-') => BramaOperatorType::Deccrement,
                ('-', '=') => BramaOperatorType::AssignSubtraction,
                ('<', '=') => BramaOperatorType::LessEqualThan,
                ('<', '<') => BramaOperatorType::BitwiseLeftShift,
                ('&', '&') => BramaOperatorType::And,
                ('&', '=') => BramaOperatorType::BitwiseAndAssign,
                ('|', '|') => BramaOperatorType::Or,
                ('|', '=') => BramaOperatorType::BitwiseOrAssign,
                ('*', '=') => BramaOperatorType::AssignMultiplication,
                ('*', '/') => BramaOperatorType::CommentMultilineEnd,
                ('=', '=') => BramaOperatorType::Equal,
                ('%', '=') => BramaOperatorType::AssignModulus,
                ('^', '=') => BramaOperatorType::BitwiseXorAssign,
                _ =>  BramaOperatorType::None
            };

            if operator_type != BramaOperatorType::None {
                tokinizer.increase_index();
            }
            else {
                operator_type = match ch {
                    '^' => BramaOperatorType::BitwiseXor,
                    '%' => BramaOperatorType::Modulo,
                    '!' => BramaOperatorType::Not,
                    '=' => BramaOperatorType::Assign,
                    '*' => BramaOperatorType::Multiplication,
                    '|' => BramaOperatorType::BitwiseOr,
                    '&' => BramaOperatorType::BitwiseAnd,
                    '<' => BramaOperatorType::LessThan,
                    '-' => BramaOperatorType::Subtraction,
                    '+' => BramaOperatorType::Addition,
                    '/' => BramaOperatorType::Division,
                    '?' => BramaOperatorType::QuestionMark,
                    ':' => BramaOperatorType::ColonMark,
                    '~' => BramaOperatorType::BitwiseNot,
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
        }

        if operator_type == BramaOperatorType::None {
            return Err((String::from("Char not valid"), tokinizer.line, tokinizer.column));
        }

        return Ok(BramaTokenType::Operator(operator_type));
    }
}