// crates/zenthra-widgets/src/macros.rs

/// Re-export IntoBorder from zenthra_core as the canonical single source of truth.
pub use zenthra_core::IntoBorder;

/// Visual text embellishments and styles for `.decoration(...)`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextDecoration {
    Underline,
    Strikethrough,
    Italic,
    Highlight(zenthra_core::Color),
    LineHeight(f32),
}

/// Trait to support single decoration, tuple `(Decoration, Color)`, or arrays `[...]`
pub trait IntoDecoration {
    fn apply_decoration(self, style: &mut zenthra_text::prelude::TextStyle);
}

impl IntoDecoration for TextDecoration {
    fn apply_decoration(self, style: &mut zenthra_text::prelude::TextStyle) {
        match self {
            TextDecoration::Underline => {
                style.underline = true;
            }
            TextDecoration::Strikethrough => {
                style.strikethrough = true;
            }
            TextDecoration::Italic => {
                style.font_style = zenthra_text::prelude::FontStyle::Italic;
            }
            TextDecoration::Highlight(c) => {
                style.highlight_color = Some(c);
            }
            TextDecoration::LineHeight(lh) => {
                style.line_height = lh;
            }
        }
    }
}

impl IntoDecoration for (TextDecoration, zenthra_core::Color) {
    fn apply_decoration(self, style: &mut zenthra_text::prelude::TextStyle) {
        match self.0 {
            TextDecoration::Underline => {
                style.underline = true;
                style.underline_color = Some(self.1);
            }
            TextDecoration::Strikethrough => {
                style.strikethrough = true;
                style.strikethrough_color = Some(self.1);
            }
            TextDecoration::Highlight(_) => {
                style.highlight_color = Some(self.1);
            }
            other => other.apply_decoration(style),
        }
    }
}

impl<T: IntoDecoration, const N: usize> IntoDecoration for [T; N] {
    fn apply_decoration(self, style: &mut zenthra_text::prelude::TextStyle) {
        for item in self {
            item.apply_decoration(style);
        }
    }
}

impl<T: IntoDecoration> IntoDecoration for Vec<T> {
    fn apply_decoration(self, style: &mut zenthra_text::prelude::TextStyle) {
        for item in self {
            item.apply_decoration(style);
        }
    }
}

impl<T: IntoDecoration + Copy> IntoDecoration for &[T] {
    fn apply_decoration(self, style: &mut zenthra_text::prelude::TextStyle) {
        for &item in self {
            item.apply_decoration(style);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SIBLING INTERACTION & STATE MODIFIERS (Hover, Active, Focus)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationCurve {
    EaseOut,
    EaseInOut,
    Linear,
    Spring,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationLoop {
    Pulse,
    Shimmer,
    Bounce,
    Spin,
}

/// Unified animation configuration for state transitions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnimationConfig {
    pub duration_ms: u32,
    pub delay_ms: u32,
    pub ease: AnimationCurve,
    pub loop_effect: Option<AnimationLoop>,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration_ms: 150,
            delay_ms: 0,
            ease: AnimationCurve::EaseOut,
            loop_effect: None,
        }
    }
}

/// Unified property enum representing any property that can change during interaction states.
#[derive(Debug, Clone, PartialEq)]
pub enum StateProp {
    // 1. Box Style
    Bg(zenthra_core::Background),
    Border(zenthra_core::Border, Option<zenthra_core::BorderAlignment>),
    Shadow(zenthra_core::Color, [f32; 2], f32),
    Radius([f32; 4]),
    Padding(zenthra_core::EdgeInsets),
    Margin(zenthra_core::EdgeInsets),
    Width(f32),
    Height(f32),
    Opacity(f32),
    BackdropBlur(f32),

    // 2. Typography
    Color(zenthra_core::Color),
    FontSize(f32),
    Underline(bool),
    FontWeight(zenthra_text::prelude::FontWeight),

    // 3. Transform
    Scale(f32),

    // 4. Interaction
    Cursor(crate::text::CursorIcon),
    Tooltip(String),

    // 5. Animation
    Animation(AnimationConfig),
    Duration(u32),
    Delay(u32),
    Ease(AnimationCurve),
    LoopEffect(AnimationLoop),
}

impl StateProp {
    #[inline]
    pub fn delay(mut self, ms: u32) -> Self {
        match &mut self {
            StateProp::Animation(ref mut cfg) => cfg.delay_ms = ms,
            StateProp::Duration(d) => {
                return StateProp::Animation(AnimationConfig {
                    duration_ms: *d,
                    delay_ms: ms,
                    ease: AnimationCurve::EaseOut,
                    loop_effect: None,
                });
            }
            _ => return StateProp::Delay(ms),
        }
        self
    }

    #[inline]
    pub fn ease_out(mut self) -> Self {
        match &mut self {
            StateProp::Animation(ref mut cfg) => cfg.ease = AnimationCurve::EaseOut,
            StateProp::Duration(d) => {
                return StateProp::Animation(AnimationConfig {
                    duration_ms: *d,
                    delay_ms: 0,
                    ease: AnimationCurve::EaseOut,
                    loop_effect: None,
                });
            }
            _ => return StateProp::Ease(AnimationCurve::EaseOut),
        }
        self
    }

    #[inline]
    pub fn ease_in_out(mut self) -> Self {
        match &mut self {
            StateProp::Animation(ref mut cfg) => cfg.ease = AnimationCurve::EaseInOut,
            StateProp::Duration(d) => {
                return StateProp::Animation(AnimationConfig {
                    duration_ms: *d,
                    delay_ms: 0,
                    ease: AnimationCurve::EaseInOut,
                    loop_effect: None,
                });
            }
            _ => return StateProp::Ease(AnimationCurve::EaseInOut),
        }
        self
    }

    #[inline]
    pub fn linear(mut self) -> Self {
        match &mut self {
            StateProp::Animation(ref mut cfg) => cfg.ease = AnimationCurve::Linear,
            StateProp::Duration(d) => {
                return StateProp::Animation(AnimationConfig {
                    duration_ms: *d,
                    delay_ms: 0,
                    ease: AnimationCurve::Linear,
                    loop_effect: None,
                });
            }
            _ => return StateProp::Ease(AnimationCurve::Linear),
        }
        self
    }

    #[inline]
    pub fn spring(mut self) -> Self {
        match &mut self {
            StateProp::Animation(ref mut cfg) => cfg.ease = AnimationCurve::Spring,
            StateProp::Duration(d) => {
                return StateProp::Animation(AnimationConfig {
                    duration_ms: *d,
                    delay_ms: 0,
                    ease: AnimationCurve::Spring,
                    loop_effect: None,
                });
            }
            _ => return StateProp::Ease(AnimationCurve::Spring),
        }
        self
    }

    #[inline]
    pub fn pulse(mut self) -> Self {
        match &mut self {
            StateProp::Animation(ref mut cfg) => cfg.loop_effect = Some(AnimationLoop::Pulse),
            StateProp::Duration(d) => {
                return StateProp::Animation(AnimationConfig {
                    duration_ms: *d,
                    delay_ms: 0,
                    ease: AnimationCurve::EaseOut,
                    loop_effect: Some(AnimationLoop::Pulse),
                });
            }
            _ => return StateProp::LoopEffect(AnimationLoop::Pulse),
        }
        self
    }

    #[inline]
    pub fn shimmer(mut self) -> Self {
        match &mut self {
            StateProp::Animation(ref mut cfg) => cfg.loop_effect = Some(AnimationLoop::Shimmer),
            StateProp::Duration(d) => {
                return StateProp::Animation(AnimationConfig {
                    duration_ms: *d,
                    delay_ms: 0,
                    ease: AnimationCurve::EaseOut,
                    loop_effect: Some(AnimationLoop::Shimmer),
                });
            }
            _ => return StateProp::LoopEffect(AnimationLoop::Shimmer),
        }
        self
    }

    #[inline]
    pub fn bounce(mut self) -> Self {
        match &mut self {
            StateProp::Animation(ref mut cfg) => cfg.loop_effect = Some(AnimationLoop::Bounce),
            StateProp::Duration(d) => {
                return StateProp::Animation(AnimationConfig {
                    duration_ms: *d,
                    delay_ms: 0,
                    ease: AnimationCurve::EaseOut,
                    loop_effect: Some(AnimationLoop::Bounce),
                });
            }
            _ => return StateProp::LoopEffect(AnimationLoop::Bounce),
        }
        self
    }

    #[inline]
    pub fn spin(mut self) -> Self {
        match &mut self {
            StateProp::Animation(ref mut cfg) => cfg.loop_effect = Some(AnimationLoop::Spin),
            StateProp::Duration(d) => {
                return StateProp::Animation(AnimationConfig {
                    duration_ms: *d,
                    delay_ms: 0,
                    ease: AnimationCurve::EaseOut,
                    loop_effect: Some(AnimationLoop::Spin),
                });
            }
            _ => return StateProp::LoopEffect(AnimationLoop::Spin),
        }
        self
    }
}

/// Sibling 1: `Style` (Surfaces, Borders, Shadows, Geometry, Spacing)
pub struct Style;

impl Style {
    #[inline]
    pub fn bg(bg: impl Into<zenthra_core::Background>) -> StateProp {
        StateProp::Bg(bg.into())
    }

    #[inline]
    pub fn border(border: impl zenthra_core::IntoBorder) -> StateProp {
        let (b, align) = border.into_border();
        StateProp::Border(b, align)
    }

    #[inline]
    pub fn shadow(color: zenthra_core::Color, x: f32, y: f32, blur: f32) -> StateProp {
        StateProp::Shadow(color, [x, y], blur)
    }

    #[inline]
    pub fn radius_all(r: f32) -> StateProp {
        StateProp::Radius([r; 4])
    }

    #[inline]
    pub fn radius(tl: f32, tr: f32, br: f32, bl: f32) -> StateProp {
        StateProp::Radius([tl, tr, br, bl])
    }

    #[inline]
    pub fn radius_top(r: f32) -> StateProp {
        StateProp::Radius([r, r, 0.0, 0.0])
    }

    #[inline]
    pub fn radius_bottom(r: f32) -> StateProp {
        StateProp::Radius([0.0, 0.0, r, r])
    }

    #[inline]
    pub fn radius_left(r: f32) -> StateProp {
        StateProp::Radius([r, 0.0, 0.0, r])
    }

    #[inline]
    pub fn radius_right(r: f32) -> StateProp {
        StateProp::Radius([0.0, r, r, 0.0])
    }

    #[inline]
    pub fn padding_all(p: f32) -> StateProp {
        StateProp::Padding(zenthra_core::EdgeInsets::all(p))
    }

    #[inline]
    pub fn padding(t: f32, r: f32, b: f32, l: f32) -> StateProp {
        StateProp::Padding(zenthra_core::EdgeInsets { top: t, right: r, bottom: b, left: l })
    }

    #[inline]
    pub fn padding_x(p: f32) -> StateProp {
        StateProp::Padding(zenthra_core::EdgeInsets::symmetric(0.0, p))
    }

    #[inline]
    pub fn padding_y(p: f32) -> StateProp {
        StateProp::Padding(zenthra_core::EdgeInsets::symmetric(p, 0.0))
    }

    #[inline]
    pub fn margin_all(m: f32) -> StateProp {
        StateProp::Margin(zenthra_core::EdgeInsets::all(m))
    }

    #[inline]
    pub fn margin(t: f32, r: f32, b: f32, l: f32) -> StateProp {
        StateProp::Margin(zenthra_core::EdgeInsets { top: t, right: r, bottom: b, left: l })
    }

    #[inline]
    pub fn margin_x(m: f32) -> StateProp {
        StateProp::Margin(zenthra_core::EdgeInsets::symmetric(0.0, m))
    }

    #[inline]
    pub fn margin_y(m: f32) -> StateProp {
        StateProp::Margin(zenthra_core::EdgeInsets::symmetric(m, 0.0))
    }

    #[inline]
    pub fn width(w: f32) -> StateProp {
        StateProp::Width(w)
    }

    #[inline]
    pub fn height(h: f32) -> StateProp {
        StateProp::Height(h)
    }

    #[inline]
    pub fn opacity(a: f32) -> StateProp {
        StateProp::Opacity(a)
    }

    #[inline]
    pub fn backdrop_blur(radius: f32) -> StateProp {
        StateProp::BackdropBlur(radius)
    }
}

/// Sibling 2: `Text` (Typography)
pub struct Text;

impl Text {
    #[inline]
    pub fn color(c: zenthra_core::Color) -> StateProp {
        StateProp::Color(c)
    }

    #[inline]
    pub fn size(s: f32) -> StateProp {
        StateProp::FontSize(s)
    }

    #[inline]
    pub fn underline(u: bool) -> StateProp {
        StateProp::Underline(u)
    }

    #[inline]
    pub fn font_weight(w: impl Into<zenthra_text::prelude::FontWeight>) -> StateProp {
        StateProp::FontWeight(w.into())
    }
}

/// Sibling 3: `Transform` (Visual Pop & Physics)
pub struct Transform;

impl Transform {
    #[inline]
    pub fn scale(s: f32) -> StateProp {
        StateProp::Scale(s)
    }
}

/// Sibling 4: `Cursor` (Mouse Cursor Feedback)
pub struct Cursor;

#[allow(non_upper_case_globals)]
impl Cursor {
    pub const Pointer: StateProp = StateProp::Cursor(crate::text::CursorIcon::Pointer);
    pub const Grab: StateProp = StateProp::Cursor(crate::text::CursorIcon::Grab);
    pub const Grabbing: StateProp = StateProp::Cursor(crate::text::CursorIcon::Grabbing);
    pub const Text: StateProp = StateProp::Cursor(crate::text::CursorIcon::Text);
    pub const Crosshair: StateProp = StateProp::Cursor(crate::text::CursorIcon::Crosshair);
    pub const ColResize: StateProp = StateProp::Cursor(crate::text::CursorIcon::ColResize);
    pub const RowResize: StateProp = StateProp::Cursor(crate::text::CursorIcon::RowResize);
    pub const Default: StateProp = StateProp::Cursor(crate::text::CursorIcon::Default);

    #[inline]
    pub fn icon(icon: crate::text::CursorIcon) -> StateProp {
        StateProp::Cursor(icon)
    }
}

/// Sibling 5: `Tooltip` (Hover Hint Popups)
pub struct Tooltip;

impl Tooltip {
    #[inline]
    pub fn new(msg: impl Into<String>) -> StateProp {
        StateProp::Tooltip(msg.into())
    }
}

/// Sibling 6: `Animation` (Transitions, Springs & Micro-Effects)
pub struct Animation;

impl Animation {
    #[inline]
    pub fn duration(ms: u32) -> StateProp {
        StateProp::Animation(AnimationConfig {
            duration_ms: ms,
            delay_ms: 0,
            ease: AnimationCurve::EaseOut,
            loop_effect: None,
        })
    }

    #[inline]
    pub fn delay(ms: u32) -> StateProp {
        StateProp::Delay(ms)
    }

    #[inline]
    pub fn ease_out() -> StateProp {
        StateProp::Ease(AnimationCurve::EaseOut)
    }

    #[inline]
    pub fn ease_in_out() -> StateProp {
        StateProp::Ease(AnimationCurve::EaseInOut)
    }

    #[inline]
    pub fn linear() -> StateProp {
        StateProp::Ease(AnimationCurve::Linear)
    }

    #[inline]
    pub fn spring() -> StateProp {
        StateProp::Ease(AnimationCurve::Spring)
    }

    #[inline]
    pub fn pulse() -> StateProp {
        StateProp::LoopEffect(AnimationLoop::Pulse)
    }

    #[inline]
    pub fn shimmer() -> StateProp {
        StateProp::LoopEffect(AnimationLoop::Shimmer)
    }

    #[inline]
    pub fn bounce() -> StateProp {
        StateProp::LoopEffect(AnimationLoop::Bounce)
    }

    #[inline]
    pub fn spin() -> StateProp {
        StateProp::LoopEffect(AnimationLoop::Spin)
    }

    #[inline]
    pub fn none() -> StateProp {
        StateProp::Duration(0)
    }
}

/// Free helper to create a tooltip directly: `tooltip("...")`
#[inline]
pub fn tooltip(msg: impl Into<String>) -> StateProp {
    Tooltip::new(msg)
}

/// Free helper to create a scale directly: `scale(1.03)`
#[inline]
pub fn scale(s: f32) -> StateProp {
    Transform::scale(s)
}

/// Stored overrides to apply when a widget is in a state (hover, active, focus).
#[derive(Debug, Clone, PartialEq)]
pub struct WidgetStateOverrides {
    pub style: zenthra_core::BoxStyle,
    pub text_color: Option<zenthra_core::Color>,
    pub font_size: Option<f32>,
    pub underline: Option<bool>,
    pub font_weight: Option<zenthra_text::prelude::FontWeight>,
    pub scale: f32,
    pub cursor: Option<crate::text::CursorIcon>,
    pub tooltip: Option<String>,
    pub duration_ms: Option<u32>,
    pub delay_ms: Option<u32>,
    pub ease: Option<AnimationCurve>,
    pub loop_effect: Option<AnimationLoop>,
    pub backdrop_blur: Option<f32>,
    pub has_overrides: bool,
}

impl Default for WidgetStateOverrides {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetStateOverrides {
    pub fn new() -> Self {
        Self {
            style: zenthra_core::BoxStyle::new(),
            text_color: None,
            font_size: None,
            underline: None,
            font_weight: None,
            scale: 1.0,
            cursor: None,
            tooltip: None,
            duration_ms: None,
            delay_ms: None,
            ease: None,
            loop_effect: None,
            backdrop_blur: None,
            has_overrides: false,
        }
    }

    pub fn apply_prop(&mut self, prop: StateProp) {
        self.has_overrides = true;
        match prop {
            StateProp::Bg(bg) => self.style.bg = Some(bg),
            StateProp::Border(b, align) => {
                self.style.border = b;
                if let Some(a) = align {
                    self.style.border_alignment = a;
                }
            }
            StateProp::Shadow(c, off, blur) => {
                self.style.shadow_color = Some(c);
                self.style.shadow_offset = off;
                self.style.shadow_blur = blur;
            }
            StateProp::Radius(r) => self.style.radius = r,
            StateProp::Padding(p) => self.style.padding = p,
            StateProp::Margin(m) => self.style.margin = m,
            StateProp::Width(w) => self.style.width = Some(w),
            StateProp::Height(h) => self.style.height = Some(h),
            StateProp::Opacity(o) => self.style.opacity = o,
            StateProp::BackdropBlur(r) => self.backdrop_blur = Some(r),
            StateProp::Color(c) => self.text_color = Some(c),
            StateProp::FontSize(s) => self.font_size = Some(s),
            StateProp::Underline(u) => self.underline = Some(u),
            StateProp::FontWeight(w) => self.font_weight = Some(w),
            StateProp::Scale(s) => self.scale = s,
            StateProp::Cursor(c) => self.cursor = Some(c),
            StateProp::Tooltip(t) => self.tooltip = Some(t),
            StateProp::Animation(cfg) => {
                self.duration_ms = Some(cfg.duration_ms);
                self.delay_ms = Some(cfg.delay_ms);
                self.ease = Some(cfg.ease);
                self.loop_effect = cfg.loop_effect;
            }
            StateProp::Duration(d) => self.duration_ms = Some(d),
            StateProp::Delay(d) => self.delay_ms = Some(d),
            StateProp::Ease(e) => self.ease = Some(e),
            StateProp::LoopEffect(l) => self.loop_effect = Some(l),
        }
    }
}

/// Trait to support single `StateProp`, fixed array `[StateProp; N]`, or `Vec<StateProp>`
pub trait IntoStateProps {
    fn apply_to_overrides(self, target: &mut WidgetStateOverrides);
}

impl IntoStateProps for StateProp {
    fn apply_to_overrides(self, target: &mut WidgetStateOverrides) {
        target.apply_prop(self);
    }
}

impl<const N: usize> IntoStateProps for [StateProp; N] {
    fn apply_to_overrides(self, target: &mut WidgetStateOverrides) {
        for prop in self {
            target.apply_prop(prop);
        }
    }
}

impl IntoStateProps for Vec<StateProp> {
    fn apply_to_overrides(self, target: &mut WidgetStateOverrides) {
        for prop in self {
            target.apply_prop(prop);
        }
    }
}

impl IntoStateProps for &[StateProp] {
    fn apply_to_overrides(self, target: &mut WidgetStateOverrides) {
        for prop in self {
            target.apply_prop(prop.clone());
        }
    }
}

impl From<crate::text::CursorIcon> for StateProp {
    fn from(c: crate::text::CursorIcon) -> Self {
        StateProp::Cursor(c)
    }
}

impl From<zenthra_core::Color> for StateProp {
    fn from(c: zenthra_core::Color) -> Self {
        StateProp::Bg(zenthra_core::Background::Solid(c))
    }
}

/// Implements `.hover(...)` and `.active(...)` methods on builder structs with `hover_state` and `active_state`.
#[macro_export]
macro_rules! impl_interaction_state_methods {
    ($builder:ident) => {
        $crate::impl_interaction_state_methods!($builder, 'u, 'a);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        impl<$($lt),+> $builder<$($lt),+> {
            #[inline]
            pub fn hover(mut self, props: impl $crate::macros::IntoStateProps) -> Self {
                if self.hover_state.is_none() {
                    let mut s = $crate::macros::WidgetStateOverrides::new();
                    s.style = self.style.clone();
                    self.hover_state = Some(s);
                }
                if let Some(ref mut h) = self.hover_state {
                    props.apply_to_overrides(h);
                }
                self
            }

            #[inline]
            pub fn active(mut self, props: impl $crate::macros::IntoStateProps) -> Self {
                if self.active_state.is_none() {
                    let mut s = $crate::macros::WidgetStateOverrides::new();
                    s.style = self.style.clone();
                    self.active_state = Some(s);
                }
                if let Some(ref mut a) = self.active_state {
                    props.apply_to_overrides(a);
                }
                self
            }
        }
    };
}

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

/// Implements margin builder methods on a builder struct with `self.style`.
#[macro_export]
macro_rules! impl_margin {
    ($builder:ident) => {
        $crate::impl_margin!($builder, 'u, 'a);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        impl<$($lt),+> $builder<$($lt),+> {
            #[inline]
            pub fn margin(mut self, t: f32, r: f32, b: f32, l: f32) -> Self {
                self.style.margin = zenthra_core::EdgeInsets { top: t, right: r, bottom: b, left: l };
                self
            }

            #[inline]
            pub fn margin_all(mut self, m: f32) -> Self {
                self.style.margin = zenthra_core::EdgeInsets::all(m);
                self
            }

            #[inline]
            pub fn margin_x(mut self, m: f32) -> Self {
                self.style.margin.left = m;
                self.style.margin.right = m;
                self
            }

            #[inline]
            pub fn margin_y(mut self, m: f32) -> Self {
                self.style.margin.top = m;
                self.style.margin.bottom = m;
                self
            }

            #[inline]
            pub fn margin_top(mut self, m: f32) -> Self {
                self.style.margin.top = m;
                self
            }

            #[inline]
            pub fn margin_bottom(mut self, m: f32) -> Self {
                self.style.margin.bottom = m;
                self
            }

            #[inline]
            pub fn margin_left(mut self, m: f32) -> Self {
                self.style.margin.left = m;
                self
            }

            #[inline]
            pub fn margin_right(mut self, m: f32) -> Self {
                self.style.margin.right = m;
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
            pub fn border(mut self, border: impl $crate::macros::IntoBorder) -> Self {
                let (b, align) = border.into_border();
                self.style.border = b;
                if let Some(align) = align {
                    self.style.border_alignment = align;
                }
                self
            }

            #[inline]
            pub fn shadow(mut self, color: zenthra_core::Color, x: f32, y: f32, blur: f32) -> Self {
                self.style.shadow_color = Some(color);
                self.style.shadow_offset = [x, y];
                self.style.shadow_blur = blur;
                self
            }
        }
    };
}

/// Comprehensive macro that implements all dimensions, padding, margin, radius, and visual box styling methods.
#[macro_export]
macro_rules! impl_box_style_methods {
    ($builder:ident) => {
        $crate::impl_dimensions!($builder);
        $crate::impl_padding!($builder);
        $crate::impl_margin!($builder);
        $crate::impl_radius!($builder);
        $crate::impl_visuals!($builder);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        $crate::impl_dimensions!($builder, $($lt),+);
        $crate::impl_padding!($builder, $($lt),+);
        $crate::impl_margin!($builder, $($lt),+);
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
            pub fn size(mut self, size: f32) -> Self {
                self.text_style.font_size = size;
                self
            }

            #[inline]
            pub fn color(mut self, color: zenthra_core::Color) -> Self {
                self.text_style.color = color;
                self
            }

            #[inline]
            pub fn font_family(mut self, family: impl Into<String>) -> Self {
                self.text_style.font_family = Some(family.into());
                self
            }

            #[inline]
            pub fn font_weight(mut self, weight: impl Into<zenthra_text::prelude::FontWeight>) -> Self {
                self.text_style.font_weight = weight.into();
                self
            }
        }
    };
}

/// Implements text wrapping flow builder methods (wrap)
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
        }
    };
}

/// Implements text decoration builder methods (decoration, ellipsis)
#[macro_export]
macro_rules! impl_text_decorations {
    ($builder:ident) => {
        $crate::impl_text_decorations!($builder, 'u, 'a);
    };
    ($builder:ident, $($lt:lifetime),+) => {
        impl<$($lt),+> $builder<$($lt),+> {
            #[inline]
            pub fn decoration(mut self, d: impl $crate::macros::IntoDecoration) -> Self {
                d.apply_decoration(&mut self.text_style);
                self
            }

            #[inline]
            pub fn ellipsis(mut self, ellipsis: bool) -> Self {
                self.text_style.ellipsis = ellipsis;
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

#[cfg(test)]
mod tests {
    use super::*;
    use zenthra_core::{BoxStyle, Color, Styleable};
    use zenthra_text::prelude::{FontWeight, TextStyle};

    #[allow(dead_code)]
    struct TestBoxWidget<'u, 'a> {
        _marker: std::marker::PhantomData<(&'u (), &'a ())>,
        pub style: BoxStyle,
        pub hover_state: Option<WidgetStateOverrides>,
        pub active_state: Option<WidgetStateOverrides>,
    }

    impl<'u, 'a> Styleable for TestBoxWidget<'u, 'a> {
        fn box_style_mut(&mut self) -> &mut BoxStyle { &mut self.style }
        fn box_style(&self) -> &BoxStyle { &self.style }
    }

    impl_box_style_methods!(TestBoxWidget);
    impl_interaction_state_methods!(TestBoxWidget);

    #[allow(dead_code)]
    struct TestTextWidget<'u, 'a> {
        _marker: std::marker::PhantomData<(&'u (), &'a ())>,
        pub text_style: TextStyle,
    }

    impl_text_style_methods!(TestTextWidget);

    #[test]
    fn test_box_style_methods_and_siblings() {
        let mut widget = TestBoxWidget {
            _marker: std::marker::PhantomData,
            style: BoxStyle::new(),
            hover_state: None,
            active_state: None,
        };

        widget = widget
            .style(BoxStyle::new())
            .width(200.0)
            .height(100.0)
            .fill_x()
            .fill_y()
            .fill()
            .min_width(50.0)
            .max_width(500.0)
            .min_height(30.0)
            .max_height(300.0)
            .padding(10.0, 15.0, 20.0, 25.0)
            .padding_all(8.0)
            .padding_x(12.0)
            .padding_y(16.0)
            .padding_top(2.0)
            .padding_bottom(4.0)
            .padding_left(6.0)
            .padding_right(8.0)
            .margin(1.0, 2.0, 3.0, 4.0)
            .margin_all(5.0)
            .margin_x(10.0)
            .margin_y(15.0)
            .margin_top(1.0)
            .margin_bottom(2.0)
            .margin_left(3.0)
            .margin_right(4.0)
            .radius(1.0, 2.0, 3.0, 4.0)
            .radius_all(8.0)
            .radius_top_left(2.0)
            .radius_top_right(3.0)
            .radius_bottom_right(4.0)
            .radius_bottom_left(5.0)
            .radius_top(6.0)
            .radius_bottom(7.0)
            .radius_left(8.0)
            .radius_right(9.0)
            .bg(Color::rgb(0.1, 0.2, 0.3))
            .border((Color::WHITE, 2.0))
            .shadow(Color::rgba(0.0, 0.0, 0.0, 0.5), 0.0, 4.0, 8.0)
            .hover([
                Style::bg(Color::rgb(0.2, 0.3, 0.4)),
                Style::border((Color::YELLOW, 3.0)),
                Style::shadow(Color::BLACK, 0.0, 8.0, 16.0),
                Style::radius_all(12.0),
                Transform::scale(1.05),
                Cursor::Pointer,
                Tooltip::new("Hover tooltip"),
            ])
            .active([
                Transform::scale(0.95),
                Style::opacity(0.8),
            ]);

        assert_eq!(widget.style.min_width, Some(50.0));
        assert_eq!(widget.style.max_width, Some(500.0));
        assert_eq!(widget.style.min_height, Some(30.0));
        assert_eq!(widget.style.max_height, Some(300.0));
        assert!(widget.hover_state.is_some());
        assert!(widget.active_state.is_some());

        let hover = widget.hover_state.unwrap();
        assert_eq!(hover.scale, 1.05);
        assert_eq!(hover.cursor, Some(crate::text::CursorIcon::Pointer));
        assert_eq!(hover.tooltip, Some("Hover tooltip".to_string()));

        let active = widget.active_state.unwrap();
        assert_eq!(active.scale, 0.95);
        assert_eq!(active.style.opacity, 0.8);
    }

    #[test]
    fn test_text_style_methods() {
        let mut widget = TestTextWidget {
            _marker: std::marker::PhantomData,
            text_style: TextStyle::default(),
        };

        widget = widget
            .size(16.0)
            .color(Color::rgb(0.9, 0.9, 0.9))
            .font_family("Inter")
            .font_weight(FontWeight::Bold)
            .wrap(zenthra_text::prelude::TextWrap::Word)
            .ellipsis(true)
            .decoration(TextDecoration::Underline);

        assert_eq!(widget.text_style.font_size, 16.0);
        assert_eq!(widget.text_style.color, Color::rgb(0.9, 0.9, 0.9));
        assert_eq!(widget.text_style.font_family, Some("Inter".to_string()));
        assert_eq!(widget.text_style.font_weight, FontWeight::Bold);
        assert!(widget.text_style.ellipsis);
        assert!(widget.text_style.underline);
    }
}
