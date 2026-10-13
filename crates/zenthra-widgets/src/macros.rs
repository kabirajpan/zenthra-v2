// crates/zenthra-widgets/src/macros.rs

/// Implements layout dimension builder methods (`width`, `height`, `fill_x`, `fill_y`, etc.) on a builder struct with `self.style`.
#[macro_export]
macro_rules! impl_dimensions {
    ($builder:ident) => {
        $crate::impl_dimensions!($builder, 'u, 'a);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        impl<$($lt),+> $builder<$($lt),+> {
            #[inline]
            pub fn width(mut self, w: f32) -> Self {
                self.style.width = Some(w);
                self.style.fill_x = false;
                self
            }

            #[inline]
            pub fn height(mut self, h: f32) -> Self {
                self.style.height = Some(h);
                self.style.fill_y = false;
                self
            }

            #[inline]
            pub fn fill_x(mut self) -> Self {
                self.style.fill_x = true;
                self
            }

            #[inline]
            pub fn fill_y(mut self) -> Self {
                self.style.fill_y = true;
                self
            }

            #[inline]
            pub fn fill(mut self) -> Self {
                self.style.fill_x = true;
                self.style.fill_y = true;
                self
            }


            #[inline]
            pub fn max_width(mut self, w: f32) -> Self {
                self.style.max_width = Some(w);
                self
            }

            #[inline]
            pub fn max_height(mut self, h: f32) -> Self {
                self.style.max_height = Some(h);
                self
            }

            #[inline]
            pub fn min_width(mut self, w: f32) -> Self {
                self.style.min_width = Some(w);
                self
            }

            #[inline]
            pub fn min_height(mut self, h: f32) -> Self {
                self.style.min_height = Some(h);
                self
            }
        }
    };
}

/// Implements padding builder methods on a builder struct with `self.style`.
#[macro_export]
macro_rules! impl_padding {
    ($builder:ident) => {
        $crate::impl_padding!($builder, 'u, 'a);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        impl<$($lt),+> $builder<$($lt),+> {
            #[inline]
            pub fn padding(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
                self.style.padding = zenthra_core::EdgeInsets { top: t, right: r, bottom: b, left: l };
                self
            }

            #[inline]
            pub fn padding_all(mut self, p: f32) -> Self {
                self.style.padding = zenthra_core::EdgeInsets::all(p);
                self
            }

            #[inline]
            pub fn padding_x(mut self, p: f32) -> Self {
                self.style.padding.left = p;
                self.style.padding.right = p;
                self
            }

            #[inline]
            pub fn padding_y(mut self, p: f32) -> Self {
                self.style.padding.top = p;
                self.style.padding.bottom = p;
                self
            }

            #[inline]
            pub fn padding_top(mut self, p: f32) -> Self {
                self.style.padding.top = p;
                self
            }

            #[inline]
            pub fn padding_bottom(mut self, p: f32) -> Self {
                self.style.padding.bottom = p;
                self
            }

            #[inline]
            pub fn padding_left(mut self, p: f32) -> Self {
                self.style.padding.left = p;
                self
            }

            #[inline]
            pub fn padding_right(mut self, p: f32) -> Self {
                self.style.padding.right = p;
                self
            }
        }
    };
}

/// Implements corner radius builder methods on a builder struct with `self.style`.
#[macro_export]
macro_rules! impl_radius {
    ($builder:ident) => {
        $crate::impl_radius!($builder, 'u, 'a);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        impl<$($lt),+> $builder<$($lt),+> {
            #[inline]
            pub fn radius(mut self, tl: f32, tr: f32, br: f32, bl: f32) -> Self {
                self.style.radius = [tl, tr, br, bl];
                self
            }

            #[inline]
            pub fn radius_all(mut self, r: f32) -> Self {
                self.style.radius = [r, r, r, r];
                self
            }

            #[inline]
            pub fn radius_top_left(mut self, r: f32) -> Self {
                self.style.radius[0] = r;
                self
            }

            #[inline]
            pub fn radius_top_right(mut self, r: f32) -> Self {
                self.style.radius[1] = r;
                self
            }

            #[inline]
            pub fn radius_bottom_right(mut self, r: f32) -> Self {
                self.style.radius[2] = r;
                self
            }

            #[inline]
            pub fn radius_bottom_left(mut self, r: f32) -> Self {
                self.style.radius[3] = r;
                self
            }

            #[inline]
            pub fn radius_top(mut self, r: f32) -> Self {
                self.style.radius[0] = r;
                self.style.radius[1] = r;
                self
            }

            #[inline]
            pub fn radius_bottom(mut self, r: f32) -> Self {
                self.style.radius[2] = r;
                self.style.radius[3] = r;
                self
            }

            #[inline]
            pub fn radius_left(mut self, r: f32) -> Self {
                self.style.radius[0] = r;
                self.style.radius[3] = r;
                self
            }

            #[inline]
            pub fn radius_right(mut self, r: f32) -> Self {
                self.style.radius[1] = r;
                self.style.radius[2] = r;
                self
            }

        }
    };
}

/// Implements visual styling builder methods (`bg`, `border`, `shadow`, etc.) on a builder struct with `self.style`.
#[macro_export]
macro_rules! impl_visuals {
    ($builder:ident) => {
        $crate::impl_visuals!($builder, 'u, 'a);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        impl<$($lt),+> $builder<$($lt),+> {
            #[inline]
            pub fn style(mut self, style: zenthra_core::BoxStyle) -> Self {
                self.style = style;
                self
            }

            #[inline]
            pub fn bg(mut self, b: impl Into<zenthra_core::Background>) -> Self {
                match b.into() {
                    zenthra_core::Background::None => self.style.bg = None,
                    other => self.style.bg = Some(other),
                }
                self
            }

            #[inline]
            pub fn bg_gradient(
                mut self,
                start: zenthra_core::Color,
                end: zenthra_core::Color,
                direction: zenthra_core::GradientDirection,
            ) -> Self {
                let (dir_type, angle) = direction.to_type_and_angle();
                if dir_type == 2.0 {
                    self.style.bg = Some(zenthra_core::Background::Gradient(
                        zenthra_core::Gradient::radial(zenthra_core::Align::Center, [start, end]),
                    ));
                } else {
                    self.style.bg = Some(zenthra_core::Background::Gradient(
                        zenthra_core::Gradient::linear(angle.to_degrees(), [start, end]),
                    ));
                }
                self
            }

            #[inline]
            pub fn bg_opacity(mut self, opacity: f32) -> Self {
                if let Some(ref mut bg) = self.style.bg {
                    match bg {
                        zenthra_core::Background::Solid(ref mut c) => c.a = opacity,
                        zenthra_core::Background::Gradient(ref mut g) => match g {
                            zenthra_core::Gradient::Linear { ref mut stops, .. } => {
                                for s in stops.iter_mut() {
                                    s.color.a *= opacity;
                                }
                            }
                            zenthra_core::Gradient::Radial { ref mut stops, .. } => {
                                for s in stops.iter_mut() {
                                    s.color.a *= opacity;
                                }
                            }
                            zenthra_core::Gradient::Mesh { ref mut points } => {
                                for p in points.iter_mut() {
                                    p.1.a *= opacity;
                                }
                            }
                        },
                        zenthra_core::Background::None => {}
                    }
                }
                self
            }

            #[inline]
            pub fn border(mut self, border: impl Into<zenthra_core::Border>) -> Self {
                self.style.border = border.into();
                self
            }

            #[inline]
            pub fn border_alignment(mut self, alignment: zenthra_core::BorderAlignment) -> Self {
                self.style.border_alignment = alignment;
                self
            }

            #[inline]
            pub fn shadow(mut self, color: zenthra_core::Color, x: f32, y: f32, blur: f32) -> Self {
                self.style.shadow_color = Some(color);
                self.style.shadow_offset = [x, y];
                self.style.shadow_blur = blur;
                self
            }

            #[inline]
            pub fn shadow_opacity(mut self, opacity: f32) -> Self {
                self.style.shadow_opacity = opacity;
                self
            }

            #[inline]
            pub fn opacity(mut self, o: f32) -> Self {
                self.style.opacity = o;
                self
            }
        }
    };
}

/// Comprehensive macro that implements all dimensions, padding, radius, and visual box styling methods.
#[macro_export]
macro_rules! impl_box_style_methods {
    ($builder:ident) => {
        $crate::impl_dimensions!($builder);
        $crate::impl_padding!($builder);
        $crate::impl_radius!($builder);
        $crate::impl_visuals!($builder);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        $crate::impl_dimensions!($builder, $($lt),+);
        $crate::impl_padding!($builder, $($lt),+);
        $crate::impl_radius!($builder, $($lt),+);
        $crate::impl_visuals!($builder, $($lt),+);
    };
}

/// Implements core typography builder methods (font_size, color, family, weight, bold, italic, line_height)
#[macro_export]
macro_rules! impl_font_style_methods {
    ($builder:ident) => {
        $crate::impl_font_style_methods!($builder, 'u, 'a);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        impl<$($lt),+> $builder<$($lt),+> {
            #[inline]
            pub fn text_style(mut self, style: zenthra_text::prelude::TextStyle) -> Self {
                self.text_style = style;
                self
            }

            #[inline]
            pub fn font_size(mut self, size: f32) -> Self {
                self.text_style.font_size = size;
                self
            }

            #[inline]
            pub fn color(mut self, color: zenthra_core::Color) -> Self {
                self.text_style.color = color;
                self
            }

            #[inline]
            pub fn text_color(self, color: zenthra_core::Color) -> Self {
                self.color(color)
            }

            #[inline]
            pub fn family(mut self, family: impl Into<String>) -> Self {
                self.text_style.font_family = Some(family.into());
                self
            }

            #[inline]
            pub fn font_family(self, family: impl Into<String>) -> Self {
                self.family(family)
            }

            #[inline]
            pub fn monospace(mut self) -> Self {
                self.text_style.font_family = Some("monospace".to_string());
                self
            }

            #[inline]
            pub fn weight(mut self, weight: impl Into<zenthra_text::prelude::FontWeight>) -> Self {
                self.text_style.font_weight = weight.into();
                self
            }

            #[inline]
            pub fn font_weight(self, weight: impl Into<zenthra_text::prelude::FontWeight>) -> Self {
                self.weight(weight)
            }

            #[inline]
            pub fn bold(mut self) -> Self {
                self.text_style.font_weight = zenthra_text::prelude::FontWeight::Bold;
                self
            }

            #[inline]
            pub fn bold_when(mut self, bold: bool) -> Self {
                if bold {
                    self.text_style.font_weight = zenthra_text::prelude::FontWeight::Bold;
                }
                self
            }

            #[inline]
            pub fn font_style(mut self, style: zenthra_text::prelude::FontStyle) -> Self {
                self.text_style.font_style = style;
                self
            }

            #[inline]
            pub fn italic(mut self) -> Self {
                self.text_style.font_style = zenthra_text::prelude::FontStyle::Italic;
                self
            }

            #[inline]
            pub fn italic_when(mut self, italic: bool) -> Self {
                if italic {
                    self.text_style.font_style = zenthra_text::prelude::FontStyle::Italic;
                }
                self
            }

            #[inline]
            pub fn line_height(mut self, lh: f32) -> Self {
                self.text_style.line_height = lh;
                self
            }
        }
    };
}

/// Implements text wrapping flow builder methods (wrap, auto_wrap, no_wrap)
#[macro_export]
macro_rules! impl_text_wrap_methods {
    ($builder:ident) => {
        $crate::impl_text_wrap_methods!($builder, 'u, 'a);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        impl<$($lt),+> $builder<$($lt),+> {
            #[inline]
            pub fn wrap(mut self, wrap: impl Into<zenthra_text::prelude::TextWrap>) -> Self {
                self.text_style.wrap = wrap.into();
                self
            }

            #[inline]
            pub fn auto_wrap(mut self) -> Self {
                self.text_style.wrap = zenthra_text::prelude::TextWrap::Word;
                self
            }

            #[inline]
            pub fn no_wrap(mut self) -> Self {
                self.text_style.wrap = zenthra_text::prelude::TextWrap::None;
                self
            }
        }
    };
}

/// Implements text decoration builder methods (ellipsis, underline, strikethrough, highlight)
#[macro_export]
macro_rules! impl_text_decorations {
    ($builder:ident) => {
        $crate::impl_text_decorations!($builder, 'u, 'a);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        impl<$($lt),+> $builder<$($lt),+> {
            #[inline]
            pub fn highlight(mut self, color: zenthra_core::Color) -> Self {
                self.text_style.highlight_color = Some(color);
                self
            }

            #[inline]
            pub fn ellipsis(mut self, ellipsis: bool) -> Self {
                self.text_style.ellipsis = ellipsis;
                self
            }

            #[inline]
            pub fn underline(mut self, underline: bool) -> Self {
                self.text_style.underline = underline;
                self
            }

            #[inline]
            pub fn underline_color(mut self, color: zenthra_core::Color) -> Self {
                self.text_style.underline = true;
                self.text_style.underline_color = Some(color);
                self
            }

            #[inline]
            pub fn strikethrough(mut self, strikethrough: bool) -> Self {
                self.text_style.strikethrough = strikethrough;
                self
            }

            #[inline]
            pub fn strikethrough_color(mut self, color: zenthra_core::Color) -> Self {
                self.text_style.strikethrough = true;
                self.text_style.strikethrough_color = Some(color);
                self
            }
        }
    };
}

/// Implements full typography styling, wrapping, and decoration builder methods
#[macro_export]
macro_rules! impl_text_style_methods {
    ($builder:ident) => {
        $crate::impl_font_style_methods!($builder);
        $crate::impl_text_wrap_methods!($builder);
        $crate::impl_text_decorations!($builder);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        $crate::impl_font_style_methods!($builder, $($lt),+);
        $crate::impl_text_wrap_methods!($builder, $($lt),+);
        $crate::impl_text_decorations!($builder, $($lt),+);
    };
}
