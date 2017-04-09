extern crate feroxide;
use feroxide::*;
use feroxide::data_atoms::*;


extern crate gtk;
use gtk::prelude::*;
use gtk::*;

use std::rc::Rc;
use std::cell::RefCell;


macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}


fn main() {
    #![allow(unused)]

    println!("Hello World!");

    // Testing feroxide
    println!("Water has {} g/mol",
             Molecule::from_string("H2O".to_owned()).unwrap().mass());


    // Get reaction
    let reaction_water =
        ElemReaction::<Molecule>::molecule_from_string("2H2 + O2 → 2H2O".to_owned()).unwrap();


    // Create container
    let mut fo_container = Rc::new(RefCell::new(feroxide::Container::<Molecule> {
                                                    contents: vec![],
                                                    available_energy: 0.0,
                                                }));



    // Initialise GTK
    if gtk::init().is_err() {
        panic!("Failed to initialise GTK.");
    }


    // Create window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("feroxide");
    window.set_position(WindowPosition::Center);
    window.set_size_request(400, 400);

    window.connect_delete_event(|_, _| {
                                    gtk::main_quit();
                                    Inhibit(false)
                                });


    //////// Menu bar
    let menu_bar = MenuBar::new();


    // Add
    let menu_add = Menu::new();
    let item_add = MenuItem::new_with_label("Add");
    item_add.set_submenu(Some(&menu_add));

    menu_bar.append(&item_add);

    // Add > Atoms
    let menu_add_atoms = Menu::new();
    let item_add_atoms = MenuItem::new_with_label("Atoms");
    item_add_atoms.set_submenu(Some(&menu_add_atoms));
    menu_add.append(&item_add_atoms);

    // Add > Atoms > Hydrogen
    let item_add_atoms_hydrogen = MenuItem::new_with_label("H: hydrogen");
    menu_add_atoms.append(&item_add_atoms_hydrogen);

    // Add > Atoms > Oxygen
    let item_add_atoms_oxygen = MenuItem::new_with_label("O: oxygen");
    menu_add_atoms.append(&item_add_atoms_oxygen);


    // Add > Energy
    let item_add_energy = MenuItem::new_with_label("Energy: 100J");
    menu_add.append(&item_add_energy);


    // React
    let menu_react = Menu::new();
    let item_react = MenuItem::new_with_label("React");
    item_react.set_submenu(Some(&menu_react));
    menu_bar.append(&item_react);

    // React > Water
    let item_react_water = MenuItem::new_with_label(reaction_water.symbol().as_str());
    menu_react.append(&item_react_water);



    window.add(&menu_bar);


    item_add_atoms_hydrogen.connect_activate(clone!(fo_container => move |_| {
        let mut fo_container = fo_container.borrow_mut();

        fo_container.add_elements(&vec! {
            ContainerCompound {
                element: molecule_from_atom!(HYDROGEN),
                moles: 10.0
            }
        });

        println!("{}", fo_container.stringify());
    }));


    item_add_atoms_oxygen.connect_activate(clone!(fo_container => move |_| {
        let mut fo_container = fo_container.borrow_mut();

        fo_container.add_elements(&vec! {
            ContainerCompound {
                element: molecule_from_atom!(OXYGEN),
                moles: 10.0
            }
        });

        println!("{}", fo_container.stringify());
    }));


    item_add_energy.connect_activate(clone!(fo_container => move |_| {
        let mut fo_container = fo_container.borrow_mut();

        fo_container.available_energy += 100.0;

        println!("{}", fo_container.stringify());
    }));

    item_react_water.connect_activate(clone!(fo_container => move |_| {
        let mut fo_container = fo_container.borrow_mut();

        fo_container.react(& reaction_water);

        println!("{}", fo_container.stringify());
    }));

    //////// --------


    window.show_all();
    gtk::main();
}
