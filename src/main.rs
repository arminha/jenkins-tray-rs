extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate find_folder;
extern crate gtk_sys;
extern crate gtk;
extern crate libappindicator;

mod jenkins;

use reqwest::IntoUrl;

use gtk::{WidgetExt, MenuShellExt, MenuItemExt, ContainerExt};
use libappindicator::{AppIndicator, AppIndicatorStatus};

use std::env;
use std::sync::{Arc, Mutex};

fn main() {
    let jenkins_url = env::args().nth(1).expect("jenkins url");
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let blue = assets.join("blue.png");
    let yellow = assets.join("yellow.png");

    let mut indicator = AppIndicator::new("libappindicator test application", "");
    indicator.set_status(AppIndicatorStatus::APP_INDICATOR_STATUS_ACTIVE);
    indicator.set_icon_full(blue.to_str().unwrap(), "blue");
    let mut m = gtk::Menu::new();

    let mutex = Arc::new(Mutex::new(indicator));

    let mi = create_menu_item("Exit", Some("application-exit"));
    mi.connect_activate(|_| gtk::main_quit());

    let mi2 = create_menu_item("Change icon", None);
    let mutex2 = mutex.clone();
    mi2.connect_activate(move |_| {
        let mut indicator = mutex2.lock().unwrap();
        indicator.set_icon_full(yellow.to_str().unwrap(), "yellow");
    });

    let mi3 = create_menu_item("Print Jobs", None);
    mi3.connect_activate(move |_| print_jobs(&jenkins_url));

    m.append(&mi2);
    m.append(&mi3);
    m.append(&mi);

    mutex.lock().unwrap().set_menu(&mut m);
    m.show_all();
    gtk::main();
}

fn create_menu_item(label: &str, icon_name: Option<&str>) -> gtk::MenuItem {
    if let Some(icon_name) = icon_name {
        let icon = gtk::Image::new_from_icon_name(icon_name, gtk::IconSize::Menu.into());
        let lbl = gtk::Label::new(label);
        let b = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        b.add(&icon);
        b.add(&lbl);
        let mi = gtk::MenuItem::new();
        mi.add(&b);
        mi
    } else {
        gtk::MenuItem::new_with_label(label)
    }
}

fn print_jobs<T: IntoUrl>(jenkins_url: T) {
    match jenkins::retrieve_jobs(jenkins_url) {
        Err(e) => {
            println!("Error: {}", e.description());
            println!("{:?}", e);
        }
        Ok(jobs) => {
            for job in jobs {
                println!("{:?}", job);
            }
        }
    }
}
