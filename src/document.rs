mod arguments;
mod construction;
mod control;
mod creation;
mod ending;
mod identity;
mod identity_selection;
mod identity_selector;
mod loader;
mod namespace;
mod number;
mod operator;
mod owner;
mod permit;
mod placement;
mod publication;
mod range;
mod ready;
mod retirement;
mod scope;
mod space;
mod subject;
mod supply;
mod supply_binding;
mod validation;

pub use arguments::Arguments;
pub use construction::Construction;
pub use control::Control;
pub use creation::Creation;
pub use ending::Ending;
pub use identity::Identity;
pub use identity_selection::IdentitySelection;
pub use identity_selector::IdentitySelector;
pub use loader::Loader;
pub use namespace::Namespace;
pub use operator::Operator;
pub use owner::Owner;
pub use permit::Permit;
pub use placement::Placement;
pub use publication::Publication;
pub use range::Range;
pub use ready::Ready;
pub use retirement::Retirement;
pub use scope::Scope;
pub use space::Space;
pub use subject::Subject;
pub use supply::Supply;
pub use supply_binding::SupplyBinding;
pub use validation::Validation;

use serde::{Deserialize, Serialize};

/// One schema used as owned host input or as a borrowed runtime declaration.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(
    deny_unknown_fields,
    bound(deserialize = "S: Deserialize<'de> + Default")
)]
pub struct Unit<S> {
    pub schema: u32,
    pub id: S,
    pub loader: Loader<S>,
    #[serde(default)]
    pub operator: Operator<S>,
    #[serde(default)]
    pub identity: Identity<S>,
    #[serde(default)]
    pub control: Control<S>,
    #[serde(default)]
    pub supply: Supply<S>,
}

impl<S: AsRef<str>> Unit<S> {
    pub fn validate(&self) -> Result<(), Validation> {
        validation::validate(self)
    }
}
