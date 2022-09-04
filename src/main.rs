use std::{
    fs::{
        File,
        OpenOptions,
    },
    io::{
        self,
        Read,
    },
    env,
};

use regex::Regex;
use toml::toml;

mod config;

use config::Config;


#[derive(Debug)]
enum Token {
    Label,
    Directive,
    Comment,
    Instruction,
    Empty,
}

struct Parser {
    label: Regex,
    directive: Regex,
    comment: Regex,
    instruction: Regex,
}

impl Parser {
    fn new (config: Config) -> Parser {
        Self {
            label: config.label.build(),
            directive: config.directive.build(),
            comment: config.comment.build(),
            instruction: config.instruction.build(),
        }
    }

    fn token (&self, s: &str) -> Option<Token> {
        if s.trim().len() == 0 {
            Some(Token::Empty)
        } else if self.label.is_match(s) {
            Some(Token::Label)
        } else if self.directive.is_match(s) {
            Some(Token::Directive)
        } else if self.comment.is_match(s) {
            Some(Token::Comment)
        } else if self.instruction.is_match(s) {
            Some(Token::Instruction)
        } else {
            None
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let parser = {
        let mut buffer = String::new();
        let mut file = OpenOptions::new().read(true).open("Config.toml")?;
        file.read_to_string(&mut buffer);
        let config: Config = toml::from_str(&buffer).unwrap();
        Parser::new(config)
    };

    let mut buffer = String::new();
    let mut file = OpenOptions::new().read(true).open(&args[1])?;

    file.read_to_string(&mut buffer)?;

    for line in buffer.split('\n') {
        println!("{:?}", parser.token(line));
    }

    Ok(())
}
