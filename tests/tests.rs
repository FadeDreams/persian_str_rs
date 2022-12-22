extern crate persian_str_rs;
use persian_str_rs::{
    transform_to_persian_digits, transform_int_to_persian_digits,
    transform_to_english_digits, filter_english_numbers,
    filter_persian_numbers, numeric_only,
    filter_non_persian_alphabet, normalize_arabic

};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_to_persian_digits() {
        assert_eq!(transform_to_persian_digits("1234567890"), "۱۲۳۴۵۶۷۸۹۰");
        assert_eq!(transform_to_persian_digits("0123456789"), "۰۱۲۳۴۵۶۷۸۹");
        assert_eq!(transform_to_persian_digits("9876543210"), "۹۸۷۶۵۴۳۲۱۰");
        assert_eq!(transform_to_persian_digits("abc123def456"), "abc۱۲۳def۴۵۶");
    }

       #[test]
    fn test_transform_int_to_persian_digits() {
        assert_eq!(transform_int_to_persian_digits(1234567890), "۱۲۳۴۵۶۷۸۹۰");
        assert_eq!(transform_int_to_persian_digits(0), "۰");
        assert_eq!(transform_int_to_persian_digits(123), "۱۲۳");
    }

    #[test]
    fn test_transform_to_english_digits() {
        assert_eq!(transform_to_english_digits("۱۲۳۴۵۶۷۸۹۰"), "1234567890");
        assert_eq!(transform_to_english_digits("۰۱۲۳۴۵۶۷۸۹"), "0123456789");
        assert_eq!(transform_to_english_digits("۹۸۷۶۵۴۳۲۱۰"), "9876543210");
        assert_eq!(transform_to_english_digits("abc۱۲۳def۴۵۶"), "abc123def456");
    }

    #[test]
    fn test_filter_english_numbers() {
        assert_eq!(filter_english_numbers("123456"), "123456");
        assert_eq!(filter_english_numbers("123.456"), "123.456");
        assert_eq!(filter_english_numbers("123 456"), "123456");
        assert_eq!(filter_english_numbers("123,456"), "123456");
    }

    #[test]
    fn test_filter_persian_numbers() {
        let input = "۱a";
        let expected_output = "۱";
        assert_eq!(filter_persian_numbers(input), expected_output);
    }

 

    #[test]
    fn test_numeric_only() {
        let input = "12345 abc";
        let expected_output = "12345";
        assert_eq!(numeric_only(input), expected_output);
    }

    #[test]
    fn test_filter_non_persian_alphabet() {
        let input = "سلام aaa";
        let expected_output = "سلام";
        assert_eq!(filter_non_persian_alphabet(input), expected_output);
    }

    #[test]
    fn test_normalize_arabic() {
        let input = "آئیی كabc بپتثجچ ى";
        let expected_output = "آئیی کabc بپتثجچ ی";
        assert_eq!(normalize_arabic(input), expected_output);
    }

}
