use taffy::{
    Display, GridTemplateComponent, LengthPercentage, MaxTrackSizingFunction,
    MinTrackSizingFunction, Style, TrackSizingFunction,
};

#[derive(Debug, Clone)]
pub struct GridConfig {
    pub columns: usize,
    pub gap: f32,
    pub padding: f32,
}

impl GridConfig {
    pub fn new(columns: usize) -> Self {
        Self {
            columns,
            gap: 0.0,
            padding: 0.0,
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

    pub fn into_style(self) -> Style {
        let track = TrackSizingFunction {
            min: MinTrackSizingFunction::auto(),
            max: MaxTrackSizingFunction::fr(1.0),
        };

        // ✅ wrap each track in GridTemplateComponent::Single
        let tracks = vec![GridTemplateComponent::Single(track); self.columns];

        let p = LengthPercentage::length(self.padding);
        Style {
            display: Display::Grid,
            grid_template_columns: tracks,
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
            ..Default::default()
        }
    }
}
