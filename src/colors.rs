use piston_window::types::Color;


pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
pub const DARK_GREY: Color = [0.75, 0.75, 0.75, 1.0];
pub const GREY: Color = [0.5, 0.5, 0.5, 1.0];
pub const LIGHT_GREY: Color = [0.25, 0.25, 0.25, 1.0];
pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

pub const TRANSPARENT: Color = [0.0, 0.0, 0.0, 0.0];

pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];

pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
pub const MAGENTA: Color = [1.0, 0.0, 1.0, 1.0];
pub const CYAN: Color = [0.0, 1.0, 1.0, 1.0];


/// Check if the given color values are between 0.0 and 1.0
pub fn check_color_bounds(color: Color) -> Color {
    let mut r = color[0];
    let mut g = color[1];
    let mut b = color[2];
    let mut a = color[3];

    if r < 0.0 { r = 0.0; }
    else if r > 1.0 { r = 1.0; }

    if g < 0.0 { g = 0.0; }
    else if g > 1.0 { g = 1.0; }

    if b < 0.0 { b = 0.0; }
    else if b > 1.0 { b = 1.0; }

    if a < 0.0 { a = 0.0; }
    else if a > 1.0 { a = 1.0; }

    [r, g, b, a]
}


/// Multiply all values of a color by a given factor
/// You probably want to call `check_color_bounds` after this to make sure
/// the values are valid
pub fn multiply_color(color: Color, factor: f32, include_alpha: bool) -> Color {
    let r = color[0] * factor;
    let g = color[1] * factor;
    let b = color[2] * factor;

    let a = if include_alpha {
        color[3] * factor
    } else {
        color[3]
    };

    [r, g, b, a]
}


/// Make a color darker by a given percentage (1.0 is 100%)
pub fn darker(color: Color, percentage: f32, include_alpha: bool) -> Color {
    let color = multiply_color(color, 1.0 + percentage, include_alpha);
    check_color_bounds(color)
}


/// Make a color lighter by a given percentage (1.0 is 100%)
pub fn lighter(color: Color, percentage: f32, include_alpha: bool) -> Color {
    let color = multiply_color(color, 1.0 - percentage, include_alpha);
    check_color_bounds(color)
}
