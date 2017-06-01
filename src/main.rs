extern crate reqwest;

use reqwest::IntoUrl;

use std::env;
use std::io::Read;

fn main() {
    let jenkins_url = env::args().nth(1).expect("jenkins url");
    if let Err(e) = do_request(&jenkins_url) {
        println!("Error: {}", e.description());
        println!("{:?}", e);
    }
}

fn do_request<T: IntoUrl>(jenkins_url: T) -> Result<(), Box<std::error::Error>> {
    let url = jenkins_url
        .into_url()?
        .join("api/json?tree=jobs[name,color]")?;
    let mut resp = reqwest::get(url)?;
    let mut content = String::new();
    resp.read_to_string(&mut content)?;
    println!("{}", content);
    Ok(())
}
