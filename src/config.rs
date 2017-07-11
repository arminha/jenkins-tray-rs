
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
    pub fn from_file(path: &Path) -> Result<Config, Box<Error>> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        toml::from_str(&content).map_err(|e| Box::new(e) as Box<Error>)
    }

    pub fn write_to_file(&self, path: &Path) -> Result<(), Box<Error>> {
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
