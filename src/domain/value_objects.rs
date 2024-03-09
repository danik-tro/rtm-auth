use nutype::nutype;

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 255),
    derive(Debug, PartialEq, PartialOrd, Clone)
)]
pub struct Username(String);
