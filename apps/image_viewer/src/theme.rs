use zenthra_core::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

pub struct ThemeColors {
    pub bg_base: Color,
    pub bg_panel: Color,
    pub bg_sidebar: Color,
    pub bg_active: Color,
    pub border: Color,
    pub accent: Color,
    pub text_primary: Color,
    pub text_muted: Color,
    pub text_dim: Color,
}

impl ThemeColors {
    pub fn get(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self {
                bg_base:      Color::rgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0), // 0 – pure black window bg
                bg_panel:     Color::rgb(2.0 / 255.0, 2.0 / 255.0, 2.0 / 255.0), // 2 – menu bar / toolbar
                bg_sidebar:   Color::rgb(1.0 / 255.0, 1.0 / 255.0, 1.0 / 255.0), // 1 – sidebar / popup bg
                bg_active:    Color::rgb(3.0 / 255.0, 3.0 / 255.0, 3.0 / 255.0), // 3 – selected item bg
                border:       Color::rgb(3.0 / 255.0, 3.0 / 255.0, 3.0 / 255.0), // 3 – hover / border
                accent:       Color::rgb(255.0 / 255.0, 214.0 / 255.0, 0.0 / 255.0), // #FFD600 vivid yellow
                text_primary: Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0), // #e0e0e0
                text_muted:   Color::rgb(102.0 / 255.0, 102.0 / 255.0, 102.0 / 255.0), // #666666
                text_dim:     Color::rgb(68.0 / 255.0, 68.0 / 255.0, 68.0 / 255.0),    // #444444
            },
            ThemeMode::Light => Self {
                bg_base: Color::rgb(248.0 / 255.0, 248.0 / 255.0, 246.0 / 255.0),
                bg_panel: Color::WHITE,
                bg_sidebar: Color::rgb(242.0 / 255.0, 242.0 / 255.0, 240.0 / 255.0),
                bg_active: Color::rgb(250.0 / 255.0, 244.0 / 255.0, 232.0 / 255.0),
                border: Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0),
                accent: Color::rgb(200.0 / 255.0, 160.0 / 255.0, 0.0 / 255.0), // vivid yellow for light mode
                text_primary: Color::rgb(26.0 / 255.0, 26.0 / 255.0, 26.0 / 255.0),
                text_muted: Color::rgb(136.0 / 255.0, 136.0 / 255.0, 136.0 / 255.0),
                text_dim: Color::rgb(170.0 / 255.0, 170.0 / 255.0, 170.0 / 255.0),
            },
        }
    }
}
