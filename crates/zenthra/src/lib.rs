pub mod app;

// Re-export everything the user needs
pub use app::App;
pub use zenthra_core::{
    Color, EdgeInsets, Event, Id, Point, Rect, RenderMode, Response, Size, Align, BorderAlignment,
    ImageSource, ObjectFit, BackdropFilter, Filter, style,
};
pub use zenthra_widgets::container::{Direction, Wrap};
pub use zenthra_widgets::text::{FontWeight, CursorIcon, TextWrap, FontStyle};
pub use zenthra_widgets::{
    Ui, ButtonBuilder, ContainerBuilder, InputBuilder, SliderBuilder, TextBuilder,
    TextAreaBuilder, FloatingWindowBuilder, ImageBuilder, CardBuilder, PanelBuilder,
    StackBuilder, CheckboxBuilder, ToggleBuilder, RadioBuilder, DropdownBuilder,
    MenuBarBuilder, MenuBuilder, SubMenuBuilder, MenuItemBuilder, icons,
};
pub use zenthra_platform::app::WindowAction;
pub use zenthra_platform::event::PlatformEvent;
pub use zenthra_platform as platform;

pub mod prelude {
    pub use crate::App;
    pub use zenthra_core::{
        Color, EdgeInsets, RenderMode, Response, Align, BorderAlignment, ImageSource, ObjectFit,
        BackdropFilter, Filter, style,
    };
    pub use zenthra_widgets::container::{Direction, Wrap};
    pub use zenthra_widgets::text::{FontWeight, CursorIcon, TextWrap, FontStyle};
    pub use zenthra_widgets::{
        Ui, ButtonBuilder, ContainerBuilder, InputBuilder, SliderBuilder, TextBuilder,
        TextAreaBuilder, FloatingWindowBuilder, ImageBuilder, CardBuilder, PanelBuilder,
        StackBuilder, CheckboxBuilder, ToggleBuilder, RadioBuilder, DropdownBuilder,
        MenuBarBuilder, MenuBuilder, SubMenuBuilder, MenuItemBuilder, icons,
    };
    pub use zenthra_platform::app::WindowAction;
    pub use zenthra_platform::event::PlatformEvent;
    pub use zenthra_platform as platform;
}
