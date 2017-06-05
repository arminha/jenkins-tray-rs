extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate find_folder;
extern crate gtk_sys;
extern crate gtk;
extern crate libappindicator;
extern crate open;

mod jenkins;
mod tray;

use reqwest::IntoUrl;

use tray::{Tray, TrayStatus};

use std::cell::RefCell;
use std::env;
use std::rc::Rc;

fn main() {
    let jenkins_url = env::args().nth(1).expect("jenkins url");
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let tray_cell = Rc::new(RefCell::new(Tray::new()));
    let mut tray = tray_cell.borrow_mut();

    {
        let tray_cell = tray_cell.clone();
        tray.add_menu_item("Change icon", None, move |_| {
            let mut tray = tray_cell.borrow_mut();
            tray.set_status(TrayStatus::Success);
        });
    }
    {
        let jenkins_url = jenkins_url.clone();
        tray.add_menu_item("Print Jobs", None, move |_| print_jobs(&jenkins_url));
    }
    {
        let jenkins_url = jenkins_url.clone();
        tray.add_menu_item("Open Jenkins",
                           None,
                           move |_| if open::that(&jenkins_url).is_err() {
                               println!("Failed to open Jenkins");
                           });
    }
    tray.add_menu_item("Exit", Some("application-exit"), |_| gtk::main_quit());

    tray.show_all();
    std::mem::drop(tray);

    gtk::main();
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
