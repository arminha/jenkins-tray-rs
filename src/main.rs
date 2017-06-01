extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use reqwest::IntoUrl;

use std::env;

fn main() {
    let jenkins_url = env::args().nth(1).expect("jenkins url");
    if let Err(e) = do_request(&jenkins_url) {
        println!("Error: {}", e.description());
        println!("{:?}", e);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct JobList {
    jobs: Vec<Job>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Job {
    name: String,
    color: String,
}

fn do_request<T: IntoUrl>(jenkins_url: T) -> Result<(), Box<std::error::Error>> {
    let url = jenkins_url
        .into_url()?
        .join("api/json?tree=jobs[name,color]")?;
    let mut resp = reqwest::get(url)?;
    let job_list: JobList = resp.json()?;
    println!("{:?}", job_list);
    Ok(())
}
