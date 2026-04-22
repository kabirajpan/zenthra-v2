use cosmic_text::{CacheKey, FontSystem, SwashCache, SwashContent};
use etagere::{size2, AtlasAllocator};
use rustc_hash::FxHashMap;

const ATLAS_SIZE: u32 = 2048;

#[derive(Debug, Clone, Copy)]
pub struct AtlasGlyph {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
    pub width: u32,
    pub height: u32,
    pub left: i32,
    pub top: i32,
}

pub struct GlyphAtlas {
    allocator: AtlasAllocator,
    cache: FxHashMap<CacheKey, AtlasGlyph>,
    swash_cache: SwashCache,
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    staging: Vec<(u32, u32, u32, u32, Vec<u8>)>,
}

impl GlyphAtlas {
    pub fn new(device: &wgpu::Device) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Zenthra Glyph Atlas"),
            size: wgpu::Extent3d {
                width: ATLAS_SIZE,
                height: ATLAS_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&Default::default());

        Self {
            allocator: AtlasAllocator::new(size2(ATLAS_SIZE as i32, ATLAS_SIZE as i32)),
            cache: FxHashMap::default(),
            swash_cache: SwashCache::new(),
            texture,
            texture_view,
            staging: Vec::new(),
        }
    }

    pub fn get_or_insert(
        &mut self,
        font_system: &mut FontSystem,
        key: CacheKey,
    ) -> Option<AtlasGlyph> {
        if let Some(&g) = self.cache.get(&key) {
            return Some(g);
        }

        let (w, h, left, top, data) = {
            let image_opt = self.swash_cache.get_image(font_system, key);
            let image = image_opt.as_ref()?;

            let w = image.placement.width;
            let h = image.placement.height;
            if w == 0 || h == 0 {
                return None;
            }

            let data: Vec<u8> = match image.content {
                SwashContent::Mask => image.data.to_vec(),
                SwashContent::Color | SwashContent::SubpixelMask => {
                    image.data.chunks(4).map(|c| c[3]).collect()
                }
            };

            (w, h, image.placement.left, image.placement.top, data)
        };

        let alloc = self.allocator.allocate(size2(w as i32 + 1, h as i32 + 1))?;
        let x = alloc.rectangle.min.x as u32;
        let y = alloc.rectangle.min.y as u32;

        self.staging.push((x, y, w, h, data));

        let g = AtlasGlyph {
            u0: x as f32 / ATLAS_SIZE as f32,
            v0: y as f32 / ATLAS_SIZE as f32,
            u1: (x + w) as f32 / ATLAS_SIZE as f32,
            v1: (y + h) as f32 / ATLAS_SIZE as f32,
            width: w,
            height: h,
            left,
            top,
        };
        self.cache.insert(key, g);
        Some(g)
    }

    pub fn flush(&mut self, queue: &wgpu::Queue) {
        for (x, y, w, h, data) in self.staging.drain(..) {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x, y, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                &data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(w),
                    rows_per_image: None,
                },
                wgpu::Extent3d {
                    width: w,
                    height: h,
                    depth_or_array_layers: 1,
                },
            );
        }
    }

    pub fn clear(&mut self) {
        self.allocator = AtlasAllocator::new(size2(ATLAS_SIZE as i32, ATLAS_SIZE as i32));
        self.cache.clear();
        self.staging.clear();
    }
}
