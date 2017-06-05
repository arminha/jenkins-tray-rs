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

use jenkins::JenkinsStatus;
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
        let jenkins_url = jenkins_url.clone();
        tray.add_menu_item("Open Jenkins",
                           None,
                           move || if open::that(&jenkins_url).is_err() {
                               println!("Failed to open Jenkins");
                           });
    }
    {
        let jenkins_url = jenkins_url.clone();
        let tray_cell = tray_cell.clone();
        tray.add_menu_item("Update",
                           None,
                           move || if let Some(status) = retrieve_tray_status(&jenkins_url) {
                               let mut tray = tray_cell.borrow_mut();
                               tray.set_status(status);
                           });
    }
    tray.add_menu_item("Exit", Some("application-exit"), || gtk::main_quit());

    tray.show_all();
    std::mem::drop(tray);

    gtk::main();
}

fn retrieve_tray_status<T: IntoUrl>(jenkins_url: T) -> Option<TrayStatus> {
    match jenkins::retrieve_jobs(jenkins_url) {
        Err(e) => {
            println!("Error: {}", e.description());
            println!("{:?}", e);
            None
        }
        Ok(jobs) => {
            let status = jenkins::aggregate_status(jobs);
            let tray_status = match status {
                JenkinsStatus::Success => TrayStatus::Success,
                JenkinsStatus::Unstable(_) => TrayStatus::Unstable,
                JenkinsStatus::Failure(_) => TrayStatus::Failure,
                JenkinsStatus::Unknown => TrayStatus::Unknown,
                JenkinsStatus::NotBuilt => TrayStatus::NotBuilt,
            };
            Some(tray_status)
        }
    }
}
