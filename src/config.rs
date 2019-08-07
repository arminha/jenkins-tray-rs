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
use serde::{Deserialize, Serialize};
use toml;

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub jenkins: JenkinsConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JenkinsConfig {
    pub url: String,
    pub user: Option<String>,
    pub access_token: Option<String>,
    pub name: Option<String>,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Config, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        toml::from_str(&content).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    pub fn write_to_file(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let content = toml::to_string(self)?;
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn deserialize() {
        let toml_str = r#"
            [jenkins]
            url = "https://example.com/"
            name = "Example"
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!("https://example.com/", config.jenkins.url);
    }

    #[test]
    fn serialize() {
        let config = Config {
            jenkins: JenkinsConfig {
                url: "https://example.com/".to_string(),
                name: None,
                user: Some("username".to_string()),
                access_token: Some("token".to_string()),
            },
        };
        let toml_string = toml::to_string(&config).unwrap();
        let expected = r#"[jenkins]
url = "https://example.com/"
user = "username"
access_token = "token"
"#;
        assert_eq!(expected, toml_string);
    }

}
