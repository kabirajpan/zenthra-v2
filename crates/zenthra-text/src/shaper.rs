use cosmic_text::{Attrs, Buffer, Family, Metrics, Shaping, Style, Weight};
use zenthra_core::Color;

/// Input properties for text shaping.
#[derive(Debug, Clone)]
pub struct TextProperties {
    pub text: String,
    pub font_size: f32,
    pub line_height: f32, // multiplier, e.g. 1.2
    pub color: Color,
    pub family: TextFamily,
    pub weight: u16, // 100–900
    pub italic: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextFamily {
    SansSerif,
    Serif,
    Monospace,
    Cursive,
    Fantasy,
    Named(String),
}

impl Default for TextProperties {
    fn default() -> Self {
        Self {
            text: String::new(),
            font_size: 16.0,
            line_height: 1.2,
            color: Color::WHITE,
            family: TextFamily::SansSerif,
            weight: 400,
            italic: false,
        }
    }
}

/// Result of shaping a `TextProperties` — owns the cosmic-text Buffer.
pub struct ShapedText {
    pub buffer: Buffer,
}

impl ShapedText {
    pub fn shape(
        font_system: &mut cosmic_text::FontSystem,
        props: &TextProperties,
        max_width: Option<f32>,
    ) -> Self {
        let line_px = props.font_size * props.line_height;
        let metrics = Metrics::new(props.font_size, line_px);
        let mut buf = Buffer::new(font_system, metrics);

        let family = match &props.family {
            TextFamily::SansSerif => Family::SansSerif,
            TextFamily::Serif => Family::Serif,
            TextFamily::Monospace => Family::Monospace,
            TextFamily::Cursive => Family::Cursive,
            TextFamily::Fantasy => Family::Fantasy,
            TextFamily::Named(name) => Family::Name(name.as_str()),
        };

        let color = cosmic_text::Color::rgba(
            (props.color.r * 255.0) as u8,
            (props.color.g * 255.0) as u8,
            (props.color.b * 255.0) as u8,
            (props.color.a * 255.0) as u8,
        );

        let attrs = Attrs::new()
            .family(family)
            .weight(Weight(props.weight))
            .style(if props.italic {
                Style::Italic
            } else {
                Style::Normal
            })
            .color(color);

        buf.set_size(font_system, max_width, None);
        buf.set_text(font_system, &props.text, &attrs, Shaping::Advanced, None);
        buf.shape_until_scroll(font_system, false);

        Self { buffer: buf }
    }
}
