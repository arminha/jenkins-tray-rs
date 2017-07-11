extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate find_folder;
extern crate gtk_sys;
extern crate gtk;
extern crate libappindicator;
extern crate open;
extern crate toml;

mod config;
mod jenkins;
mod tray;
mod xdg_basedir;

use config::{Config, JenkinsConfig};
use jenkins::{JenkinsStatus, JenkinsView};
use tray::{Tray, TrayStatus};

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    let config = read_config_file().expect("could not read config file");
    let jenkins_config = &config.jenkins;
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // channel for tray updates
    let (tx, rx) = mpsc::channel::<JenkinsStatus>();

    let jenkins = Arc::new(
        JenkinsView::new(
            &jenkins_config.url,
            (&jenkins_config.user).clone(),
            (&jenkins_config.access_token).clone(),
        ).unwrap(),
    );

    let mut tray = Tray::new();
    add_open_jenkins_menu_item(&mut tray, &jenkins_config.url);
    add_update_menu_item(&mut tray, &tx, jenkins.clone());
    tray.add_menu_item("Exit", Some("application-exit"), gtk::main_quit);

    let tray_cell = Rc::new(RefCell::new(tray));
    gtk::timeout_add(500, move || {
        if let Some(status) = rx.try_iter().next() {
            update_tray(&tray_cell, status);
        }
        gtk::Continue(true)
    });

    start_periodic_update(tx, jenkins, Duration::from_secs(30));

    gtk::main();
}

fn read_config_file() -> Result<Config, Box<Error>> {
    let mut path = xdg_basedir::config_home();
    path.push("jenkins-tray");
    path.push("settings.toml");
    if path.is_file() {
        Config::from_file(&path)
    } else {
        let config = Config {
            jenkins: JenkinsConfig {
                url: "https://example.com/".to_string(),
                name: None,
                user: None,
                access_token: None,
            },
        };
        config.write_to_file(&path)?;
        println!("Please edit config file at {}", path.display());
        std::process::exit(0);
    }
}

fn add_open_jenkins_menu_item(tray: &mut Tray, jenkins_url: &str) {
    let jenkins_url = jenkins_url.to_owned();
    tray.add_menu_item("Open Jenkins", None, move || if open::that(&jenkins_url)
        .is_err()
    {
        println!("Failed to open Jenkins");
    });
}

fn add_update_menu_item(tray: &mut Tray, tx: &Sender<JenkinsStatus>, jenkins: Arc<JenkinsView>) {
    let tx = tx.clone();
    tray.add_menu_item("Update", None, move || {
        let tx = tx.clone();
        let jenkins = jenkins.clone();
        thread::spawn(move || if let Some(status) = retrieve_status(&jenkins) {
            tx.send(status).unwrap();
        });
    });
}

fn start_periodic_update(tx: Sender<JenkinsStatus>, jenkins: Arc<JenkinsView>, interval: Duration) {
    thread::spawn(move || loop {
        if let Some(status) = retrieve_status(&jenkins) {
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

fn retrieve_status(jenkins: &JenkinsView) -> Option<JenkinsStatus> {
    match jenkins.retrieve_jobs() {
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
