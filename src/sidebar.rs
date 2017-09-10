use piston_window::*;
use piston_window::types::*;

use colors;

pub struct Sidebar {
    rectangle: types::Rectangle,
}


impl Sidebar {
    pub fn new(dimensions: (Scalar, Scalar), start_position: (Scalar, Scalar)) -> Sidebar {
        Sidebar {
            rectangle: [start_position.0, start_position.1, dimensions.0, dimensions.1]
        }
    }


    pub fn draw(&mut self, ctx: Context, g2d: &mut G2d) {
        let rect = rectangle::Rectangle::new(colors::DARK_GREY);

        rect.draw(
            self.rectangle,
            &Default::default(),
            ctx.transform.trans(0.0, 0.0),
            g2d
        );
    }


    pub fn event(&mut self, event: &Event) {
        if let Some(resize_args) = event.resize_args() {
            // self.rectangle[2] = Scalar::from(resize_args[0]) / 10.0;
            self.rectangle[3] = Scalar::from(resize_args[1]);
        }
    }
}
