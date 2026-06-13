use crate::traits::rasterizer::Rasterizer;
use crate::types::glyph::RasterizedGlyph;
use crate::types::shaped_glyph::ShapedGlyph;
use cosmic_text::{SwashCache, FontSystem};

/// A Rasterizer implementation that uses the `swash` engine.
/// 
/// This default implementation leverages `cosmic-text`'s SwashCache, 
/// but wraps it to satisfy the Zentype Rasterizer trait.
use std::sync::{Arc, Mutex};

/// A Rasterizer implementation that uses the `swash` engine.
/// 
/// This default implementation leverages `cosmic-text`'s SwashCache, 
/// but wraps it to satisfy the Zentype Rasterizer trait.
pub struct SwashRasterizer {
    cache: SwashCache,
    font_system: Arc<Mutex<FontSystem>>,
}

impl SwashRasterizer {
    /// Creates a new SwashRasterizer with a shared FontSystem.
    pub fn new(font_system: Arc<Mutex<FontSystem>>) -> Self {
        Self {
            cache: SwashCache::new(),
            font_system,
        }
    }

    /// Access the internal font system (to synchronize fonts).
    pub fn font_system(&self) -> Arc<Mutex<FontSystem>> {
        self.font_system.clone()
    }
}

impl Rasterizer for SwashRasterizer {
    fn rasterize(&mut self, glyph: &ShapedGlyph) -> Option<RasterizedGlyph> {
        let mut fs = self.font_system.lock().unwrap();
        // Use the cache to get the pixel data
        let image = self.cache.get_image(&mut fs, glyph.key).as_ref()?;
        
        let is_color = matches!(image.content, cosmic_text::SwashContent::Color);

        Some(RasterizedGlyph {
            width: image.placement.width,
            height: image.placement.height,
            left: image.placement.left,
            top: image.placement.top,
            data: image.data.clone(),
            is_color,
        })
    }
}
