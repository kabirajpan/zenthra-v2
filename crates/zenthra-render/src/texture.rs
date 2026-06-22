fn load_image_or_svg(bytes: &[u8], max_dim: Option<u32>) -> Result<(Vec<u8>, u32, u32), image::ImageError> {
    match image::load_from_memory(bytes) {
        Ok(img) => {
            let resized = if let Some(max) = max_dim {
                img.thumbnail(max, max)
            } else {
                img
            };
            let rgba = resized.to_rgba8();
            let dim = rgba.dimensions();
            Ok((rgba.into_raw(), dim.0, dim.1))
        }
        Err(e) => {
            // Try SVG rendering
            let opt = resvg::usvg::Options::default();
            match resvg::usvg::Tree::from_data(bytes, &opt) {
                Ok(rtree) => {
                    let size = rtree.size();
                    let mut w = size.width();
                    let mut h = size.height();
                    
                    let target_max = max_dim.unwrap_or(256) as f32;
                    let current_max = w.max(h);
                    if current_max > 0.0 {
                        let scale = target_max / current_max;
                        w *= scale;
                        h *= scale;
                    }
                    
                    let w_u32 = w.round() as u32;
                    let h_u32 = h.round() as u32;
                    
                    if w_u32 > 0 && h_u32 > 0 {
                        if let Some(mut pixmap) = resvg::tiny_skia::Pixmap::new(w_u32, h_u32) {
                            let transform = resvg::tiny_skia::Transform::from_scale(
                                w / size.width(),
                                h / size.height(),
                            );
                            resvg::render(&rtree, transform, &mut pixmap.as_mut());
                            return Ok((pixmap.data().to_vec(), w_u32, h_u32));
                        }
                    }
                    Err(e)
                }
                Err(_) => Err(e),
            }
        }
    }
}

pub fn create_texture_bind_group(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bgl: &wgpu::BindGroupLayout,
    bytes: &[u8],
) -> Result<(wgpu::BindGroup, u32, u32), image::ImageError> {
    let (rgba, w, h) = load_image_or_svg(bytes, None)?;
    let size = wgpu::Extent3d {
        width: w,
        height: h,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Image Texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &rgba,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * w),
            rows_per_image: Some(h),
        },
        size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::MipmapFilterMode::Linear,
        ..Default::default()
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
        label: Some("Image Bind Group"),
    });

    Ok((bind_group, w, h))
}

pub fn create_texture_bind_group_thumbnail(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bgl: &wgpu::BindGroupLayout,
    bytes: &[u8],
    max_dim: u32,
) -> Result<(wgpu::BindGroup, u32, u32), image::ImageError> {
    let (rgba, w, h) = load_image_or_svg(bytes, Some(max_dim))?;
    let size = wgpu::Extent3d {
        width: w,
        height: h,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Thumbnail Texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &rgba,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * w),
            rows_per_image: Some(h),
        },
        size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::MipmapFilterMode::Linear,
        ..Default::default()
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
        label: Some("Thumbnail Bind Group"),
    });

    Ok((bind_group, w, h))
}
