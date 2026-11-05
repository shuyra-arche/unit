#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    Frame,
    Encoding,
    Size,
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid System event: {self:?}")
    }
}
impl core::error::Error for Error {}
