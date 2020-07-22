pub fn first_char(s: &str) -> char {
    s.chars().collect::<Vec<char>>()[0]
}

pub fn is_string_alphanumeric(s: &str) -> bool {
    s.chars().all(char::is_alphanumeric)
}
