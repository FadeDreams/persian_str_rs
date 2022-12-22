use regex::Regex;

pub fn transform_to_persian_digits(text: &str) -> String {
    text.replace("0", "۰")
        .replace("1", "۱")
        .replace("2", "۲")
        .replace("3", "۳")
        .replace("4", "۴")
        .replace("5", "۵")
        .replace("6", "۶")
        .replace("7", "۷")
        .replace("8", "۸")
        .replace("9", "۹")
}

pub fn transform_int_to_persian_digits(value: i32) -> String {
    transform_to_persian_digits(&format!("{}", value))
}

pub fn transform_to_english_digits(text: &str) -> String {
    text.replace("۰", "0")
        .replace("۱", "1")
        .replace("۲", "2")
        .replace("۳", "3")
        .replace("۴", "4")
        .replace("۵", "5")
        .replace("۶", "6")
        .replace("۷", "7")
        .replace("۸", "8")
        .replace("۹", "9")
}

pub fn filter_english_numbers(text: &str) -> String {
    let re = regex::Regex::new(r"[^0-9.]").unwrap();
    re.replace_all(text, "").to_string()
}

pub fn filter_persian_numbers(text: &str) -> String {
    //let re = Regex::new(r"[^۰-۹.]").unwrap();
    let re = Regex::new(r"[^۰۱۲۳۴۵۶۷۸۹]").unwrap();
    //let re = Regex::new(r"[^\p{Nd}]").unwrap();
    re.replace_all(text, "").to_string()
}

pub fn numeric_only(text: &str) -> String {
    let re = Regex::new(r"[^۰-۹0-9.]").unwrap();
    re.replace_all(text, "").to_string()
}

pub fn filter_non_persian_alphabet(text: &str) -> String {
    let re = Regex::new(r"[^\u{0600}-\u{06FF}.]").unwrap();
    re.replace_all(text, "").to_string()
}

pub fn normalize_arabic(text: &str) -> String {
    let replacements = [
        ("ي", "ی"),
        ("ك", "ک"),
        ("دِ", "د"),
        ("بِ", "ب"),
        ("زِ", "ز"),
        ("ذِ", "ذ"),
        ("ِشِ", "ش"),
        ("ِسِ", "س"),
        ("ى", "ی"),
    ];

    let mut result = text.to_owned();
    for (from, to) in replacements.iter() {
        result = result.replace(from, to);
    }
    result
}


