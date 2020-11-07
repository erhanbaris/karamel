use crate::types::*;

pub struct NumberParser;

impl NumberParser {
    fn increase(&mut self, tokinizer: &mut Tokinizer<'_>) -> char {
        tokinizer.increase_index();
        return tokinizer.get_char();
    }

    fn get_digits(&mut self, tokinizer: &mut Tokinizer<'_>) -> (u8, u64) {
        let mut number: u64   = 0;
        let mut num_count: u8 = 0;
        let mut ch :char      = tokinizer.get_char();

        while !tokinizer.is_end() && ch.is_ascii_digit() {
            num_count += 1;

            number *= u64::pow(10, 1);
            number += ch as u64 - '0' as u64;

            ch = self.increase(tokinizer);
        }

        return (num_count, number);
    }

    fn detect_number_system(&mut self, tokinizer: &mut Tokinizer<'_>) -> BramaNumberSystem {
        if tokinizer.get_char() == '0' {
            return match tokinizer.get_next_char() {
                'b' | 'B' => {
                    self.increase(tokinizer);
                    self.increase(tokinizer);
                    BramaNumberSystem::Binary
                },
                'x' | 'X' => {
                    self.increase(tokinizer);
                    self.increase(tokinizer);
                    BramaNumberSystem::Hexadecimal
                },
                '0'..='7' => {
                    self.increase(tokinizer);
                    BramaNumberSystem::Octal
                },
                _ => BramaNumberSystem::Decimal
            };
        }

        return BramaNumberSystem::Decimal;
    }

    fn parse_hex(&mut self, tokinizer: &mut Tokinizer<'_>) -> BramaTokenType {
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

        return BramaTokenType::Integer(number as i64);
    }

    fn parse_octal(&mut self, tokinizer: &mut Tokinizer<'_>) -> BramaTokenType {
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

        return BramaTokenType::Integer(number as i64);
    }

    fn parse_binary(&mut self, tokinizer: &mut Tokinizer<'_>) -> BramaTokenType {
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

        return BramaTokenType::Integer(number as i64);
    }

    fn parse_decimal(&mut self, tokinizer: &mut Tokinizer<'_>) -> BramaTokenType {
        /*
        [NUMBER](.[NUMBER](E(-+)[NUMBER]))
        */
        let mut before_comma: u64 = 0;
        let mut is_double         = false;
        let mut ch :char          = tokinizer.get_char();

        let (_, digits) = self.get_digits(tokinizer);
        before_comma    = digits;
        ch              = tokinizer.get_char();

        /* Double number */
        if !tokinizer.is_end() && ch == '.' {
            let mut after_comma   = 0;
            let mut dot_place: u8 = 0;
            is_double             = true;

            ch = self.increase(tokinizer);

            let (digit_num, digits) = self.get_digits(tokinizer);
            after_comma = digits;
            dot_place   = digit_num;
            ch          = tokinizer.get_char();

            if !tokinizer.is_end() && (ch == 'e' || ch == 'E') {
                let mut is_minus      = false;
                let mut e_after: u64  = 0;

                ch = self.increase(tokinizer);

                if !tokinizer.is_end() {
                    match ch {
                        '-' => {
                            is_minus = true;
                            ch = self.increase(tokinizer);
                        },

                        '+' => ch = self.increase(tokinizer),
                        _ => {}
                    }
                }

                let (_, digits) = self.get_digits(tokinizer);
                e_after    = digits;
                ch = self.increase(tokinizer);

                let num = before_comma as f64 + (after_comma as f64 * f64::powi(10.0, -1 * dot_place as i32));

                return match is_minus {
                    true  => BramaTokenType::Double(num / f64::powi(10.0, e_after as i32)),
                    false => BramaTokenType::Double(num * f64::powi(10.0, e_after as i32))
                }
            }

            let num = before_comma as f64 + (after_comma as f64 * f64::powi(10.0, -1 * dot_place as i32));
            return BramaTokenType::Double(num)
        }

        return BramaTokenType::Integer(before_comma as i64);
    }
}

impl TokenParser for NumberParser {
    fn parse(&mut self, tokinizer: &mut Tokinizer<'_>) -> Result<BramaTokenType, (String, u32, u32)> {
        let number_system = self.detect_number_system(tokinizer);

        return match number_system {
            BramaNumberSystem::Binary      => Ok(self.parse_binary(tokinizer)),
            BramaNumberSystem::Octal       => Ok(self.parse_octal(tokinizer)),
            BramaNumberSystem::Decimal     => Ok(self.parse_decimal(tokinizer)),
            BramaNumberSystem::Hexadecimal => Ok(self.parse_hex(tokinizer)),
            _ => Err((String::from("Number not parsed"), tokinizer.line, tokinizer.column))
        };
    }

    fn validate(&mut self, tokinizer: &mut Tokinizer<'_>) -> BramaStatus {
        BramaStatus::Ok
    }
}