// crates/zenthra-widgets/src/containers/panel.rs

use crate::ui::Ui;
use zenthra_core::{Color, Id, Align};

pub struct PanelBuilder<'u, 'a, 'b> {
    ui: &'u mut Ui<'a>,
    id: Id,

    // Styling
    width: Option<f32>,
    height: Option<f32>,
    padding: f32,
    bg: Color,
    border_color: Color,
    border_width: f32,
    radius: f32,

    // Header properties
    title: Option<String>,
    subtitle: Option<String>,
    header_bg: Option<Color>,
    header_padding: f32,
    
    // Collapsing state
    collapsible: bool,
    collapsed: Option<&'b mut bool>,

    // Shadows
    shadow_color: Color,
    shadow_offset: [f32; 2],
    shadow_blur: f32,
    shadow_opacity: f32,
}

impl<'u, 'a, 'b> PanelBuilder<'u, 'a, 'b> {
    pub fn new(ui: &'u mut Ui<'a>) -> Self {
        let id = ui.id();
        Self {
            ui,
            id,
            width: None,
            height: None,
            padding: 14.0,
            bg: Color::rgb(0.12, 0.12, 0.15),
            border_color: Color::rgb(0.2, 0.2, 0.24),
            border_width: 1.0,
            radius: 8.0,
            title: None,
            subtitle: None,
            header_bg: Some(Color::rgb(0.15, 0.15, 0.19)),
            header_padding: 12.0,
            collapsible: true,
            collapsed: None,
            shadow_color: Color::rgba(0.0, 0.0, 0.0, 0.3),
            shadow_offset: [0.0, 2.0],
            shadow_blur: 8.0,
            shadow_opacity: 0.25,
        }
    }

    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::Hasher;
        id.hash(&mut hasher);
        self.id = Id::from_u64(hasher.finish());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn collapsed(mut self, state: &'b mut bool) -> Self {
        self.collapsed = Some(state);
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

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = Some(w);
        self.height = Some(h);
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border_color = color;
        self.border_width = width;
        self
    }

    pub fn radius(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }

    pub fn header_bg(mut self, color: Color) -> Self {
        self.header_bg = Some(color);
        self
    }

    pub fn shadow(mut self, color: Color, x: f32, y: f32, blur: f32) -> Self {
        self.shadow_color = color;
        self.shadow_offset = [x, y];
        self.shadow_blur = blur;
        self
    }

    pub fn show<F>(mut self, f: F)
    where F: FnOnce(&mut Ui) {
        // Resolve collapsed state
        let collapsed_id = Id::from_u64(self.id.raw().wrapping_add(8000000));
        
        let mut is_collapsed = if let Some(ref state) = self.collapsed {
            **state
        } else {
            self.ui.interaction_state.get(&collapsed_id).map(|&v| v > 0.5).unwrap_or(false)
        };

        // Outer panel container
        let mut panel_container = self.ui.container()
            .id(self.id)
            .column()
            .bg(self.bg)
            .border(self.border_color, self.border_width)
            .radius_all(self.radius)
            .shadow(self.shadow_color, self.shadow_offset[0], self.shadow_offset[1], self.shadow_blur)
            .shadow_opacity(self.shadow_opacity);

        if let Some(w) = self.width {
            panel_container = panel_container.width(w);
        }
        if let Some(h) = self.height {
            panel_container = panel_container.height(h);
        }

        panel_container.show(|ui: &mut Ui| {
            // Header Section (only rendered if title, subtitle, or collapsible is configured)
            let has_header = self.title.is_some() || self.subtitle.is_some() || self.collapsible;
            
            if has_header {
                let header_id = Id::from_u64(self.id.raw().wrapping_add(9000000));
                let mut header_container = ui.container()
                    .id(header_id)
                    .full_width()
                    .row()
                    .halign(Align::SpaceBetween)
                    .valign(Align::Center)
                    .padding_all(self.header_padding);

                if let Some(hbg) = self.header_bg {
                    header_container = header_container.bg(hbg);
                }

                // If expanded, only round top corners. If collapsed, round all corners to match outer container.
                let bottom_r = if is_collapsed { self.radius } else { 0.0 };
                header_container = header_container.radius(self.radius, self.radius, bottom_r, bottom_r);

                let header_resp = header_container.show(|ui: &mut Ui| {
                    // Left: Title and subtitle column
                    ui.container()
                        .column()
                        .show(|ui: &mut Ui| {
                            if let Some(ref t) = self.title {
                                ui.text(t).size(14.0).bold().color(Color::WHITE).show();
                            }
                            if let Some(ref s) = self.subtitle {
                                if self.title.is_some() {
                                    ui.spacing(2.0);
                                }
                                ui.text(s).size(11.0).color(Color::rgb(0.55, 0.55, 0.65)).show();
                            }
                        });

                    // Right: Expand/Collapse Chevron Indicator
                    if self.collapsible {
                        let chevron = if is_collapsed {
                            crate::icons::NF_FA_CHEVRON_RIGHT
                        } else {
                            crate::icons::NF_FA_CHEVRON_DOWN
                        };
                        ui.text(chevron).size(12.0).color(Color::rgb(0.6, 0.6, 0.7)).show();
                    }
                });

                // Toggle collapse on click
                if self.collapsible && header_resp.clicked {
                    is_collapsed = !is_collapsed;
                    if let Some(ref mut state) = self.collapsed {
                        **state = is_collapsed;
                    } else {
                        ui.interaction_state.insert(collapsed_id, if is_collapsed { 1.0 } else { 0.0 });
                    }
                    ui.needs_redraw = true;
                }
            }

            // Body Section (only if not collapsed)
            if !is_collapsed {
                // If header is present, we draw a thin divider line
                if has_header {
                    ui.container()
                        .full_width()
                        .height(1.0)
                        .bg(self.border_color)
                        .show(|_| {});
                }

                ui.container()
                    .full_width()
                    .column()
                    .padding_all(self.padding)
                    .show(f);
            }
        });
    }
}
