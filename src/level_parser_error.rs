// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use toml::de::Error as DeserializationError;
use toml::ser::Error as SerializationError;
use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub enum LevelParserError {
    DeserializationError(String, DeserializationError),
    SerializationError(String, SerializationError),
}

impl fmt::Display for LevelParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LevelParserError::SerializationError(ref description, _) => {
                write!(f, "Serialization error: {}", description)
            },
            &LevelParserError::DeserializationError(ref description, _) => {
                write!(f, "Deserialization error: {}", description)
            },
        }
    }
}

impl Error for LevelParserError {
    fn description(&self) -> &str {
        match self {
            &LevelParserError::SerializationError(_, _) => {
                "SerializationError"
            },
            &LevelParserError::DeserializationError(_, _) => {
                "DeserializationError"
            }
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &LevelParserError::DeserializationError(_, ref deserialization_error) => {
                Some(deserialization_error)
            },
            &LevelParserError::SerializationError(_, ref serialization_error) => {
                Some(serialization_error)
            }
        }
    }
}

pub type LevelParserResult<T> = Result<T, LevelParserError>;

impl From<SerializationError> for LevelParserError {
    fn from(error: SerializationError) -> Self {
        LevelParserError::SerializationError(format!("Error while serializing the level as a TOML file."), error)
    }
}

impl From<DeserializationError> for LevelParserError {
    fn from(error: DeserializationError) -> Self {
        LevelParserError::DeserializationError(format!("Error while deserializing the level as a Rust structure."), error)
    }
}