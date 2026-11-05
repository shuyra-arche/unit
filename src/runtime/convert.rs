use crate::{document as d, runtime as r};
use alloc::vec::Vec;
use system_api::{identity, operator};

impl TryFrom<d::Unit<&'static str>> for r::UnitFile {
    type Error = d::Validation;
    fn try_from(unit: d::Unit<&'static str>) -> Result<Self, Self::Error> {
        unit.validate()?;
        let native = Self {
            name: unit.id,
            loader: r::LoaderConfig {
                image: unit.loader.image,
                space: match unit.loader.space {
                    d::Space::User => env::ProgramKind::User,
                    d::Space::Supervisor => env::ProgramKind::Supervisor,
                },
                args: unit
                    .loader
                    .args
                    .into_iter()
                    .map(|arg| usize::try_from(arg).map_err(|_| d::Validation::Loader))
                    .collect::<Result<_, _>>()?,
                stack: usize::try_from(unit.loader.stack).map_err(|_| d::Validation::Loader)?,
            },
            identity: r::Identity {
                aliases: unit.identity.aliases,
                grants: unit
                    .identity
                    .grants
                    .into_iter()
                    .map(|code| identity::Grant::from_action(code).ok_or(d::Validation::Grant))
                    .collect::<Result<_, _>>()?,
            },
            supply: r::SupplyConfig {
                bindings: unit
                    .supply
                    .bindings
                    .into_iter()
                    .map(|binding| {
                        Ok(crate::Binding {
                            source: system_api::supply::Key::of(binding.source),
                            resource: system_api::supply::Key::of(binding.resource),
                            slot: env::Mark::new(binding.slot),
                            permission: env::Permission::from_bits(binding.permission)
                                .ok_or(d::Validation::Supply)?,
                        })
                    })
                    .collect::<Result<_, _>>()?,
            },
            operator: r::OperatorConfig {
                namespace: unit
                    .operator
                    .namespace
                    .into_iter()
                    .map(|namespace| {
                        Ok(r::Namespace {
                            key: operator::NamespaceKey::of(namespace.key),
                            placement: match namespace.placement {
                                d::Placement::Shared => crate::operator::Placement::Shared,
                                d::Placement::Instance => crate::operator::Placement::Instance,
                            },
                            path: namespace.path,
                            scope: match namespace.scope {
                                d::Scope::Subtree => r::Scope::Subtree,
                                d::Scope::Names(names) => r::Scope::Names(names),
                            },
                            ops: operator::Ops::from_bits(namespace.ops as u64)
                                .ok_or(d::Validation::Namespace)?,
                            permit: namespace.permit.map(permit).transpose()?,
                            source_mark: namespace.source_mark,
                            bindings: namespace
                                .bindings
                                .into_iter()
                                .map(|binding| {
                                    Ok(crate::operator::Binding {
                                        name: binding.name,
                                        mark: env::Mark::new(binding.mark),
                                        view: env::Access::from_bits(binding.view)
                                            .ok_or(d::Validation::Namespace)?,
                                        permit: permit(binding.permit)?,
                                    })
                                })
                                .collect::<Result<_, _>>()?,
                        })
                    })
                    .collect::<Result<_, _>>()?,
            },
            control: r::ControlConfig {
                creation: unit.control.creation,
                after: Some(
                    unit.control
                        .after
                        .into_iter()
                        .map(crate::Dependency::Unit)
                        .collect(),
                ),
                ending: match unit.control.ending {
                    d::Ending::Resident => crate::Ending::Resident,
                    d::Ending::Transient => crate::Ending::Transient,
                    d::Ending::Told => crate::Ending::Told,
                },
                ready: match unit.control.ready {
                    d::Ready::Spawn => crate::Ready::OnSpawn,
                    d::Ready::Signal(mark) => crate::Ready::Signal(env::Mark::new(mark)),
                },
                construction: unit
                    .control
                    .construction
                    .into_iter()
                    .map(construction)
                    .collect::<Result<_, _>>()?,
                retirement: unit
                    .control
                    .retirement
                    .into_iter()
                    .map(|item| crate::Retirement {
                        provider: item.provider,
                        namespace: item.namespace as usize,
                        face: item.face,
                    })
                    .collect(),
                supervision: unit.control.supervision,
                groups: unit.control.groups,
                guarded: unit.control.guarded,
            },
        };
        if !native.valid() {
            return Err(d::Validation::Supply);
        }
        Ok(native)
    }
}

fn task(raw: u64) -> Result<env::TaskId, d::Validation> {
    let value = usize::try_from(raw).map_err(|_| d::Validation::Construction)?;
    if value == 0 {
        return Err(d::Validation::Construction);
    }
    Ok(env::TaskId::new(value))
}
fn permit(value: d::Permit) -> Result<operator::Permit, d::Validation> {
    Ok(match value {
        d::Permit::Public => operator::Permit::Public,
        d::Permit::Bound => operator::Permit::Bound,
        d::Permit::Opener(raw) => operator::Permit::Opener(operator::EntryId::new(
            usize::try_from(raw).map_err(|_| d::Validation::Namespace)?,
        )),
        d::Permit::Identity(selector) => operator::Permit::Identity(match selector {
            d::IdentitySelector::Exact { authority, slot } => {
                identity::Selector::Exact(identity::PrincipalId::new(task(authority)?, slot))
            }
            d::IdentitySelector::DescendantOf { authority, slot } => {
                identity::Selector::DescendantOf(identity::PrincipalId::new(task(authority)?, slot))
            }
            d::IdentitySelector::MemberOf { authority, slot } => {
                identity::Selector::MemberOf(identity::CoalitionId::new(task(authority)?, slot))
            }
        }),
    })
}
fn construction(
    value: d::Construction<&'static str>,
) -> Result<crate::Construction, d::Validation> {
    Ok(crate::Construction {
        selector: value.unit.map_or(
            crate::ConstructionSelector::Image,
            crate::ConstructionSelector::Unit,
        ),
        owner: match value.owner {
            d::Owner::Caller => crate::ConstructionOwner::Caller,
            d::Owner::Selected => crate::ConstructionOwner::Selected,
            d::Owner::Task(raw) => crate::ConstructionOwner::Exact(task(raw)?),
            d::Owner::Unit(name) => crate::ConstructionOwner::NamedUnit(name),
        },
        identity: match value.identity {
            d::IdentitySelection::Caller => crate::ConstructionIdentity::Caller,
            d::IdentitySelection::Selected => crate::ConstructionIdentity::Selected,
            d::IdentitySelection::Exact(subject) => {
                let authority = task(subject.authority)?;
                let principal = identity::PrincipalId::new(authority, subject.principal);
                let coalitions: Vec<_> = subject
                    .coalitions
                    .into_iter()
                    .map(|slot| identity::CoalitionId::new(authority, slot))
                    .collect();
                crate::ConstructionIdentity::Exact(
                    identity::Subject::new(principal, &coalitions)
                        .map_err(|_| d::Validation::Construction)?,
                )
            }
        },
        arguments: crate::ConstructionArguments {
            min: value.arguments.min,
            max: value.arguments.max,
        },
        stack: crate::ConstructionRange {
            min: value.stack.min,
            max: value.stack.max,
        },
        max_image: usize::try_from(value.max_image).map_err(|_| d::Validation::Construction)?,
    })
}
