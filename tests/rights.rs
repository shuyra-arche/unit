use unit::document::{
    Rights,
    rights::{self, Operations, Window},
};
#[test]
fn explicit_target_operations_roundtrip_without_numeric_permission_masks() {
    let rights: Rights = toml_edit::de::from_str(
        "operations={clone=['push','peek']}\nmodifiable=true\ndistributable=true",
    )
    .unwrap();
    assert_eq!(
        rights.operations,
        Operations::Clone(vec![rights::Clone::Push, rights::Clone::Peek])
    );
    assert!(rights.valid());
    let bytes = postcard::to_allocvec(&rights).unwrap();
    let decoded: Rights = postcard::from_bytes(&bytes).unwrap();
    assert_eq!(rights, decoded);
    for invalid in [
        "operations={clone=['open']}",
        "operations={share=['push']}",
        "operations={sign=['assign']}",
        "operations=3",
        "permission=1",
    ] {
        assert!(
            toml_edit::de::from_str::<Rights>(invalid).is_err(),
            "{invalid}"
        );
    }
}
#[test]
fn empty_duplicate_and_inconsistent_authorities_are_not_valid_declarations() {
    for text in [
        "operations={clone=[]}",
        "operations={clone=['push','push']}",
        "operations={sign=['listen']}\nmodifiable=true",
        "operations={none=['flat']}\nmodifiable=true",
        "operations={clone=['peek']}\nwindow={pages={offset=0,count=1}}",
    ] {
        let value: Rights = toml_edit::de::from_str(text).unwrap();
        assert!(!value.valid(), "{text}");
    }
    let mut value: Rights =
        toml_edit::de::from_str("operations={share=['open']}\nwindow={pages={offset=1,count=3}}")
            .unwrap();
    assert!(value.valid());
    value.window = Window::Pages {
        offset: u64::MAX,
        count: 1,
    };
    assert!(!value.valid());
}
#[test]
fn schema_two_is_a_distinct_borrowed_record_and_rejects_schema_one_rights() {
    use unit::{document::v2::Unit, record};
    let unit: Unit<String> = toml_edit::de::from_str("schema=2\nid='x'\n[loader]\nimage='x'\n[supply]\nbindings=[{source='system/initrd',resource='payload',slot=9,rights={operations={share=['open','shut']}}}]").unwrap();
    unit.validate().unwrap();
    let bytes = record::v2::encode(&unit).unwrap();
    assert!(record::decode(&bytes).is_err());
    let decoded = record::v2::decode(&bytes).unwrap();
    assert_eq!(
        decoded.supply.bindings[0].rights,
        unit.supply.bindings[0].rights
    );
    assert!(bytes.as_ptr_range().contains(&decoded.id.as_ptr()));
    for end in 0..bytes.len() {
        assert!(record::v2::decode(&bytes[..end]).is_err());
    }
    assert!(toml_edit::de::from_str::<Unit<String>>("schema=2\nid='x'\n[loader]\nimage='x'\n[supply]\nbindings=[{source='p',resource='r',slot=9,permission=1}]").is_err());
    let mut stale = unit;
    stale.schema = 1;
    assert!(record::v2::encode(&stale).is_err());
}
