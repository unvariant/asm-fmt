use regex::Regex;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RegexConfig {
    prefix: Option<String>,
    suffix: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub label: RegexConfig,
    pub directive: RegexConfig,
    pub comment: RegexConfig,
    pub instruction: RegexConfig,
}

impl RegexConfig {
    pub fn build (&self) -> Regex {
        let mut re = String::new();

        if let Some(s) = &self.prefix {
            re += s;
        }

        re += ".*";

        if let Some(s) = &self.suffix {
            re += s;
        }

        Regex::new(&re).unwrap()
    }
}

