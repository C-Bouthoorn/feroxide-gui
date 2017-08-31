extern crate feroxide_gui;
use feroxide_gui::*;

extern crate feroxide;
use feroxide::*;

extern crate piston_window;
use piston_window::*;
use piston_window::types::*;

use std::process;

// You could specify another OpenGL version here,
// None will use the default one
const OPENGL: Option<OpenGL> = None;

const TITLE: &str = "Feroxide";
const DIMENSIONS: (u32, u32) = (1000, 200);
const FONT_PATH: &str = "/usr/share/fonts/TTF/VeraMono.ttf";
const FONT_SIZE: FontSize = 20;

const CTRL_C: &str = "\u{3}";


fn set_up_window() -> PistonWindow {
    let window_settings = WindowSettings::new(TITLE, DIMENSIONS)
        .decorated(true)
        .exit_on_esc(true)
        .maybe_opengl(OPENGL)
        .resizable(true)
        .vsync(false);

    let mut window: PistonWindow = window_settings.build().unwrap();

    window.set_lazy(false);
    window.set_max_fps(60);

    window
}

fn get_glyphs(window: &PistonWindow) -> Glyphs {
    let factory = window.factory.clone();
    let texture_settings = TextureSettings::new();

    Glyphs::new(FONT_PATH, factory, texture_settings).unwrap()
}


fn main() {
    let mut window = set_up_window();
    let mut glyphs = get_glyphs(&window);

    let mut container = Container {
        contents: vec! {
            ContainerCompound {
                element: Ion::from_string("H2".to_owned()).unwrap(),
                moles: Moles::from(2000.0)
            },
            ContainerCompound {
                element: Ion::from_string("O2".to_owned()).unwrap(),
                moles: Moles::from(1000.0)
            },
            ContainerCompound {
                element: Ion::from_string("H2O".to_owned()).unwrap(),
                moles: Moles::from(1000.0)
            },
        },
        available_energy: Energy::from(10_000.0),
    };

    let water_reaction_right = ElemReaction::<Ion>::ion_from_string("2H2 + O2 > 2H2O".to_owned()).unwrap();
    let water_reaction_left  = ElemReaction::<Ion>::ion_from_string("2 H2O < 2H2 + O2".to_owned()).unwrap();


    while let Some(event) = window.next() {

        /*
        if let Some(button_args) = event.button_args() {
            if button_args.button == Button::Mouse(MouseButton::Left) && button_args.state == ButtonState::Press {
                container.react(&water_reaction);
            }
        }
        */

        if let Some(string) = event.text_args() {
            if string == ">" {
                container.react(&water_reaction_right);
            }
            else if string == "<" {
                container.react(&water_reaction_left);
            }
            else if string == CTRL_C {
                process::exit(0);
            }
        }

        window.draw_2d(&event, |ctx, g2d| {
            // Clear screen
            clear(colors::WHITE, g2d);

            let mut line_nr = 1;
            let mut print_line = |string: String, color: Color| {
                text::Text::new_color(color, FONT_SIZE)
                    .draw(
                        &string,
                        &mut glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(10.0, (line_nr * FONT_SIZE * 2) as Scalar),
                        g2d,
                    );

                line_nr += 1;
            };

            // Write reactions
            print_line(format!(">: {}", water_reaction_right), colors::BLACK);
            print_line(format!("<: {}", water_reaction_left), colors::BLACK);

            // Write contents
            let content_string = container.contents.iter().map(|x| { x.name() + "; " }).collect::<String>();
            print_line(content_string, colors::BLACK);

            // Write energy
            let energy_color =
                if container.available_energy <= water_reaction_left.energy_cost() {
                    colors::RED
                } else {
                    colors::BLACK
                };

            print_line(format!("{} J", container.available_energy), energy_color);
        });
    }
}
