extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate find_folder;
extern crate gtk_sys;
extern crate gtk;
extern crate libappindicator;

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
    match retrieve_jobs(jenkins_url) {
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

#[derive(Serialize, Deserialize, Debug)]
struct JobList {
    jobs: Vec<Job>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Job {
    name: String,
    color: Color,
    #[serde(rename = "lastBuild")]
    last_build: Option<Build>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Color {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "red_anime")]
    RedAnime,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "yellow_anime")]
    YellowAnime,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "blue_anime")]
    BlueAnime,
    // for historical reasons they are called grey.
    #[serde(rename = "grey")]
    Grey,
    #[serde(rename = "grey_anime")]
    GreyAnime,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "disabled_anime")]
    DisabledAnime,
    #[serde(rename = "aborted")]
    Aborted,
    #[serde(rename = "aborted_anime")]
    AbortedAnime,
    #[serde(rename = "notbuilt")]
    NotBuilt,
    #[serde(rename = "notbuilt_anime")]
    NotBuiltAnime,
}

#[derive(Serialize, Deserialize, Debug)]
struct Build {
    number: u32,
    result: BuildResult,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
enum BuildResult {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "UNSTABLE")]
    Unstable,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "NOT_BUILT")]
    NotBuilt,
    #[serde(rename = "ABORTED")]
    Aborted,
}

fn retrieve_jobs<T: IntoUrl>(jenkins_url: T) -> Result<Vec<Job>, Box<std::error::Error>> {
    let url = jenkins_url
        .into_url()?
        .join("api/json?tree=jobs[name,color,lastBuild[number,result,timestamp]]")?;
    let mut resp = reqwest::get(url)?;
    let job_list: JobList = resp.json()?;
    Ok(job_list.jobs)
}
