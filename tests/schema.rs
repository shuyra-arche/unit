use system_api::operator::{Ops, PathBuf};
use unit::document::{self, Unit, Validation};

fn declaration() -> Unit<String> {
    let mut unit = Unit::default();
    unit.schema = 1;
    unit.id = "probe".into();
    unit.loader.image = "probe".into();
    unit
}

fn namespace() -> document::Namespace<String> {
    document::Namespace {
        key: "service".into(),
        placement: document::Placement::Shared,
        path: "".into(),
        scope: document::Scope::Subtree,
        ops: 0,
        permit: None,
        source_mark: None,
        bindings: vec![],
    }
}

#[test]
fn schema_one_codes_preserve_the_existing_authority_contract() {
    let mut unit = declaration();
    for grant in 0..=u8::MAX {
        unit.identity.grants = vec![grant];
        assert_eq!(
            unit.validate().is_ok(),
            system_api::identity::Grant::from_action(grant).is_some(),
            "grant {grant}"
        );
    }
    unit.identity.grants.clear();
    unit.operator.namespace.push(namespace());
    for ops in 0..=u16::MAX {
        unit.operator.namespace[0].ops = ops;
        assert_eq!(
            unit.validate().is_ok(),
            Ops::from_bits(u64::from(ops)).is_some_and(Ops::is_valid),
            "ops {ops}"
        );
    }
}

#[test]
fn schema_one_rejects_mixed_access_and_unknown_supply_bits() {
    let mut unit = declaration();
    unit.operator.namespace.push(namespace());
    unit.operator.namespace[0]
        .bindings
        .push(document::Publication {
            name: "read".into(),
            mark: 1,
            view: 0,
            permit: document::Permit::Public,
        });
    for bits in (0..=255).chain((0..32).map(|bit| 1 << bit)) {
        unit.operator.namespace[0].bindings[0].view = bits;
        assert_eq!(
            unit.validate().is_ok(),
            env::Access::from_bits(bits).is_some_and(|access| access != env::Access::NONE)
        );
    }
    unit.operator.namespace.clear();
    unit.supply.bindings.push(document::SupplyBinding {
        source: "system/initrd".into(),
        resource: "payload".into(),
        slot: 1,
        permission: 0,
    });
    for bits in (0..=255).chain((0..32).map(|bit| 1 << bit)) {
        unit.supply.bindings[0].permission = bits;
        assert_eq!(
            unit.validate().is_ok(),
            env::Permission::from_bits(bits).is_some_and(|permission| !permission.is_empty())
        );
    }
    unit.supply.bindings[0].permission = 1;
    let reserved = 0x5355_5050_4c59_4445u64.rotate_right(23);
    assert_eq!(
        system_api::supply::descriptor_mark(env::Mark::new(reserved)),
        env::Mark::NONE
    );
    unit.supply.bindings[0].slot = reserved;
    assert_eq!(unit.validate(), Err(Validation::Supply));
}

#[test]
fn schema_one_names_and_paths_keep_canonical_bounds() {
    let mut unit = declaration();
    for name in ["", ".", "..", "a/b", "a\0b", "a\nb", "probe", "圜"] {
        unit.id = name.into();
        assert_eq!(
            unit.validate().is_ok(),
            system_api::operator::name::valid(name)
        );
    }
    unit.id = "probe".into();
    unit.operator.namespace.push(namespace());
    for path in [
        "a",
        "/a",
        "a/",
        "a//b",
        "a/../b",
        "a/b/c/d/e/f/g/h",
        "a/b/c/d/e/f/g/h/i",
    ] {
        unit.operator.namespace[0].path = path.into();
        let accepted = PathBuf::try_new(path).is_some_and(|value| value.as_str() == path)
            && path.split('/').all(system_api::operator::name::valid);
        assert_eq!(unit.validate().is_ok(), accepted, "path {path}");
    }
    unit.operator.namespace.clear();
    unit.loader.args = vec![0; system_api::loader::MAX_ARGS];
    assert!(unit.validate().is_ok());
    unit.loader.args.push(0);
    assert_eq!(unit.validate(), Err(Validation::Loader));
    unit.loader.args.clear();
    unit.control.requires = (0..env::ledger::manifest::MAX_PROGRAMS)
        .map(|index| format!("dependency-{index}"))
        .collect();
    assert!(unit.validate().is_ok());
    unit.control.requires.push("overflow".into());
    assert_eq!(unit.validate(), Err(Validation::Dependency));
}
