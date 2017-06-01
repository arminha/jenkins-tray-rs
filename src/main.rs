extern crate reqwest;

use reqwest::IntoUrl;

use std::env;
use std::io::Read;

fn main() {
    let url = env::args().nth(1).expect("url");
    if let Err(e) = do_request(&url) {
        println!("Error: {}", e.description());
        println!("{:?}", e);
    }
}

fn do_request<T: IntoUrl>(url: T) -> Result<(), Box<std::error::Error>> {
    let mut resp = reqwest::get(url)?;
    let mut content = String::new();
    resp.read_to_string(&mut content)?;
    println!("{}", content);
    Ok(())
}
