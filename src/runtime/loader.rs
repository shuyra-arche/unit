pub struct LoaderConfig {
    pub image: &'static str,
    pub space: env::ProgramKind,
    pub args: alloc::vec::Vec<usize>,
    pub stack: usize,
}
