use std::borrow::Cow;

/// Provide password requirements configuration to organizations (e.g. min length, max length, etc.)
///
/// Please do not recommend organizations to use password requirements that are too strict. It's ridiculous and annoying.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PasswordRequirements {
    min_length: usize,
    max_length: usize,
    min_lowercase: usize,
    min_uppercase: usize,
    min_numbers: usize,
    min_symbols: usize,
}

pub fn validate_password<'a, T>(_val: T) -> bool
where
    T: Into<Cow<'a, str>>,
{
    false
}
