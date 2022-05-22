/*
Copyright (C) 2017  Armin Häberling

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
use reqwest::blocking::Client;
use reqwest::{IntoUrl, Url};
use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct JobList {
    jobs: Vec<Job>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Job {
    name: String,
    color: Color,
    #[serde(rename = "lastBuild")]
    last_build: Option<Build>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Build {
    number: u32,
    result: Option<BuildResult>,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq)]
pub enum JenkinsStatus {
    Success,
    Unstable(Job),
    Failure(Job),
    NotBuilt,
    Unknown,
}

pub struct JenkinsView {
    jenkins_url: Url,
    username: Option<String>,
    access_token: Option<String>,
    client: Client,
}

impl Job {
    fn build_timestamp(&self) -> Option<u64> {
        self.last_build.as_ref().map(|b| b.timestamp)
    }
}

impl JenkinsStatus {
    fn from_job(job: Job) -> JenkinsStatus {
        match job.color {
            Color::Red | Color::RedAnime => JenkinsStatus::Failure(job),
            Color::Yellow | Color::YellowAnime => JenkinsStatus::Unstable(job),
            Color::Blue | Color::BlueAnime => JenkinsStatus::Success,
            Color::NotBuilt | Color::NotBuiltAnime => JenkinsStatus::NotBuilt,
            _ => JenkinsStatus::Unknown,
        }
    }

    fn aggregate(self, other: JenkinsStatus) -> JenkinsStatus {
        fn more_recent(job1: Job, job2: Job) -> Job {
            let t1 = job1.build_timestamp().unwrap_or(0);
            let t2 = job2.build_timestamp().unwrap_or(0);
            if t1 >= t2 {
                job1
            } else {
                job2
            }
        }
        match self {
            JenkinsStatus::Unknown => other,
            JenkinsStatus::NotBuilt => match other {
                JenkinsStatus::Unknown => self,
                _ => other,
            },
            JenkinsStatus::Success => match other {
                JenkinsStatus::Unstable(_) | JenkinsStatus::Failure(_) => other,
                _ => self,
            },
            JenkinsStatus::Unstable(job1) => match other {
                JenkinsStatus::Failure(_) => other,
                JenkinsStatus::Unstable(job2) => JenkinsStatus::Unstable(more_recent(job1, job2)),
                _ => JenkinsStatus::Unstable(job1),
            },
            JenkinsStatus::Failure(job1) => match other {
                JenkinsStatus::Failure(job2) => JenkinsStatus::Failure(more_recent(job1, job2)),
                _ => JenkinsStatus::Failure(job1),
            },
        }
    }
}

impl JenkinsView {
    pub fn new<T: IntoUrl>(
        jenkins_url: T,
        username: Option<String>,
        access_token: Option<String>,
    ) -> Result<JenkinsView, Box<dyn Error>> {
        let jenkins_url = jenkins_url.into_url()?;
        let client = Client::new();
        Ok(JenkinsView {
            jenkins_url,
            username,
            access_token,
            client,
        })
    }

    pub fn retrieve_jobs(&self) -> Result<Vec<Job>, Box<dyn Error>> {
        let url = self
            .jenkins_url
            .join("api/json?tree=jobs[name,color,lastBuild[number,result,timestamp]]")?;
        let mut request = self.client.get(url);
        if self.username.is_some() && self.access_token.is_some() {
            request =
                request.basic_auth(self.username.as_ref().unwrap(), self.access_token.as_ref());
        }
        let resp = request.send()?;
        let job_list: JobList = resp.json()?;
        Ok(job_list.jobs)
    }
}

pub fn aggregate_status(jobs: Vec<Job>) -> JenkinsStatus {
    let mut state = JenkinsStatus::Unknown;
    for job in jobs {
        let next_state = JenkinsStatus::from_job(job);
        state = state.aggregate(next_state);
    }
    state
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn aggregate_status_no_job() {
        assert_eq!(JenkinsStatus::Unknown, aggregate_status(Vec::new()));
    }

    #[test]
    fn aggregate_status_one_job() {
        let job = Job {
            name: "test".to_string(),
            color: Color::NotBuilt,
            last_build: None,
        };
        let jobs = vec![job];
        assert_eq!(JenkinsStatus::NotBuilt, aggregate_status(jobs));
    }

    #[test]
    fn deserialize_empty_job_list() {
        let json = r#"{"jobs": []}"#;
        let list: JobList = serde_json::from_str(json).unwrap();
        assert_eq!(list.jobs, [])
    }

    #[test]
    fn deserialize_job_list_with_one_job() {
        let json = r#"{"jobs": [
  {
      "name": "jobname",
      "color": "blue",
      "lastBuild": {
        "number": 28,
        "result": "SUCCESS",
        "timestamp": 1547148202107
      }
  }
]}"#;
        let list: JobList = serde_json::from_str(json).unwrap();
        assert_eq!(
            list.jobs,
            [Job {
                name: "jobname".to_owned(),
                color: Color::Blue,
                last_build: Some(Build {
                    number: 28,
                    result: Some(BuildResult::Success),
                    timestamp: 1547148202107
                })
            }]
        )
    }

    #[test]
    fn deserialize_job_without_build_result() {
        let json = r#"{"jobs": [
  {
      "name": "jobname",
      "color": "grey_anime",
      "lastBuild": {
        "number": 28,
        "result": null,
        "timestamp": 1547148202107
      }
  }
]}"#;
        let list: JobList = serde_json::from_str(json).unwrap();
        assert_eq!(
            list.jobs,
            [Job {
                name: "jobname".to_owned(),
                color: Color::GreyAnime,
                last_build: Some(Build {
                    number: 28,
                    result: None,
                    timestamp: 1547148202107
                })
            }]
        )
    }

    #[test]
    fn deserialize_job_without_build() {
        let json = r#"{"jobs": [
  {
      "name": "jobname",
      "color": "grey",
      "lastBuild": null
  }
]}"#;
        let list: JobList = serde_json::from_str(json).unwrap();
        assert_eq!(
            list.jobs,
            [Job {
                name: "jobname".to_owned(),
                color: Color::Grey,
                last_build: None
            }]
        )
    }
}
