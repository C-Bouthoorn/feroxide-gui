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
    println!("Water has {} g/mol", Molecule::from_string("H2O".to_owned()).unwrap().mass());


    // Create container
    let mut fo_container = Rc::new(RefCell::new(feroxide::Container::<Molecule> {
        contents: vec! {},
        available_energy: 0.0
    }));


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



    item_hydrogen.connect_activate(clone!(fo_container => move |_| {
        let mut fo_container = fo_container.borrow_mut();

        fo_container.add_elements(&vec! {
            ContainerCompound {
                element: molecule_from_atom!(HYDROGEN),
                moles: 10.0
            }
        });

        println!("{}", fo_container.stringify());
    }));


    item_oxygen.connect_activate(clone!(fo_container => move |_| {
        let mut fo_container = fo_container.borrow_mut();

        fo_container.add_elements(&vec! {
            ContainerCompound {
                element: molecule_from_atom!(OXYGEN),
                moles: 10.0
            }
        });

        println!("{}", fo_container.stringify());
    }));


    window.add(& menu_bar);

    window.show_all();

    gtk::main();
}
