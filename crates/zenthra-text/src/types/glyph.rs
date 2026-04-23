use cosmic_text::CacheKey;

/// Unique identifier for a glyph at a specific size and style.
/// Directly maps to `cosmic_text::CacheKey`.
pub type GlyphKey = CacheKey;

/// Metadata for a glyph stored in the GPU atlas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AtlasEntry {
    /// Normalized UV coordinates (top-left) in the atlas texture.
    pub uv_pos: [f32; 2],
    /// Normalized UV size (width, height) in the atlas texture.
    pub uv_size: [f32; 2],
    /// Physical pixel size [width, height] of the glyph in the atlas.
    pub pixel_size: [f32; 2],
    /// Physical pixel offset [left, top] relative to the logical origin.
    pub pixel_offset: [f32; 2],
}



/// Raw pixel data and metrics for a rasterized glyph.
pub struct RasterizedGlyph {
    /// Width of the rasterized bitmap.
    pub width: u32,
    /// Height of the rasterized bitmap.
    pub height: u32,
    /// Left offset (displacement from origin).
    pub left: i32,
    /// Top offset (displacement from origin).
    pub top: i32,
    /// Grayscale or RGBA pixel data.
    pub data: Vec<u8>,
}


pub use zenthra_core::GlyphInstance;
