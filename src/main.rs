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
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    let jenkins_url = env::args().nth(1).expect("jenkins url");
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // channel for tray updates
    let (tx, rx) = mpsc::channel::<JenkinsStatus>();

    let mut tray = Tray::new();
    add_open_jenkins_menu_item(&mut tray, &jenkins_url);
    add_update_menu_item(&mut tray, &tx, &jenkins_url);
    tray.add_menu_item("Exit", Some("application-exit"), || gtk::main_quit());

    let tray_cell = Rc::new(RefCell::new(tray));
    gtk::timeout_add(500, move || {
        if let Some(status) = rx.try_iter().next() {
            update_tray(&tray_cell, status);
        }
        gtk::Continue(true)
    });

    start_periodic_update(tx, jenkins_url, Duration::from_secs(30));

    gtk::main();
}

fn add_open_jenkins_menu_item(tray: &mut Tray, jenkins_url: &str) {
    let jenkins_url = jenkins_url.to_owned();
    tray.add_menu_item("Open Jenkins", None, move || if open::that(&jenkins_url)
        .is_err()
    {
        println!("Failed to open Jenkins");
    });
}

fn add_update_menu_item(tray: &mut Tray, tx: &Sender<JenkinsStatus>, jenkins_url: &str) {
    let jenkins_url = jenkins_url.to_owned();
    let tx = tx.clone();
    tray.add_menu_item("Update", None, move || {
        let jenkins_url = jenkins_url.clone();
        let tx = tx.clone();
        thread::spawn(move || if let Some(status) = retrieve_status(
            &jenkins_url,
        )
        {
            tx.send(status).unwrap();
        });
    });
}

fn start_periodic_update(tx: Sender<JenkinsStatus>, jenkins_url: String, interval: Duration) {
    thread::spawn(move || loop {
        if let Some(status) = retrieve_status(&jenkins_url) {
            tx.send(status).unwrap();
        }
        thread::sleep(interval);
    });
}

impl From<JenkinsStatus> for TrayStatus {
    fn from(status: JenkinsStatus) -> Self {
        match status {
            JenkinsStatus::Success => TrayStatus::Success,
            JenkinsStatus::Unstable(_) => TrayStatus::Unstable,
            JenkinsStatus::Failure(_) => TrayStatus::Failure,
            JenkinsStatus::Unknown => TrayStatus::Unknown,
            JenkinsStatus::NotBuilt => TrayStatus::NotBuilt,
        }
    }
}

fn update_tray(tray_cell: &Rc<RefCell<Tray>>, status: JenkinsStatus) {
    let tray_status = status.into();
    let mut tray = tray_cell.borrow_mut();
    println!("Update status: {:?}", &tray_status);
    tray.set_status(tray_status);
}

fn retrieve_status<T: IntoUrl>(jenkins_url: T) -> Option<JenkinsStatus> {
    match jenkins::retrieve_jobs(jenkins_url) {
        Err(e) => {
            println!("Error: {}\n{:?}", e.description(), e);
            None
        }
        Ok(jobs) => {
            let status = jenkins::aggregate_status(jobs);
            Some(status)
        }
    }
}
