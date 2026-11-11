pub use super::Error;

use crate::document::v2::Unit;
use alloc::{string::String, vec::Vec};

const MAGIC: &[u8; 4] = b"VNU2";
pub const MAX_BYTES: usize = 128 * 1024;

pub fn encode(unit: &Unit<String>) -> Result<Vec<u8>, Error> {
    unit.validate().map_err(Error::Invalid)?;
    let payload = postcard::to_allocvec(unit).map_err(|_| Error::Encoding)?;
    let total = payload.len().checked_add(8).ok_or(Error::Size)?;
    if total > MAX_BYTES {
        return Err(Error::Size);
    }
    let mut bytes = Vec::with_capacity(total);
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes.extend(payload);
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<Unit<&str>, Error> {
    if bytes.len() > MAX_BYTES {
        return Err(Error::Size);
    }
    if bytes.get(..4) != Some(MAGIC) {
        return Err(Error::Header);
    }
    if bytes.get(4..8) != Some(&[2, 0, 0, 0]) {
        return Err(Error::Version);
    }
    let (unit, rest): (Unit<&str>, _) =
        postcard::take_from_bytes(&bytes[8..]).map_err(|_| Error::Encoding)?;
    if !rest.is_empty() {
        return Err(Error::Trailing);
    }
    unit.validate().map_err(Error::Invalid)?;
    Ok(unit)
}
