//! Native declarations own their arrays; strings borrow the immutable boot image.
//! Keeping a Unit alive never requires leaking allocations or a global registry.
mod control;
mod convert;
mod identity;
mod loader;
mod namespace;
mod operator;
mod scope;
mod supply;
mod target;
mod unit;

pub use control::ControlConfig;
pub use identity::Identity;
pub use loader::LoaderConfig;
pub use namespace::Namespace;
pub use operator::OperatorConfig;
pub use scope::Scope;
pub use supply::SupplyConfig;
pub use target::Target;
pub use unit::UnitFile;
pub type Unit = alloc::sync::Arc<UnitFile>;
