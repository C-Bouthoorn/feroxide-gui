extern crate feroxide;
use feroxide::*;
// use feroxide::data_atoms::*;


extern crate gtk;
use gtk::prelude::*;
use gtk::*;


fn main() {
    #![allow(unused)]

    println!("Hello World!");

    // Testing feroxide
    println!("Water has {} g/mol", Molecule::from_string("H2O".to_owned()).unwrap().mass());


    // Create container
    let mut fo_container = feroxide::Container::<Molecule> {
        contents: vec! {},
        available_energy: 0.0
    };


    // Create window
    if gtk::init().is_err() {
        panic!("Failed to initialise GTK.");
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("feroxide");
    window.set_position(WindowPosition::Center);
    window.set_size_request(400, 400);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });



    let menu_bar = MenuBar::new();

    let menu_add = Menu::new();
    let item_add = MenuItem::new_with_label("Add");
    let item_hydrogen = MenuItem::new_with_label("H: hydrogen");
    let item_oxygen = MenuItem::new_with_label("O: oxygen");

    menu_add.append(& item_hydrogen);
    menu_add.append(& item_oxygen);
    item_add.set_submenu(Some(& menu_add));

    menu_bar.append(& item_add);

    /*
    let item = MenuItem::new_with_label("Add one");

    struct Variable {
        pub value: u32
    }

    let mut x = Variable {
        value: 0
    };

    impl Variable {
        pub fn add(&mut self, x: u32) {
            self.value += x;
        }
    }

    item.connect_activate(move |_| {
        x.add(10);
    });
    */

    item_hydrogen.connect_activate(move |_| {
        /*
        fo_container.add_elements(&vec! {
            ContainerCompound {
                element: molecule_from_atom!(HYDROGEN),
                moles: 10.0
            }
        });
        */
    });

    window.add(& menu_bar);

    window.show_all();

    gtk::main();
}
