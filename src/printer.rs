use piston_window::*;
use piston_window::types::*;
use piston_window::character::CharacterCache;


/// Prints text to the screen
pub struct Printer {
    pub font_size: FontSize,
    pub glyphs: Glyphs,
    pub ctx: Context,
    line_nr: u32,
    left_padding: Scalar,
}


impl Printer {
    /// Create a new Printer
    pub fn new(font_size: FontSize, glyphs: Glyphs, ctx: Context) -> Printer {
        Printer {
            font_size: font_size,
            glyphs: glyphs,
            ctx: ctx,
            line_nr: 1,
            left_padding: 0.0
        }
    }


    /// Goes to the next line
    pub fn newline(&mut self) {
        self.line_nr += 1;
        self.left_padding = 0.0;
    }


    /// Print without ending with a new line
    pub fn print(&mut self, string: &str, color: Color, g2d: &mut G2d) {
        text::Text::new_color(color, self.font_size)
            .draw(
                string,

                &mut self.glyphs,
                &self.ctx.draw_state,
                self.ctx.transform.trans(self.left_padding, Scalar::from(self.line_nr * self.font_size * 2)),
                g2d,
            );

        self.left_padding += self.glyphs.width(self.font_size, string);
    }


    /// Print and end with a new line
    pub fn print_ln(&mut self, string: &str, color: Color, g2d: &mut G2d) {
        self.print(string, color, g2d);

        self.newline();
    }


    /// Parse and print a molecule string in basic (non-nesting) LaTeX format, as provided by feroxide
    pub fn print_molecule_string(&mut self, molecule_string: &str, color: Color, g2d: &mut G2d) {
        let color_text_full = text::Text::new_color(color, self.font_size);
        let color_text_half = text::Text::new_color(color, self.font_size / 2);

        let mut is_subscript = false;
        let mut is_superscript = false;

        // Parse string
        for c in molecule_string.chars() {
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


            // Get the correct Text to work with
            let text =
                if is_subscript || is_superscript {
                    color_text_half
                } else {
                    color_text_full
                };

            // Calculate height
            let mut y = Scalar::from(self.line_nr * self.font_size * 2);

            if is_superscript {
                y -= Scalar::from(text.font_size);
            }

            // Draw text
            text.draw(
                &c.to_string(),

                &mut self.glyphs,
                &self.ctx.draw_state,
                self.ctx.transform.trans(self.left_padding, y),
                g2d,
            );

            // Add left padding
            self.left_padding += self.glyphs.width(text.font_size, &c.to_string());
        }
    }


    /// Print a molecule string with an added newline
    pub fn print_molecule_string_ln(&mut self, molecule_string: &str, color: Color, g2d: &mut G2d) {
        self.print_molecule_string(molecule_string, color, g2d);
        self.newline();
    }
}
