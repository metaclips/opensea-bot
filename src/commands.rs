use std::{
    fmt::{write, Display},
    str::FromStr,
};

use crate::error::Error;

pub enum Command {
    Subscribe,
    Unsubscribe,
    ListCollection(String),
    UnlistCollection(String),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::UnlistCollection(ref e) => {
                write!(f, "Unlist collection command. Original message {}", e)
            }
            Self::ListCollection(ref e) => {
                write!(f, "List collection command. Original message {}", e)
            }
            Self::Unsubscribe => write!(f, "Unsubscribe command"),
            Self::Subscribe => write!(f, "Subscribe command"),
        }
    }
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let s = s.as_str();

        let command = s.split(" ").collect::<Vec<&str>>();
        if command.is_empty() {
            return Err(Error::ParseCommand(String::new()));
        }

        match command.first().cloned() {
            Some("subscribe") => Ok(Command::Subscribe),
            Some("unsubscribe") => Ok(Command::Unsubscribe),
            Some("list") => Ok(Command::ListCollection(String::from(command[1]))),
            Some("unlist") => Ok(Command::UnlistCollection(String::from(command[1]))),
            Some(_) => Err(Error::ParseCommand(format!("{}", s))),
            None => Err(Error::ParseCommand(String::new())),
        }
    }
}
