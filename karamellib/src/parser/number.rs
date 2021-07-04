use crate::types::*;
use crate::error::KaramelErrorType;

pub struct NumberParser;

impl NumberParser {
    fn increase(&self, tokinizer: &mut Tokinizer) -> char {
        tokinizer.increase_index();
        tokinizer.get_char()
    }

    fn get_digits(&self, tokinizer: &mut Tokinizer) -> (u8, u64) {
        let mut number: u64    = 0;
        let mut num_count: u8  = 0;
        let mut ch :char       = tokinizer.get_char();

        while !tokinizer.is_end() && (ch.is_ascii_digit() || ch == '_') {
            if ch != '_' {
                num_count += 1;

                number *= u64::pow(10, 1);
                number += ch as u64 - '0' as u64;
            }

            ch = self.increase(tokinizer);
        }

        (num_count, number)
    }

    fn detect_number_system(&self, tokinizer: &mut Tokinizer) -> KaramelNumberSystem {
        if tokinizer.get_char() == '0' {
            return match tokinizer.get_next_char() {
                'b' | 'B' => {
                    self.increase(tokinizer);
                    self.increase(tokinizer);
                    KaramelNumberSystem::Binary
                },
                'x' | 'X' => {
                    self.increase(tokinizer);
                    self.increase(tokinizer);
                    KaramelNumberSystem::Hexadecimal
                },
                '0'..='7' => {
                    self.increase(tokinizer);
                    KaramelNumberSystem::Octal
                },
                _ => KaramelNumberSystem::Decimal
            };
        }

        return KaramelNumberSystem::Decimal;
    }

    fn parse_hex(&self, tokinizer: &mut Tokinizer) -> KaramelTokenType {
        let mut number :u64 = 0;
        let mut ch :char    = tokinizer.get_char();

        while !tokinizer.is_end() && ch.is_ascii_hexdigit() {
            number = number << 4;

            let tmp_ch = ch.to_digit(16);
            if tmp_ch.is_some() {
                number += tmp_ch.unwrap() as u64;
            }

            ch = self.increase(tokinizer);
        }

        KaramelTokenType::Integer(number as i64)
    }

    fn parse_octal(&self, tokinizer: &mut Tokinizer) -> KaramelTokenType {
        let mut number :u64 = 0;
        let mut ch :char    = tokinizer.get_char();

        while !tokinizer.is_end() && ch >= '0' && ch <= '7' {
            number = number << 3;

            let tmp_ch = ch.to_digit(8);
            if tmp_ch.is_some() {
                number += tmp_ch.unwrap() as u64;
            }

            ch = self.increase(tokinizer);
        }

        KaramelTokenType::Integer(number as i64)
    }

    fn parse_binary(&self, tokinizer: &mut Tokinizer) -> KaramelTokenType {
        let mut number :u64 = 0;
        let mut ch :char    = tokinizer.get_char();

        while !tokinizer.is_end() && ch >= '0' && ch <= '1' {
            number = number << 1;

            let tmp_ch = ch.to_digit(2);
            if tmp_ch.is_some() {
                number += tmp_ch.unwrap() as u64;
            }

            ch = self.increase(tokinizer);
        }

        KaramelTokenType::Integer(number as i64)
    }

    fn parse_decimal(&self, tokinizer: &mut Tokinizer) -> KaramelTokenType {
        /*
        [NUMBER](.[NUMBER](E(-+)[NUMBER]))
        */

        let (_, digits)  = self.get_digits(tokinizer);
        let before_comma = digits;
        let mut ch       = tokinizer.get_char();
        let ch_next = tokinizer.get_next_char();

        /* Double number */
        if !tokinizer.is_end() && ch == '.' && (ch_next >= '0' && ch_next <= '9') {
            self.increase(tokinizer);

            let (digit_num, digits) = self.get_digits(tokinizer);
            let after_comma = digits;
            let dot_place   = digit_num;
            ch          = tokinizer.get_char();

            if !tokinizer.is_end() && (ch == 'e' || ch == 'E') {
                let mut is_minus      = false;

                ch = self.increase(tokinizer);

                if !tokinizer.is_end() {
                    match ch {
                        '-' => {
                            is_minus = true;
                            self.increase(tokinizer);
                        },

                        '+' => { self.increase(tokinizer); },
                        _ => {}
                    }
                }

                let (_, digits) = self.get_digits(tokinizer);
                let e_after    = digits;
                self.increase(tokinizer);

                let num = before_comma as f64 + (after_comma as f64 * f64::powi(10.0, -1 * dot_place as i32));

                return match is_minus {
                    true  => KaramelTokenType::Double(num / f64::powi(10.0, e_after as i32)),
                    false => KaramelTokenType::Double(num * f64::powi(10.0, e_after as i32))
                }
            }

            let num = before_comma as f64 + (after_comma as f64 * f64::powi(10.0, -1 * dot_place as i32));
            return KaramelTokenType::Double(num)
        }

        KaramelTokenType::Integer(before_comma as i64)
    }
}

impl TokenParser for NumberParser {
    fn check(&self, tokinizer: &mut Tokinizer) -> bool {
        let ch = tokinizer.get_char();
        let ch_next = tokinizer.get_next_char();
        (ch == '.' && (ch_next >= '0' && ch_next <= '9')) || (ch >= '0' && ch <= '9')
    }

    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<(), KaramelErrorType> {
        let start_column = tokinizer.column;
        let number_system = self.detect_number_system(tokinizer);

        let token_type = match number_system {
            KaramelNumberSystem::Binary      => self.parse_binary(tokinizer),
            KaramelNumberSystem::Octal       => self.parse_octal(tokinizer),
            KaramelNumberSystem::Decimal     => self.parse_decimal(tokinizer),
            KaramelNumberSystem::Hexadecimal => self.parse_hex(tokinizer)
        };
        tokinizer.add_token(start_column, token_type);
        
        if tokinizer.get_char().is_alphabetic() && !tokinizer.get_char().is_whitespace() {
            return Err(KaramelErrorType::NumberNotParsed);
        }
        Ok(())
    }
}