use piston_window::*;
use piston_window::types::*;
use piston_window::character::CharacterCache;


/// Prints text to the screen
pub struct Printer {
    pub font_size: FontSize,
    pub glyphs: Glyphs,
    pub ctx: Context,
    pub line_nr: u32,
    // TODO: Add left padding
}

impl Printer {
    /// Print without ending with a new line
    /// NOTE: Always goes back to x:10.0, so will overwrite previous text!
    pub fn print(&mut self, string: &str, color: Color, g2d: &mut G2d) {
        text::Text::new_color(color, self.font_size)
            .draw(
                string,

                &mut self.glyphs,
                &self.ctx.draw_state,
                self.ctx.transform.trans(10.0, Scalar::from(self.line_nr * self.font_size * 2)),
                g2d,
            );
    }


    /// Print and end with a new line
    pub fn print_ln(&mut self, string: &str, color: Color, g2d: &mut G2d) {
        self.print(string, color, g2d);

        self.line_nr += 1;
    }


    /// Parse and print a molecule string in basic (non-nesting) LaTeX format, as provided by feroxide
    /// Optional x_padding when using prefix (See: [Printer.print_molecule_string_with_prefix])
    /// NOTE: Ends with a newline
    pub fn print_molecule_string(&mut self, molecule_string: &str, color: Color, g2d: &mut G2d, x_padding: Option<Scalar>) {
        let color_text_full = text::Text::new_color(color, self.font_size);
        let color_text_half = text::Text::new_color(color, self.font_size / 2);

        let mut is_subscript = false;
        let mut is_superscript = false;
        let mut x = 10.0;

        if let Some(x_padding) = x_padding {
            x += x_padding;
        }

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


            let color_text =
                if is_subscript || is_superscript {
                    color_text_half
                } else {
                    color_text_full
                };

            let mut y = Scalar::from(self.line_nr * self.font_size * 2);

            if is_superscript {
                y -= Scalar::from(color_text.font_size);
            }

            color_text.draw(
                &c.to_string(),

                &mut self.glyphs,
                &self.ctx.draw_state,
                self.ctx.transform.trans(x, y),
                g2d,
            );

            x += self.glyphs.width(color_text.font_size, &c.to_string());
        }
    }


    /// First prints a prefix, then prints a molecule string
    /// HACK: Should be replacable with print(); print_molecule_string();
    pub fn print_molecule_string_with_prefix(&mut self, prefix: &str, prefix_color: Color, molecule_string: &str, color: Color, g2d: &mut G2d) {
        self.print_ln(prefix, prefix_color, g2d);
        let width = self.glyphs.width(self.font_size, prefix);
        self.print_molecule_string(molecule_string, color, g2d, Some(width));
        self.line_nr += 1;
    }
}
