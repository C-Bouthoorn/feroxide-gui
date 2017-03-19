extern crate feroxide_gui;

use feroxide_gui::*;


fn main() {
    println!("Hello World!");

    // Testing feroxide
    println!("Water has {} g/mol", Molecule::from_string("H2O".to_owned()).unwrap().mass());
}
