use taffy::{Dimension, Display, LengthPercentageAuto, Position, Style};

pub fn stack_container() -> Style {
    Style {
        display: Display::Block,
        position: Position::Relative,
        size: taffy::Size {
            width: Dimension::percent(1.0),
            height: Dimension::percent(1.0),
        },
        ..Default::default()
    }
}

pub fn stack_fill() -> Style {
    let zero = LengthPercentageAuto::length(0.0);
    Style {
        position: Position::Absolute,
        inset: taffy::Rect {
            left: zero,
            right: zero,
            top: zero,
            bottom: zero,
        },
        ..Default::default()
    }
}
