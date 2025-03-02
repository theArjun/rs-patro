use crate::constants::NEPALI_DIGITS;

pub fn get_digit_in_devanagari(digit: u16) -> String {
    let mut nepali_digit = String::new();
    for d in digit.to_string().chars() {
        nepali_digit.push_str(NEPALI_DIGITS[d.to_digit(10).unwrap() as usize]);
    }
    nepali_digit
}
