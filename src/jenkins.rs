use reqwest::{self, IntoUrl};

use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct JobList {
    jobs: Vec<Job>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
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

pub fn retrieve_jobs<T: IntoUrl>(jenkins_url: T) -> Result<Vec<Job>, Box<Error>> {
    let url = jenkins_url
        .into_url()?
        .join("api/json?tree=jobs[name,color,lastBuild[number,result,timestamp]]")?;
    let mut resp = reqwest::get(url)?;
    let job_list: JobList = resp.json()?;
    Ok(job_list.jobs)
}
