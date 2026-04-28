use crate::types::color::Color;
use cosmic_text::{Align, Attrs, Buffer, FontSystem, Metrics};

/// Options for configuring text rendering.
/// Use the builder-like methods for a fluent API.
#[derive(Debug, Clone, PartialEq)]
pub struct TextOptions {
    /// X coordinate of the text.
    pub x: f32,
    pub y: f32,

    pub font_size: f32,
    pub color: Color,
    pub font_family: Option<String>,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,

    pub highlight_color: Option<Color>,

    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub line_height: f32,
    pub wrap: TextWrap,
    /// Horizontal alignment. If None, per-line settings are preserved.
    pub align: Option<HorizontalAlignment>,
    /// Vertical alignment within the available space.
    pub valign: Option<VerticalAlignment>,
    pub min_width: Option<f32>,
    pub clip_rect: Option<[f32; 4]>,
    pub scale_factor: f32,
}



use std::cell::RefCell;
thread_local! {
    static LAST_APPLIED_OPTIONS: RefCell<TextOptions> = RefCell::new(TextOptions::default());
}

pub fn get_last_applied() -> TextOptions {
    LAST_APPLIED_OPTIONS.with(|opts| opts.borrow().clone())
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            font_size: 16.0,
            color: Color::WHITE,
            font_family: None,
            font_weight: FontWeight::Regular,
            font_style: FontStyle::Normal,
            highlight_color: None,
            max_width: None,
            max_height: None,
            line_height: 1.2,
            wrap: TextWrap::Word,
            align: None,
            valign: None,
            min_width: None,
            clip_rect: None,
            scale_factor: 1.0,
        }
    }
}


impl TextOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn min_width(mut self, w: f32) -> Self {
        self.min_width = Some(w);
        self
    }

    pub fn at(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn font_family(mut self, family: impl Into<String>) -> Self {
        self.font_family = Some(family.into());
        self
    }

    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = weight;
        self
    }

    pub fn font_style(mut self, style: FontStyle) -> Self {
        self.font_style = style;
        self
    }

    pub fn highlight(mut self, color: Color) -> Self {
        self.highlight_color = Some(color);
        self
    }


    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Sets the maximum height for word wrapping and vertical alignment.
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    pub fn line_height(mut self, height: f32) -> Self {
        self.line_height = height;
        self
    }

    pub fn wrap(mut self, strategy: TextWrap) -> Self {
        self.wrap = strategy;
        self
    }

    pub fn align(mut self, alignment: HorizontalAlignment) -> Self {
        self.align = Some(alignment);
        self
    }

    pub fn valign(mut self, alignment: VerticalAlignment) -> Self {
        self.valign = Some(alignment);
        self
    }

    pub fn clip_rect(mut self, x: f32, y: f32, w: f32, h: f32) -> Self {
        self.clip_rect = Some([x, y, w, h]);
        self
    }

    pub fn scale_factor(mut self, sf: f32) -> Self {
        self.scale_factor = sf;
        self
    }

    pub fn apply(&self, font_system: &mut FontSystem, buffer: &mut Buffer) {

        // Save to global state for testing tools
        LAST_APPLIED_OPTIONS.with(|opts| {
            *opts.borrow_mut() = self.clone();
        });

        // Set metrics (font size and line height)
        buffer.set_metrics(
            font_system,
            Metrics::new(self.font_size, self.font_size * self.line_height),
        );

        // Set size and wrap mode
        let wrap = match self.wrap {
            TextWrap::Word => cosmic_text::Wrap::Word,
            TextWrap::Character => cosmic_text::Wrap::Glyph,
            TextWrap::None => cosmic_text::Wrap::None,
        };

        
        buffer.set_size(font_system, self.max_width, None);
        buffer.set_wrap(font_system, wrap);

        // Shape first, THEN apply alignment once glyphs are measured
        buffer.shape_until_scroll(font_system, false);

        if let Some(alignment) = self.align {
            let align: Align = alignment.into();
            for line in buffer.lines.iter_mut() {
                line.set_align(Some(align));
            }
            // Re-shape once more to reflect alignment positions
            buffer.shape_until_scroll(font_system, false);
        }
    }

    pub fn as_attrs(&self) -> Attrs<'_> {
        use crate::types::color::ColorExt;
        let mut attrs = Attrs::new()
            .color(self.color.to_cosmic())
            .weight(self.font_weight.into())
            .style(self.font_style.into());

        if let Some(ref family) = self.font_family {
            let family_enum = match family.to_lowercase().as_str() {
                "sans-serif" => cosmic_text::Family::SansSerif,
                "serif" => cosmic_text::Family::Serif,
                "monospace" => cosmic_text::Family::Monospace,
                _ => cosmic_text::Family::Name(family),
            };
            attrs = attrs.family(family_enum);
        }

        attrs
    }
}

/// Supported font weights.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    Custom(u16),
}

/// Supported font styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

impl From<FontStyle> for cosmic_text::Style {
    fn from(style: FontStyle) -> Self {
        match style {
            FontStyle::Normal => cosmic_text::Style::Normal,
            FontStyle::Italic => cosmic_text::Style::Italic,
            FontStyle::Oblique => cosmic_text::Style::Oblique,
        }
    }
}

impl From<FontWeight> for cosmic_text::Weight {
    fn from(weight: FontWeight) -> Self {
        match weight {
            FontWeight::Thin => cosmic_text::Weight::THIN,
            FontWeight::ExtraLight => cosmic_text::Weight::EXTRA_LIGHT,
            FontWeight::Light => cosmic_text::Weight::LIGHT,
            FontWeight::Regular => cosmic_text::Weight::NORMAL,
            FontWeight::Medium => cosmic_text::Weight::MEDIUM,
            FontWeight::SemiBold => cosmic_text::Weight::SEMIBOLD,
            FontWeight::Bold => cosmic_text::Weight::BOLD,
            FontWeight::ExtraBold => cosmic_text::Weight::EXTRA_BOLD,
            FontWeight::Black => cosmic_text::Weight::BLACK,
            FontWeight::Custom(w) => cosmic_text::Weight(w),
        }
    }
}

impl From<u16> for FontWeight {
    fn from(w: u16) -> Self {
        FontWeight::Custom(w)
    }
}

impl From<f32> for FontWeight {
    fn from(w: f32) -> Self {
        FontWeight::Custom(w as u16)
    }
}

/// Strategy for wrapping text when it exceeds `max_width`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextWrap {
    #[default]
    Word,
    Character,
    None,
}

/// Horizontal text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HorizontalAlignment {
    #[default]
    Left,
    Center,
    Right,
    Justified,
}

/// Vertical text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalAlignment {
    #[default]
    Top,
    Center,
    Bottom,
}

/// Re-export EdgeInsets as Padding for Zentype compatibility
pub use zenthra_core::EdgeInsets as Padding;

impl From<HorizontalAlignment> for Align {
    fn from(alignment: HorizontalAlignment) -> Self {
        match alignment {
            HorizontalAlignment::Left => Align::Left,
            HorizontalAlignment::Center => Align::Center,
            HorizontalAlignment::Right => Align::Right,
            HorizontalAlignment::Justified => Align::Justified,
        }
    }
}
