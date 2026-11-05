#[derive(Clone)]
pub struct Namespace {
    pub key: system_api::operator::NamespaceKey,
    pub placement: crate::operator::Placement,
    pub path: &'static str,
    pub scope: super::Scope,
    pub ops: system_api::operator::Ops,
    pub permit: Option<system_api::operator::Permit>,
    pub source_mark: Option<u64>,
    pub bindings: alloc::vec::Vec<crate::operator::Binding>,
}
impl Namespace {
    pub fn valid(&self) -> bool {
        let path_valid = |text: &str| {
            system_api::operator::PathBuf::try_new(text).is_some_and(|path| path.as_str() == text)
                && text.split('/').all(system_api::operator::name::valid)
        };
        system_api::operator::name::valid(self.key.name())
            && self.key.mark() == env::Mark::of(self.key.name())
            && (self.path.is_empty() || path_valid(self.path))
            && self.ops.is_valid()
            && match &self.scope {
                super::Scope::Subtree => true,
                super::Scope::Names(names) => {
                    !names.is_empty() && names.iter().all(|name| path_valid(name))
                }
            }
            && self.bindings.iter().enumerate().all(|(index, binding)| {
                system_api::operator::name::valid(binding.name)
                    && binding.mark != env::Mark::NONE
                    && binding.view != env::Access::NONE
                    && self.bindings[..index]
                        .iter()
                        .all(|previous| previous.name != binding.name)
            })
    }
}
