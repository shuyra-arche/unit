use super::{IdentitySelection, Owner, Ready, Scope, Unit};
use core::fmt;

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
    use system_api::operator::{Ops, PathBuf, name::valid};
    if unit.schema != 1 {
        return Err(Validation::Schema);
    }
    if !valid(unit.id.as_ref()) {
        return Err(Validation::Identity);
    }
    if !valid(unit.loader.image.as_ref())
        || unit.loader.args.len() > system_api::loader::MAX_ARGS
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
    if unit.control.requires.len() > env::ledger::manifest::MAX_PROGRAMS
        || unit.control.after.len() > env::ledger::manifest::MAX_PROGRAMS
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
        PathBuf::try_new(value).is_some_and(|path| path.as_str() == value)
            && value.split('/').all(valid)
    };
    if unit.operator.namespace.len() > 32 {
        return Err(Validation::Namespace);
    }
    for namespace in &unit.operator.namespace {
        if !valid(namespace.key.as_ref())
            || (!namespace.path.as_ref().is_empty() && !path_valid(namespace.path.as_ref()))
            || !Ops::from_bits(namespace.ops as u64).is_some_and(Ops::is_valid)
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
                || !env::Access::from_bits(binding.view)
                    .is_some_and(|access| access != env::Access::NONE)
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
        || unit.identity.grants.iter().enumerate().any(|(at, code)| {
            system_api::identity::Grant::from_action(*code).is_none()
                || unit.identity.grants[..at].contains(code)
        })
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
            || !env::Permission::from_bits(binding.permission)
                .is_some_and(|permission| !permission.is_empty())
            || system_api::supply::descriptor_mark(env::Mark::new(binding.slot)) == env::Mark::NONE
            || unit.supply.bindings[..at].iter().any(|previous| {
                previous.slot == binding.slot
                    || system_api::supply::descriptor_mark(env::Mark::new(previous.slot))
                        == system_api::supply::descriptor_mark(env::Mark::new(binding.slot))
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
            || policy.arguments.max as usize > system_api::loader::MAX_ARGS
            || policy.stack.min > policy.stack.max
            || policy.stack.max > 1 << 20
            || policy
                .unit
                .as_ref()
                .is_some_and(|name| !valid(name.as_ref()))
            || (policy.unit.is_none()
                && (policy.max_image == 0
                    || policy.max_image > system_api::loader::MAX_IMAGE as u64))
            || matches!(&policy.owner, Owner::Unit(name) if !valid(name.as_ref()))
        {
            return Err(Validation::Construction);
        }
        if let IdentitySelection::Exact(subject) = &policy.identity
            && (subject.authority == 0
                || usize::try_from(subject.authority).is_err()
                || subject.coalitions.len() > system_api::identity::limits::MAX_ACTIVE_COALITIONS
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
