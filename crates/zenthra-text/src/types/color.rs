pub use zenthra_core::Color;

/// A trait to convert our core Color type to backend-specific color formats 
/// and add utility methods for Zentype.
pub trait ColorExt {
    fn to_cosmic(self) -> cosmic_text::Color;
    
    /// Parses a hex string into a Color.
    fn hex(hex_str: &str) -> Color;
}

impl ColorExt for Color {
    fn to_cosmic(self) -> cosmic_text::Color {
        cosmic_text::Color::rgba(
            (self.r * 255.0).clamp(0.0, 255.0) as u8,
            (self.g * 255.0).clamp(0.0, 255.0) as u8,
            (self.b * 255.0).clamp(0.0, 255.0) as u8,
            (self.a * 255.0).clamp(0.0, 255.0) as u8,
        )
    }

    fn hex(hex_str: &str) -> Color {
        let h = hex_str.trim_start_matches('#');
        match h.len() {
            3 => {
                let r = u8::from_str_radix(&h[0..1].repeat(2), 16).unwrap_or(0);
                let g = u8::from_str_radix(&h[1..2].repeat(2), 16).unwrap_or(0);
                let b = u8::from_str_radix(&h[2..3].repeat(2), 16).unwrap_or(0);
                Self::rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
            }
            4 => {
                let r = u8::from_str_radix(&h[0..1].repeat(2), 16).unwrap_or(0);
                let g = u8::from_str_radix(&h[1..2].repeat(2), 16).unwrap_or(0);
                let b = u8::from_str_radix(&h[2..3].repeat(2), 16).unwrap_or(0);
                let a = u8::from_str_radix(&h[3..4].repeat(2), 16).unwrap_or(255);
                Self::rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
            }
            6 => {
                let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(0);
                Self::rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
            }
            8 => {
                let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(0);
                let a = u8::from_str_radix(&h[6..8], 16).unwrap_or(255);
                Self::rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
            }
            _ => Self::BLACK, // Fallback
        }
    }
}
