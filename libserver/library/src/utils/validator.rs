use regex::Regex;

const PHONE_PATTERN: &str = "/^1(3[0-9]|4[01456879]|5[0-35-9]|6[2567]|7[0-8]|8[0-9]|9[0-35-9])\\d{8}$/";
const PASSWORD_PATTERN: &str = r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d@$!%*#?&]{8,}$";

#[inline]
pub fn is_mobile_valid(mobile: &str) -> bool {
    is_string_match(PHONE_PATTERN, mobile)
}

#[inline]
pub fn is_password_valid(password: &str) -> bool {
    is_string_match(PASSWORD_PATTERN, password)
}

#[inline]
fn is_string_match(pattern: &str, value: &str) -> bool {
    let re = Regex::new(pattern).unwrap();
    re.is_match(value)
}