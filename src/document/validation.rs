// Schema 1 fixes these bounds, codes and permission masks in the VNU1 record.
// They describe serialized declarations. Runtime consumers must separately
// translate and authorize them against their selected service and Kernel ABI.
use super::{IdentitySelection, Owner, Ready, Scope, Unit};
use core::fmt;

const MAX_ARGUMENTS: usize = 64;
const MAX_DEPENDENCIES: usize = 96;
const MAX_IMAGE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_COALITIONS: usize = 16;
const NAMESPACE_OPERATIONS: u16 = 0x03ff;
const RESOURCE_ACCESS: u32 = 0x03;
const RESOURCE_PERMISSIONS: u32 = 0x0f;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Validation {
    Schema,
    Identity,
    Loader,
    Dependency,
    Namespace,
    Grant,
    Supply,
    Construction,
    Retirement,
}

impl fmt::Display for Validation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid Unit {self:?}")
    }
}

impl core::error::Error for Validation {}

pub(super) fn validate<S: AsRef<str>>(unit: &Unit<S>) -> Result<(), Validation> {
    if unit.schema != 1 {
        return Err(Validation::Schema);
    }
    if !valid(unit.id.as_ref()) {
        return Err(Validation::Identity);
    }
    if !valid(unit.loader.image.as_ref())
        || unit.loader.args.len() > MAX_ARGUMENTS
        || unit.loader.stack > 1 << 20
    {
        return Err(Validation::Loader);
    }
    let distinct = |values: &[S]| {
        values.iter().enumerate().all(|(at, name)| {
            valid(name.as_ref())
                && values[..at]
                    .iter()
                    .all(|previous| previous.as_ref() != name.as_ref())
        })
    };
    if unit.control.requires.len() > MAX_DEPENDENCIES
        || unit.control.after.len() > MAX_DEPENDENCIES
        || unit.control.groups.len() > 32
        || !distinct(&unit.control.requires)
        || !distinct(&unit.control.after)
        || !distinct(&unit.control.groups)
        || unit
            .control
            .supervision
            .as_ref()
            .is_some_and(|name| !valid(name.as_ref()))
        || matches!(unit.control.ready, Ready::Signal(0))
    {
        return Err(Validation::Dependency);
    }
    let path_valid = |value: &str| {
        value.len() <= 255 && value.split('/').count() <= 8 && value.split('/').all(valid)
    };
    if unit.operator.namespace.len() > 32 {
        return Err(Validation::Namespace);
    }
    for namespace in &unit.operator.namespace {
        if !valid(namespace.key.as_ref())
            || (!namespace.path.as_ref().is_empty() && !path_valid(namespace.path.as_ref()))
            || namespace.ops & !NAMESPACE_OPERATIONS != 0
            || namespace
                .permit
                .as_ref()
                .is_some_and(|permit| !permit.valid())
            || namespace.bindings.len() > 32
        {
            return Err(Validation::Namespace);
        }
        if let Scope::Names(names) = &namespace.scope
            && (names.is_empty()
                || names.len() > 32
                || names.iter().any(|name| !path_valid(name.as_ref())))
        {
            return Err(Validation::Namespace);
        }
        for (at, binding) in namespace.bindings.iter().enumerate() {
            if !valid(binding.name.as_ref())
                || binding.mark == 0
                || !binding.permit.valid()
                || (binding.view == 0 || binding.view & !RESOURCE_ACCESS != 0)
                || namespace.bindings[..at]
                    .iter()
                    .any(|previous| previous.name.as_ref() == binding.name.as_ref())
            {
                return Err(Validation::Namespace);
            }
        }
    }
    if unit.identity.aliases.len() > 32
        || !distinct(&unit.identity.aliases)
        || unit
            .identity
            .grants
            .iter()
            .enumerate()
            .any(|(at, code)| !(1..=20).contains(code) || unit.identity.grants[..at].contains(code))
    {
        return Err(Validation::Grant);
    }
    let key_valid = |value: &str| {
        !value.is_empty() && value.len() <= 128 && !value.chars().any(char::is_control)
    };
    if unit.supply.bindings.len() > 64 {
        return Err(Validation::Supply);
    }
    for (at, binding) in unit.supply.bindings.iter().enumerate() {
        if !key_valid(binding.source.as_ref())
            || !key_valid(binding.resource.as_ref())
            || binding.slot == 0
            || (binding.permission == 0 || binding.permission & !RESOURCE_PERMISSIONS != 0)
            || descriptor(binding.slot) == 0
            || unit.supply.bindings[..at].iter().any(|previous| {
                previous.slot == binding.slot
                    || descriptor(previous.slot) == descriptor(binding.slot)
            })
        {
            return Err(Validation::Supply);
        }
    }
    if unit.control.construction.len() > 64 || unit.control.retirement.len() > 64 {
        return Err(Validation::Construction);
    }
    for policy in &unit.control.construction {
        if policy.arguments.min > policy.arguments.max
            || policy.arguments.max as usize > MAX_ARGUMENTS
            || policy.stack.min > policy.stack.max
            || policy.stack.max > 1 << 20
            || policy
                .unit
                .as_ref()
                .is_some_and(|name| !valid(name.as_ref()))
            || (policy.unit.is_none()
                && (policy.max_image == 0 || policy.max_image > MAX_IMAGE_BYTES))
            || matches!(&policy.owner, Owner::Unit(name) if !valid(name.as_ref()))
        {
            return Err(Validation::Construction);
        }
        if let IdentitySelection::Exact(subject) = &policy.identity
            && (subject.authority == 0
                || usize::try_from(subject.authority).is_err()
                || subject.coalitions.len() > MAX_COALITIONS
                || subject.coalitions.windows(2).any(|pair| pair[0] >= pair[1]))
        {
            return Err(Validation::Construction);
        }
    }
    if unit.control.retirement.iter().any(|retirement| {
        !valid(retirement.provider.as_ref())
            || !valid(retirement.face.as_ref())
            || retirement.namespace >= 32
    }) {
        return Err(Validation::Retirement);
    }
    Ok(())
}

fn valid(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 31
        && !name.contains('/')
        && !name.as_bytes().contains(&0)
        && name != "."
        && name != ".."
        && !name.chars().any(char::is_control)
}

fn descriptor(slot: u64) -> u64 {
    slot.rotate_left(23) ^ 0x5355_5050_4c59_4445
}
