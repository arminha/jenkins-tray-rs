extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use reqwest::IntoUrl;

use std::env;

fn main() {
    let jenkins_url = env::args().nth(1).expect("jenkins url");
    match retrieve_jobs(&jenkins_url) {
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
    color: String,
    #[serde(rename = "lastBuild")]
    last_build: Option<Build>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Build {
    number: u32,
    result: String,
    timestamp: u64,
}

fn retrieve_jobs<T: IntoUrl>(jenkins_url: T) -> Result<Vec<Job>, Box<std::error::Error>> {
    let url = jenkins_url
        .into_url()?
        .join("api/json?tree=jobs[name,color,lastBuild[number,result,timestamp]]")?;
    let mut resp = reqwest::get(url)?;
    let job_list: JobList = resp.json()?;
    Ok(job_list.jobs)
}
