use taffy::{
    AlignItems, Dimension, Display, FlexDirection, JustifyContent, LengthPercentage, Style,
};

#[derive(Debug, Clone, Default)]
pub struct FlexConfig {
    pub direction: FlexDirection,
    pub align_items: Option<AlignItems>,
    pub justify_content: Option<JustifyContent>,
    pub gap: f32,
    pub padding: f32,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl FlexConfig {
    pub fn row() -> Self {
        Self {
            direction: FlexDirection::Row,
            ..Default::default()
        }
    }
    pub fn column() -> Self {
        Self {
            direction: FlexDirection::Column,
            ..Default::default()
        }
    }
    pub fn gap(mut self, g: f32) -> Self {
        self.gap = g;
        self
    }
    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }
    pub fn align_items(mut self, a: AlignItems) -> Self {
        self.align_items = Some(a);
        self
    }
    pub fn justify_content(mut self, j: JustifyContent) -> Self {
        self.justify_content = Some(j);
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }

    pub fn into_style(self) -> Style {
        let p = LengthPercentage::length(self.padding);
        Style {
            display: Display::Flex,
            flex_direction: self.direction,
            align_items: self.align_items,
            justify_content: self.justify_content,
            gap: taffy::Size {
                width: LengthPercentage::length(self.gap),
                height: LengthPercentage::length(self.gap),
            },
            padding: taffy::Rect {
                left: p,
                right: p,
                top: p,
                bottom: p,
            },
            size: taffy::Size {
                width: self
                    .width
                    .map(Dimension::length)
                    .unwrap_or(Dimension::auto()),
                height: self
                    .height
                    .map(Dimension::length)
                    .unwrap_or(Dimension::auto()),
            },
            ..Default::default()
        }
    }
}
