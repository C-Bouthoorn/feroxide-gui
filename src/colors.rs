// Avoid loading a complete crate
type ColorComponent = f32;
type Color = [ColorComponent; 4];


pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
pub const TRANSPARENT: Color = [0.5, 0.5, 0.5, 0.0];

pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];

pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
pub const MAGENTA: Color = [1.0, 0.0, 1.0, 1.0];
pub const CYAN: Color = [0.0, 1.0, 1.0, 1.0];
