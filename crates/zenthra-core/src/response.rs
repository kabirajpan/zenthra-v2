// crates/zenthra-core/src/response.rs

#[derive(Debug, Clone, Copy, Default)]
pub struct Response {
    pub clicked: bool,
    pub hovered: bool,
    pub pressed: bool,
    pub submitted: bool,
}

impl Response {
    pub fn new() -> Self {
        Self::default()
    }
}
