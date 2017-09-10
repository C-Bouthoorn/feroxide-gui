pub extern crate piston_window;

pub mod colors;

mod printer;
pub use printer::*;

mod sidebar;
pub use sidebar::*;


#[cfg(test)]
extern crate float_cmp;
#[cfg(test)]
use float_cmp::ApproxEqUlps;


#[cfg(test)]
/// Check if two floats are approximately equal
macro_rules! assert_float_eq {
    ($left: expr, $right: expr, $precision: expr) => {
        assert!($left.approx_eq_ulps(&$right, $precision));
    }
}


#[cfg(test)]
/// Check if two colors are equal, taking care of float comparison
macro_rules! assert_color_eq {
    ($left: expr, $right: expr) => {
        assert_float_eq!($left[0], $right[0], 3);
        assert_float_eq!($left[1], $right[1], 3);
        assert_float_eq!($left[2], $right[2], 3);
        assert_float_eq!($left[3], $right[3], 3);
    }
}


#[test]
fn color_multiply() {
    use colors::multiply_color;

    let color = [0.2, 0.4, 0.8, 1.0];

    assert_color_eq!([0.1, 0.2, 0.4, 0.5], multiply_color(color, 0.5, true));
    assert_color_eq!([0.1, 0.2, 0.4, 1.0], multiply_color(color, 0.5, false));
    assert_color_eq!([0.4, 0.8, 1.6, 2.0], multiply_color(color, 2.0, true));
    assert_color_eq!([0.4, 0.8, 1.6, 1.0], multiply_color(color, 2.0, false));
}

#[test]
fn color_bounds() {
    use colors::check_color_bounds;

    let color = [0.2, -0.7, 1.3, 0.0];

    assert_color_eq!([0.2, 0.0, 1.0, 0.0], check_color_bounds(color));
}

#[test]
fn test_darker() {
    use colors::darker;

    let color = [0.3, 0.6, 0.9, 1.0];

    assert_color_eq!([0.45, 0.9, 1.0, 1.0], darker(color, 0.5, true));
    assert_color_eq!([0.45, 0.9, 1.0, 1.0], darker(color, 0.5, false));
}

#[test]
fn test_lighter() {
    use colors::lighter;

    let color = [0.3, 0.6, 0.9, 1.0];

    assert_color_eq!([0.15, 0.3, 0.45, 0.5], lighter(color, 0.5, true));
    assert_color_eq!([0.15, 0.3, 0.45, 1.0], lighter(color, 0.5, false));
}
