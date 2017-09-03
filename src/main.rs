extern crate feroxide_gui;
use feroxide_gui::*;

extern crate feroxide;
use feroxide::*;

extern crate piston_window;
use piston_window::*;
use piston_window::types::*;
use piston_window::character::CharacterCache;

use std::process;


// You could specify another OpenGL version here,
// None will use the default one
const OPENGL: Option<OpenGL> = None;

const TITLE: &str = "Feroxide";
const DIMENSIONS: (u32, u32) = (500, 500);
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


fn print_line(string: &str, color: Color, line_nr: u32, glyphs: &mut Glyphs, ctx: Context, g2d: &mut G2d) {
    text::Text::new_color(color, FONT_SIZE)
        .draw(
            string,

            glyphs,
            &ctx.draw_state,
            ctx.transform.trans(10.0, Scalar::from(line_nr * FONT_SIZE * 2)),
            g2d,
        );
}



fn print_reaction_string(reaction_string: &str, color: Color, line_nr: u32, glyphs: &mut Glyphs, ctx: Context, g2d: &mut G2d, x_padding: Option<Scalar>) {
    let color_text_full = text::Text::new_color(color, FONT_SIZE);
    let color_text_half = text::Text::new_color(color, FONT_SIZE / 2);

    let mut is_subscript = false;
    let mut is_superscript = false;
    let mut x = 10.0;

    if let Some(x_padding) = x_padding {
        x += x_padding;
    }

    for c in reaction_string.chars() {
        if c == '_' {
            is_subscript = true;
            continue;
        }
        else if c == '^' {
            is_superscript = true;
            continue;
        }
        else if c == '{' {
            continue;
        }
        else if c == '}' {
            is_subscript = false;
            is_superscript = false;
            continue;
        }


        let color_text =
            if is_subscript || is_superscript {
                color_text_half
            } else {
                color_text_full
            };

        let mut y = Scalar::from(line_nr * FONT_SIZE * 2);

        if is_superscript {
            y -= Scalar::from(color_text.font_size);
        }

        color_text.draw(
            &c.to_string(),

            glyphs,
            &ctx.draw_state,
            ctx.transform.trans(x, y),
            g2d,
        );

        x += glyphs.width(color_text.font_size, &c.to_string());
    }
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

            #[allow(unused_assignments)]
            let mut line_nr = 1;

            // Write reactions
            // TODO: DRY
            let prefix = "> ";
            print_line(prefix, colors::RED, line_nr, &mut glyphs, ctx, g2d);
            let width = glyphs.width(FONT_SIZE, prefix);
            print_reaction_string(&water_reaction_right.stringify(), colors::BLACK, line_nr, &mut glyphs, ctx, g2d, Some(width));
            line_nr += 1;

            let prefix = "< ";
            print_line(prefix, colors::RED, line_nr, &mut glyphs, ctx, g2d);
            let width = glyphs.width(FONT_SIZE, prefix);
            print_reaction_string(&water_reaction_left.stringify(), colors::BLACK, line_nr, &mut glyphs, ctx, g2d, Some(width));
            line_nr += 1;

            // New line
            line_nr += 1;

            // Write energy
            let energy_color =
                if container.available_energy <= water_reaction_left.energy_cost() {
                    colors::RED
                } else {
                    colors::GREEN
                };

            print_line(&format!("{} J", container.available_energy), energy_color, line_nr, &mut glyphs, ctx, g2d);
            line_nr += 1;

            // Write contents
            for item in &container.contents {
                //print_line(&item.stringify(), colors::BLACK, line_nr, &mut glyphs, ctx, g2d);
                print_reaction_string(&item.stringify(), colors::BLACK, line_nr, &mut glyphs, ctx, g2d, None);
                line_nr += 1;
            }

        });
    }
}
