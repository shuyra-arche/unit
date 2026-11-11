use super::ALIGNMENT;
use super::{Error, File, HEADER, Kind, MAGIC, MAX_FILES, MAX_UNITS, Unit, u32le};

/// A validated Unit archive. Parsing allocates nothing. The boot payload is
/// structurally bounded here; its format and execution validation belong to
/// the Kernel/composition owner, independently of VNI1.
#[derive(Clone, Copy)]
pub struct Archive<'a> {
    body: &'a [u8],
    capsule: &'a [u8],
    units: usize,
    files: usize,
    files_at: usize,
    boot_unit: usize,
    boot_image: usize,
}

impl<'a> Archive<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, Error> {
        if bytes.len() > u32::MAX as usize || bytes.get(8..12) != Some(MAGIC) {
            return Err(Error::Header);
        }
        if bytes.get(12..16) != Some(&[1, 0, 0, 0]) {
            return Err(Error::Version);
        }
        let units = u32le(bytes, 16)?;
        let files = u32le(bytes, 20)?;
        let boot_unit = u32le(bytes, 24)?;
        let boot_image = u32le(bytes, 28)?;
        let end = u32le(bytes, 32)?;
        if units == 0
            || units > MAX_UNITS
            || files == 0
            || files > MAX_FILES
            || boot_unit >= units
            || boot_image >= files
            || end < HEADER
        {
            return Err(Error::Header);
        }
        let offset = u32le(bytes, 0)?;
        let len = u32le(bytes, 4)?;
        if end.checked_next_multiple_of(ALIGNMENT) != Some(offset)
            || offset.checked_add(len) != Some(bytes.len())
            || !bytes
                .get(end..offset)
                .is_some_and(|padding| padding.iter().all(|b| *b == 0))
            || len == 0
        {
            return Err(Error::Payload);
        }
        let body = bytes.get(..end).ok_or(Error::Range)?;
        let mut records = Units {
            bytes: body,
            at: HEADER,
            left: units,
        };
        for index in 0..units {
            let current = records.read()?;
            let mut prior = Units {
                bytes: body,
                at: HEADER,
                left: index,
            };
            for _ in 0..index {
                if prior.read()?.id == current.id {
                    return Err(Error::Duplicate);
                }
            }
        }
        let files_at = records.at;
        let mut entries = Files {
            bytes: body,
            at: files_at,
            left: files,
        };
        for index in 0..files {
            let current = entries.read()?;
            if index == boot_image && current.kind == Kind::Resource {
                return Err(Error::Record);
            }
            let mut prior = Files {
                bytes: body,
                at: files_at,
                left: index,
            };
            for _ in 0..index {
                if prior.read()?.name == current.name {
                    return Err(Error::Duplicate);
                }
            }
        }
        if entries.at != end {
            return Err(Error::Record);
        }
        Ok(Self {
            body,
            capsule: &bytes[offset..],
            units,
            files,
            files_at,
            boot_unit,
            boot_image,
        })
    }
    pub fn capsule(&self) -> &'a [u8] {
        self.capsule
    }
    pub fn units(&self) -> Units<'a> {
        Units {
            bytes: self.body,
            at: HEADER,
            left: self.units,
        }
    }
    pub fn files(&self) -> Files<'a> {
        Files {
            bytes: self.body,
            at: self.files_at,
            left: self.files,
        }
    }
    pub fn system(&self) -> Unit<'a> {
        self.units()
            .nth(self.boot_unit)
            .expect("validated boot Unit")
    }
    pub fn image(&self) -> File<'a> {
        self.files()
            .nth(self.boot_image)
            .expect("validated boot image")
    }
    pub fn find(&self, name: &str) -> Option<File<'a>> {
        self.files().find(|file| file.name == name)
    }
}

#[derive(Clone)]
pub struct Units<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) at: usize,
    pub(super) left: usize,
}

impl<'a> Units<'a> {
    pub(super) fn read(&mut self) -> Result<Unit<'a>, Error> {
        let (id, record) = super::read(self.bytes, &mut self.at)?;
        if record.len() > super::MAX_RECORD {
            return Err(Error::Record);
        }
        Ok(Unit { id, record })
    }
}

impl<'a> Iterator for Units<'a> {
    type Item = Unit<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.left == 0 {
            return None;
        }
        self.left -= 1;
        // Only Archive can create a public iterator, after validating all entries.
        Some(self.read().expect("validated Unit record"))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.left, Some(self.left))
    }
}
impl ExactSizeIterator for Units<'_> {}
impl core::iter::FusedIterator for Units<'_> {}

#[derive(Clone)]
pub struct Files<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) at: usize,
    pub(super) left: usize,
}

impl<'a> Files<'a> {
    pub(super) fn read(&mut self) -> Result<File<'a>, Error> {
        let kind = Kind::try_from(super::u32le(self.bytes, self.at)?)?;
        self.at = self.at.checked_add(4).ok_or(Error::Range)?;
        let (name, bytes) = super::read(self.bytes, &mut self.at)?;
        Ok(File { kind, name, bytes })
    }
}
impl<'a> Iterator for Files<'a> {
    type Item = File<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.left == 0 {
            return None;
        }
        self.left -= 1;
        Some(self.read().expect("validated file record"))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.left, Some(self.left))
    }
}
impl ExactSizeIterator for Files<'_> {}
impl core::iter::FusedIterator for Files<'_> {}
