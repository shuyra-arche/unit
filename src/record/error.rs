#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    Header,
    Version,
    Size,
    Encoding,
    Trailing,
    Invalid(crate::document::Validation),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Unit record: {self:?}")
    }
}

impl core::error::Error for Error {}
