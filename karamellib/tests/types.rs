extern crate karamellib;

#[cfg(test)]
mod tests {
    use crate::karamellib::types::CharTraits;

    #[test]
    fn is_new_line() {
        assert!('\n'.is_new_line());
        assert!(!' '.is_new_line());
    }

    #[test]
    fn is_integer() {
        for ch in '0'..'9' {
            assert!(ch.is_integer());
        }
        assert!(!'a'.is_integer());
    }

    #[test]
    fn is_symbol() {
        assert!('_'.is_symbol());
        for ch in 'a'..'z' {
            assert!(ch.is_symbol());
        }
        for ch in 'A'..'Z' {
            assert!(ch.is_symbol());
        }

        assert!('$'.is_symbol());
    }

    #[test]
    fn is_whitespace() {
        assert!(' '.is_whitespace());
        assert!('\r'.is_whitespace());
        assert!('\t'.is_whitespace());
        assert!(!'2'.is_whitespace());
    }
}
