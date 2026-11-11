#![no_std]
//! Pure UnitFile and Deployment declarations with explicit declaration slices.
extern crate alloc;

#[cfg(feature = "runtime")]
pub mod construction;
#[cfg(feature = "runtime")]
pub mod deployment;
pub mod document;
pub mod event;
#[cfg(feature = "runtime")]
pub mod operator;
pub mod record;
#[cfg(feature = "runtime")]
pub mod runtime;

#[cfg(feature = "runtime")]
mod control_config;
#[cfg(feature = "runtime")]
mod ending;
#[cfg(feature = "runtime")]
mod identity;
#[cfg(feature = "runtime")]
mod image_build;
#[cfg(feature = "runtime")]
mod loader;
#[cfg(feature = "runtime")]
mod operator_config;
#[cfg(feature = "runtime")]
mod ready;
#[cfg(feature = "runtime")]
mod supply;
#[cfg(feature = "runtime")]
mod supply_config;
#[cfg(feature = "runtime")]
mod unit_file;

#[cfg(feature = "runtime")]
pub use control_config::ControlConfig;
#[cfg(feature = "runtime")]
pub use ending::Ending;
#[cfg(feature = "runtime")]
pub use guard_selection_error::GuardSelectionError;
#[cfg(feature = "runtime")]
pub use identity::Identity;
#[cfg(feature = "runtime")]
pub use image_build::ImageBuild;
#[cfg(feature = "runtime")]
pub use loader::LoaderConfig;
#[cfg(feature = "runtime")]
pub use operator_config::OperatorConfig;
#[cfg(feature = "runtime")]
pub use ready::{READY_MARK, Ready};
#[cfg(feature = "runtime")]
pub use supply::valid_supplies;
#[cfg(feature = "runtime")]
pub use supply_config::SupplyConfig;
#[cfg(feature = "runtime")]
pub use unit_file::UnitFile;

#[cfg(feature = "runtime")]
pub use construction::{
    Arguments as ConstructionArguments, Construction, Identity as ConstructionIdentity,
    Owner as ConstructionOwner, Range as ConstructionRange, Selector as ConstructionSelector,
};

#[cfg(feature = "runtime")]
pub use deployment::{
    Bootstrap, Dependency, Deployment, SCENE, Target, scene_target, select as select_deployment,
};
#[cfg(feature = "runtime")]
pub use deployment_selection_error::DeploymentSelectionError;
#[cfg(feature = "runtime")]
pub use order::{DepsFail, order_scene};
#[cfg(feature = "runtime")]
pub use retirement::Retirement;

/// Assembly failure number. `true = died here`; assembly's two steps cannot hang here.
#[cfg(feature = "runtime")]
pub type Died = env::Reason;

/// Provider-owned logical binding type; UnitFile never restates its layout.
#[cfg(feature = "runtime")]
pub use system_api::supply::Binding;

#[cfg(feature = "runtime")]
mod deployment_selection_error;
#[cfg(feature = "runtime")]
mod guard_selection;
#[cfg(feature = "runtime")]
mod guard_selection_error;
#[cfg(feature = "runtime")]
mod order;
#[cfg(feature = "runtime")]
mod retirement;
