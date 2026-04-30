use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImageSource {
    Path(PathBuf),
    Bytes(std::sync::Arc<[u8]>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}

impl Default for ObjectFit {
    fn default() -> Self {
        ObjectFit::Contain
    }
}
