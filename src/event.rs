//! Typed System observations carried in framed diagnostic-channel messages.
mod error;
mod observation;
pub use error::Error;
pub use observation::Event;

use alloc::{string::String, vec::Vec};
pub const PREFIX: &[u8] = b"\x1eVNE1:";
const MAX_BYTES: usize = 1024;

pub fn encode(event: &Event<&str>) -> Result<String, Error> {
    let payload = postcard::to_allocvec(event).map_err(|_| Error::Encoding)?;
    if payload.len() > MAX_BYTES {
        return Err(Error::Size);
    }
    let mut text = String::from("\x1eVNE1:");
    const HEX: &[u8] = b"0123456789abcdef";
    for byte in payload {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 15) as usize] as char);
    }
    text.push('\n');
    Ok(text)
}

/// Frames own their decoded strings, so no allocation outlives its backing bytes.
pub fn decode(frame: &[u8]) -> Result<Event<String>, Error> {
    let hex = frame
        .strip_prefix(PREFIX)
        .and_then(|bytes| bytes.strip_suffix(b"\n"))
        .ok_or(Error::Frame)?;
    if hex.len() > MAX_BYTES * 2 || !hex.len().is_multiple_of(2) {
        return Err(Error::Size);
    }
    let digit = |byte| match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(Error::Encoding),
    };
    let bytes: Vec<u8> = hex
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| Ok(digit(pair[0])? * 16 + digit(pair[1])?))
        .collect::<Result<_, _>>()?;
    let (event, rest) = postcard::take_from_bytes(&bytes).map_err(|_| Error::Encoding)?;
    if !rest.is_empty() {
        return Err(Error::Frame);
    }
    Ok(event)
}
