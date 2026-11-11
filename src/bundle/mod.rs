//! Versioned initrd containing Unit records and deduplicated images/resources.
//! The first eight bytes locate an opaque boot payload for the Kernel.
//! No deployment name, startup list, or host paths are stored here.

mod archive;
pub use archive::{Archive, Files, Units};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Kind {
    User = 0,
    Supervisor = 1,
    Resource = 2,
}
impl TryFrom<usize> for Kind {
    type Error = Error;
    fn try_from(value: usize) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::User),
            1 => Ok(Self::Supervisor),
            2 => Ok(Self::Resource),
            _ => Err(Error::Record),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct File<'a> {
    pub kind: Kind,
    pub name: &'a str,
    pub bytes: &'a [u8],
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Unit<'a> {
    pub id: &'a str,
    pub record: &'a [u8],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    Header,
    Version,
    Range,
    Record,
    Duplicate,
    Payload,
    Memory,
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid Unit bundle: {self:?}")
    }
}
impl core::error::Error for Error {}

/// Storage alignment in VNI1. It does not select an architecture's page model.
pub const ALIGNMENT: usize = 4096;

use alloc::vec::Vec;

pub const MAX_UNITS: usize = 96;
pub const MAX_FILES: usize = 256;
pub const MAX_RECORD: usize = crate::record::MAX_BYTES;
const HEADER: usize = 36;
const MAGIC: &[u8; 4] = b"VNI1";

/// Construct a VNI1 archive with an opaque boot payload. The composition
/// owner validates that payload with its selected Kernel capsule contract;
/// neither Unit declarations nor this storage format select a Kernel ABI.
pub fn pack(
    units: &[Unit<'_>],
    files: &[File<'_>],
    boot_unit: usize,
    boot_image: usize,
    capsule: &[u8],
) -> Result<Vec<u8>, Error> {
    if units.is_empty()
        || units.len() > MAX_UNITS
        || files.is_empty()
        || files.len() > MAX_FILES
        || boot_unit >= units.len()
        || boot_image >= files.len()
        || capsule.is_empty()
    {
        return Err(Error::Header);
    }
    let mut size = HEADER;
    for unit in units {
        if !name_valid(unit.id) || unit.record.is_empty() || unit.record.len() > MAX_RECORD {
            return Err(Error::Record);
        }
        size = size
            .checked_add(8)
            .and_then(|n| n.checked_add(unit.id.len()))
            .and_then(|n| n.checked_add(unit.record.len()))
            .ok_or(Error::Range)?;
    }
    for file in files {
        if !name_valid(file.name) || file.bytes.is_empty() {
            return Err(Error::Record);
        }
        size = size
            .checked_add(12)
            .and_then(|n| n.checked_add(file.name.len()))
            .and_then(|n| n.checked_add(file.bytes.len()))
            .ok_or(Error::Range)?;
    }
    let offset = size
        .checked_next_multiple_of(ALIGNMENT)
        .ok_or(Error::Range)?;
    let total = offset
        .checked_add(capsule.len())
        .filter(|n| *n <= u32::MAX as usize)
        .ok_or(Error::Range)?;
    let mut out = Vec::new();
    out.try_reserve_exact(total).map_err(|_| Error::Memory)?;
    for value in [offset, capsule.len()] {
        number(&mut out, value)?;
    }
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    for value in [units.len(), files.len(), boot_unit, boot_image, size] {
        number(&mut out, value)?;
    }
    for unit in units {
        entry(&mut out, unit.id, unit.record)?;
    }
    for file in files {
        number(&mut out, file.kind as usize)?;
        entry(&mut out, file.name, file.bytes)?;
    }
    out.resize(offset, 0);
    out.extend_from_slice(capsule);
    Archive::parse(&out)?;
    Ok(out)
}

fn entry(out: &mut Vec<u8>, name: &str, bytes: &[u8]) -> Result<(), Error> {
    number(out, name.len())?;
    out.extend_from_slice(name.as_bytes());
    number(out, bytes.len())?;
    out.extend_from_slice(bytes);
    Ok(())
}

fn number(out: &mut Vec<u8>, value: usize) -> Result<(), Error> {
    out.extend_from_slice(
        &u32::try_from(value)
            .map_err(|_| Error::Range)?
            .to_le_bytes(),
    );
    Ok(())
}

fn name_valid(name: &str) -> bool {
    !name.is_empty() && name.len() <= 128 && !name.chars().any(char::is_control)
}

fn u32le(bytes: &[u8], at: usize) -> Result<usize, Error> {
    let raw = bytes
        .get(at..at.checked_add(4).ok_or(Error::Range)?)
        .ok_or(Error::Record)?;
    Ok(u32::from_le_bytes(raw.try_into().map_err(|_| Error::Record)?) as usize)
}

fn read<'a>(bytes: &'a [u8], at: &mut usize) -> Result<(&'a str, &'a [u8]), Error> {
    let len = u32le(bytes, *at)?;
    *at = at.checked_add(4).ok_or(Error::Range)?;
    let end = at.checked_add(len).ok_or(Error::Range)?;
    let name = core::str::from_utf8(bytes.get(*at..end).ok_or(Error::Record)?)
        .map_err(|_| Error::Record)?;
    if !name_valid(name) {
        return Err(Error::Record);
    }
    *at = end;
    let len = u32le(bytes, *at)?;
    *at = at.checked_add(4).ok_or(Error::Range)?;
    let end = at.checked_add(len).ok_or(Error::Range)?;
    let data = bytes
        .get(*at..end)
        .filter(|data| !data.is_empty())
        .ok_or(Error::Record)?;
    *at = end;
    Ok((name, data))
}
