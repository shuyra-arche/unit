#[derive(Clone)]
pub enum Scope {
    Subtree,
    Names(alloc::vec::Vec<&'static str>),
}
