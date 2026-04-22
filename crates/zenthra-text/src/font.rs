/// Thin wrapper around cosmic-text's FontSystem.
/// Owns the font database and loaded faces.
pub struct FontSystem {
    pub inner: cosmic_text::FontSystem,
}

impl FontSystem {
    pub fn new() -> Self {
        Self {
            inner: cosmic_text::FontSystem::new(),
        }
    }
}

impl Default for FontSystem {
    fn default() -> Self {
        Self::new()
    }
}
