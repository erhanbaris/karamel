extern crate TPD;

#[cfg(test)]
mod tests {
    use crate::TPD::types::CharTraits;

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

        assert_eq!(true, '$'.is_symbol());

    }

    #[test]
    fn is_whitespace() {
        assert_eq!(true, ' '.is_whitespace());
        assert_eq!(true, '\r'.is_whitespace());
        assert_eq!(true, '\t'.is_whitespace());
        assert_eq!(false, '2'.is_whitespace());
    }
}