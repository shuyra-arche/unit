use super::{ControlConfig, Identity, LoaderConfig, OperatorConfig, SupplyConfig};

pub struct UnitFile {
    pub name: &'static str,
    pub loader: LoaderConfig,
    pub operator: OperatorConfig,
    pub identity: Identity,
    pub control: ControlConfig,
    pub supply: SupplyConfig,
}
impl UnitFile {
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn listed(&self) -> bool {
        self.control.creation == crate::document::Creation::Startup
    }
    pub fn supply(&self) -> &[crate::Binding] {
        &self.supply.bindings
    }
    pub fn valid(&self) -> bool {
        crate::valid_supplies(self.supply())
            && self.control.retirement.iter().all(crate::Retirement::valid)
    }
}
