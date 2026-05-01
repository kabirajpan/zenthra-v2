pub mod app;

// Re-export everything the user needs
pub use app::App;
pub use zenthra_core::{Color, EdgeInsets, Event, Id, Point, Rect, RenderMode, Response, Size, Align, BorderAlignment};
pub use zenthra_widgets::container::{Direction, Wrap};
pub use zenthra_widgets::text::FontWeight;
pub use zenthra_widgets::{ButtonBuilder, ContainerBuilder, SliderBuilder, TextBuilder, Ui};

pub mod prelude {
    pub use crate::App;
    pub use zenthra_core::{Color, EdgeInsets, RenderMode, Response, Align, BorderAlignment};
    pub use zenthra_widgets::container::{Direction, Wrap};
    pub use zenthra_widgets::text::FontWeight;
    pub use zenthra_widgets::{SliderBuilder, Ui};
}
