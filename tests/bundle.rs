use unit::bundle::{self, Archive, File, Kind, Unit};
fn boot() -> Vec<u8> {
    b"opaque Kernel-owned boot payload".to_vec()
}

fn sample() -> Vec<u8> {
    bundle::pack(
        &[
            Unit {
                id: "system",
                record: b"unit record",
            },
            Unit {
                id: "pipe",
                record: b"pipe record",
            },
        ],
        &[
            File {
                kind: Kind::Supervisor,
                name: "system-bin",
                bytes: b"elf",
            },
            File {
                kind: Kind::Resource,
                name: "configuration",
                bytes: b"data",
            },
        ],
        0,
        0,
        &boot(),
    )
    .unwrap()
}

#[test]
fn records_and_files_borrow_the_archive_and_keep_independent_identities() {
    let bytes = sample();
    let archive = Archive::parse(&bytes).unwrap();
    assert_eq!(archive.system().id, "system");
    assert_eq!(archive.image().name, "system-bin");
    assert_eq!(archive.units().len(), 2);
    assert_eq!(archive.units().nth(1).unwrap().id, "pipe");
    let resource = archive.find("configuration").unwrap();
    assert_eq!(resource.kind, Kind::Resource);
    assert_eq!(resource.bytes, b"data");
    assert!((bytes.as_ptr_range()).contains(&resource.bytes.as_ptr()));
    let mut files = archive.files();
    assert_eq!(files.by_ref().count(), 2);
    assert!(files.next().is_none());
}

#[test]
fn every_truncation_and_appended_data_is_rejected() {
    let mut bytes = sample();
    for end in 0..bytes.len() {
        assert!(Archive::parse(&bytes[..end]).is_err(), "{end}");
    }
    bytes.push(0);
    assert!(Archive::parse(&bytes).is_err());
}

#[test]
fn corrupted_headers_padding_and_records_are_rejected_without_partial_results() {
    let bytes = sample();
    for at in [0, 4, 8, 12, 14, 16, 20, 24, 28, 32, 36] {
        let mut bad = bytes.clone();
        bad[at..at + 4].fill(255);
        assert!(Archive::parse(&bad).is_err(), "{at}");
    }
    let end = u32::from_le_bytes(bytes[32..36].try_into().unwrap()) as usize;
    let mut bad = bytes.clone();
    bad[end] = 1;
    assert!(Archive::parse(&bad).is_err());
    let mut bad = bytes;
    bad[36..40].copy_from_slice(&0u32.to_le_bytes());
    assert!(Archive::parse(&bad).is_err());
}

#[test]
fn duplicate_names_and_a_resource_boot_image_are_rejected() {
    let units = [Unit {
        id: "same",
        record: b"one",
    }; 2];
    let file = File {
        name: "boot",
        kind: Kind::User,
        bytes: b"elf",
    };
    assert_eq!(
        bundle::pack(&units, &[file], 0, 0, &boot()),
        Err(bundle::Error::Duplicate)
    );
    assert_eq!(
        bundle::pack(&units[..1], &[file, file], 0, 0, &boot()),
        Err(bundle::Error::Duplicate)
    );
    let resource = File {
        kind: Kind::Resource,
        ..file
    };
    assert_eq!(
        bundle::pack(&units[..1], &[resource], 0, 0, &boot()),
        Err(bundle::Error::Record)
    );
}

#[test]
fn envelope_owns_indexing_while_boot_payload_remains_opaque() {
    let bytes = sample();
    let archive = Archive::parse(&bytes).unwrap();
    assert_eq!(archive.capsule(), boot());
    assert_eq!(
        u32::from_le_bytes(bytes[..4].try_into().unwrap()) as usize,
        bundle::ALIGNMENT
    );
    assert_eq!(&bytes[8..16], b"VNI1\x01\0\0\0");
    assert!((bytes.as_ptr_range()).contains(&archive.capsule().as_ptr()));
    // Kernel version and executable validation are a different contract. This
    // pure declaration library does not silently choose an old Kernel parser.
    let payload = [255; 13];
    let packed = bundle::pack(
        &[Unit {
            id: "system",
            record: b"record",
        }],
        &[File {
            kind: Kind::Supervisor,
            name: "image",
            bytes: b"elf",
        }],
        0,
        0,
        &payload,
    )
    .unwrap();
    assert_eq!(Archive::parse(&packed).unwrap().capsule(), payload);
}

#[test]
fn bounds_and_empty_boot_payload_fail_without_a_partial_archive() {
    let unit = Unit {
        id: "system",
        record: b"record",
    };
    let file = File {
        kind: Kind::Supervisor,
        name: "image",
        bytes: b"elf",
    };
    assert!(bundle::pack(&[], &[file], 0, 0, &boot()).is_err());
    assert!(bundle::pack(&[unit], &[file], 0, 0, &[]).is_err());
    let units = vec![unit; bundle::MAX_UNITS + 1];
    assert_eq!(
        bundle::pack(&units, &[file], 0, 0, &boot()),
        Err(bundle::Error::Header)
    );
    let record = vec![0; bundle::MAX_RECORD + 1];
    assert_eq!(
        bundle::pack(
            &[Unit {
                record: &record,
                ..unit
            }],
            &[file],
            0,
            0,
            &boot()
        ),
        Err(bundle::Error::Record)
    );
}
