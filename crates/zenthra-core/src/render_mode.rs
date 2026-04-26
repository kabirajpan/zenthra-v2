// crates/zenthra-core/src/render_mode.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderMode {
    #[default]
    Static,
    Continuous,
}
