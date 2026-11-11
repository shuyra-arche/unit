use unit::{
    document::{self, Unit},
    record,
};

fn declaration() -> Unit<String> {
    toml_edit::de::from_str("schema=1\nid='test.pipe-roundtrip'\n[loader]\nimage='probe'\nargs=[0,1]\n[control]\nrequires=['pipe']\nafter=['pipe']\nending='transient'\n").unwrap()
}

#[test]
fn a_toml_declaration_roundtrips_to_a_borrowed_binary_view() {
    let declaration = declaration();
    let bytes = record::encode(&declaration).unwrap();
    let decoded = record::decode(&bytes).unwrap();
    assert_eq!(decoded.id, declaration.id);
    assert_eq!(decoded.loader.args, [0, 1]);
    assert_eq!(decoded.control.requires, ["pipe"]);
    let begin = bytes.as_ptr() as usize;
    let name = decoded.id.as_ptr() as usize;
    assert!(name >= begin && name + decoded.id.len() <= begin + bytes.len());
}

#[test]
fn malformed_records_never_provide_a_partial_declaration() {
    let bytes = record::encode(&declaration()).unwrap();
    for length in 0..bytes.len() {
        assert!(record::decode(&bytes[..length]).is_err(), "prefix {length}");
    }
    let mut future = bytes.clone();
    future[4] = 2;
    assert_eq!(record::decode(&future).unwrap_err(), record::Error::Version);
    let mut trailing = bytes;
    trailing.push(0);
    assert_eq!(
        record::decode(&trailing).unwrap_err(),
        record::Error::Trailing
    );
    assert_eq!(
        record::decode(&vec![0; record::MAX_BYTES + 1]).unwrap_err(),
        record::Error::Size
    );
}

#[test]
fn control_and_authority_variants_survive_binary_encoding() {
    let mut value = declaration();
    value.control.ready = document::Ready::Signal(44);
    value.control.supervision = Some("workers".into());
    value.control.groups = vec!["scene".into()];
    value.control.guarded = true;
    value.identity.aliases = vec!["principal".into()];
    value.identity.grants = vec![1, 19];
    value.operator.namespace.push(document::Namespace {
        key: "service".into(),
        path: "pipe".into(),
        placement: document::Placement::Shared,
        scope: document::Scope::Names(vec!["create".into()]),
        ops: 1,
        permit: Some(document::Permit::Bound),
        source_mark: Some(77),
        bindings: vec![document::Publication {
            name: "create".into(),
            mark: 99,
            view: 2,
            permit: document::Permit::Public,
        }],
    });
    value.supply.bindings.push(document::SupplyBinding {
        source: "system/initrd".into(),
        resource: "payload".into(),
        slot: 57,
        permission: 1,
    });
    value.control.construction.push(document::Construction {
        unit: Some("worker".into()),
        owner: document::Owner::Unit("parent".into()),
        identity: document::IdentitySelection::Exact(document::Subject {
            authority: 1,
            principal: 2,
            coalitions: vec![3, 4],
        }),
        arguments: document::Arguments { min: 0, max: 2 },
        stack: document::Range { min: 0, max: 8192 },
        max_image: 0,
    });
    value.control.retirement.push(document::Retirement {
        provider: "pipe".into(),
        namespace: 0,
        face: "create".into(),
    });
    let encoded = record::encode(&value).unwrap();
    let decoded = record::decode(&encoded).unwrap();
    assert_eq!(decoded.control.ready, document::Ready::Signal(44));
    assert_eq!(decoded.control.ending, document::Ending::Transient);
    assert_eq!(
        decoded.operator.namespace[0].permit,
        Some(document::Permit::Bound)
    );
    assert_eq!(decoded.supply.bindings[0].source, "system/initrd");
    assert!(matches!(
        decoded.control.construction[0].owner,
        document::Owner::Unit("parent")
    ));
    assert_eq!(decoded.control.retirement[0].face, "create");
}

#[test]
fn authority_validation_rejects_unknown_bits_and_invalid_permissions() {
    let mut value = declaration();
    value.identity.grants = vec![255];
    assert_eq!(value.validate(), Err(document::Validation::Grant));
    value.identity.grants.clear();
    value.supply.bindings.push(document::SupplyBinding {
        source: "provider".into(),
        resource: "item".into(),
        slot: 1,
        permission: u32::MAX,
    });
    assert_eq!(value.validate(), Err(document::Validation::Supply));
    value.supply.bindings.clear();
    value.control.after = vec!["pipe".into(), "pipe".into()];
    assert_eq!(value.validate(), Err(document::Validation::Dependency));
}

#[test]
fn schema_typos_and_bad_subjects_are_explicit_errors() {
    assert!(
        toml_edit::de::from_str::<Unit<String>>(
            "schema=1\nid='x'\n[loader]\nimage='x'\n[control]\nafterr=[]"
        )
        .is_err()
    );
    assert!(
        !document::Permit::Identity(document::IdentitySelector::Exact {
            authority: 0,
            slot: 0
        })
        .valid()
    );
    let mut value = declaration();
    value.schema = 2;
    assert_eq!(
        record::encode(&value).unwrap_err(),
        record::Error::Invalid(document::Validation::Schema)
    );
}

#[test]
fn full_width_marks_survive_toml_and_keep_the_same_binary_u64() {
    let value: Unit<String> = toml_edit::de::from_str("schema=1\nid='account'\n[loader]\nimage='account'\nargs=['0xffffffffffffffff']\n[control]\nready={signal='9887014109901452468'}\n[supply]\nbindings=[{source='system/initrd',resource='emit',slot='18389248943054266375',permission=1}]\n").unwrap();
    assert_eq!(value.loader.args, [u64::MAX]);
    assert_eq!(
        value.control.ready,
        document::Ready::Signal(9887014109901452468)
    );
    let encoded = record::encode(&value).unwrap();
    let decoded = record::decode(&encoded).unwrap();
    assert_eq!(decoded.supply.bindings[0].slot, 18389248943054266375);
    assert_eq!(decoded.loader.args, [u64::MAX]);
    for invalid in ["-1", "'18446744073709551616'", "'shell-emit'", "'0xZZ'"] {
        let text = format!("schema=1\nid='x'\n[loader]\nimage='x'\nargs=[{invalid}]");
        assert!(toml_edit::de::from_str::<Unit<String>>(&text).is_err());
    }
}
