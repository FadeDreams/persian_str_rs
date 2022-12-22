## Persian String Library for Rust

This library provides functions for working with Persian strings in Rust. It includes functions to convert Arabic digits to Persian digits, and vice versa, as well as functions to filter and normalize Arabic and Persian text.

### Installation
To use this library in your Rust project, add the following to your Cargo.toml:
```
[dependencies]
persian_str_rs = "0.1.0"
```
Then, add the following to the top of your Rust file:
```
extern crate persian_str_rs;
use persian_str_rs::*;
```
###Usage
#### Converting between Arabic and Persian digits
To convert a string from Arabic to Persian digits, use the transform_to_persian_digits function:
```
let text = "1234567890";
let persian_text = transform_to_persian_digits(text);
// persian_text is now "۱۲۳۴۵۶۷۸۹۰"
```
To convert a string from Persian to Arabic digits, use the transform_to_english_digits function:
```
let text = "۱۲۳۴۵۶۷۸۹۰";
let arabic_text = transform_to_english_digits(text);
// arabic_text is now "1234567890"
```
You can also use the transform_int_to_persian_digits function to convert an integer value to a string with Persian digits:
```rust
let value = 12345;
let persian_text = transform_int_to_persian_digits(value);
// persian_text is now "۱۲۳۴۵"
```
#### Filtering text
To filter a string and only keep the English digits, use the filter_english_numbers function:
```
let text = "This string has 123 numbers and ۱۲۳ Persian numbers.";
let filtered_text = filter_english_numbers(text);
// filtered_text is now "123"
```
To filter a string and only keep the Arabic and Persian digits, use the numeric_only function:

```
let text = "This string has 123 numbers and ۱۲۳ Persian numbers.";
let filtered_text = numeric_only(text);
// filtered_text is now "123۱۲۳"
```
To filter a string and only keep the Persian alphabet, use the filter_non_persian_alphabet function:
```
let text = "This string has English and Persian characters: روز خوبی برایتان!";
let filtered_text = filter_non_pers
```
