pub mod app;

// Re-export everything the user needs
pub use app::App;
pub use zenthra_core::{Color, Event, Id, Point, Rect, Size};
pub use zenthra_widgets::container::{Direction, HAlign, VAlign, Wrap};
pub use zenthra_widgets::{ContainerBuilder, TextBuilder, Ui};

pub mod prelude {
    pub use crate::App;
    pub use zenthra_core::Color;
    pub use zenthra_widgets::container::{Direction, HAlign, VAlign, Wrap};
    pub use zenthra_widgets::Ui;
}
