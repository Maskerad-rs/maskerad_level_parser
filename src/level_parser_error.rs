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
use maskerad_filesystem::filesystem_error::FileSystemError;
use gltf::Error as GltfError;


#[derive(Debug)]
pub enum LevelParserError {
    DeserializationError(String, DeserializationError),
    SerializationError(String, SerializationError),
    FileSystemError(String, FileSystemError),
    GltfError(String, GltfError),
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
            &LevelParserError::FileSystemError(ref description, _) => {
                write!(f, "File system error: {}", description)
            },
            &LevelParserError::GltfError(ref description, _) => {
                write!(f, "Gltf Error: {}", description)
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
            },
            &LevelParserError::FileSystemError(_, _) => {
                "FileSystemError"
            },
            &LevelParserError::GltfError(_, _) => {
                "GltfError"
            },
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &LevelParserError::DeserializationError(_, ref deserialization_error) => {
                Some(deserialization_error)
            },
            &LevelParserError::SerializationError(_, ref serialization_error) => {
                Some(serialization_error)
            },
            &LevelParserError::FileSystemError(_, ref filesystem_error) => {
                Some(filesystem_error)
            },
            &LevelParserError::GltfError(_, ref gltf_error) => {
                Some(gltf_error)
            },
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

impl From<FileSystemError> for LevelParserError {
    fn from(error: FileSystemError) -> Self {
        LevelParserError::FileSystemError(format!("Error while manipulating the file system."), error)
    }
}

impl From<GltfError> for LevelParserError {
    fn from(error: GltfError) -> Self {
        LevelParserError::GltfError(format!("Error while manipulating gltf data."), error)
    }
}