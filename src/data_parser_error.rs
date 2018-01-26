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
pub enum DataParserError {
    DeserializationError(String, DeserializationError),
    SerializationError(String, SerializationError),
    FileSystemError(String, FileSystemError),
    GltfError(String, GltfError),
}

impl fmt::Display for DataParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DataParserError::SerializationError(ref description, _) => {
                write!(f, "Serialization error: {}", description)
            },
            &DataParserError::DeserializationError(ref description, _) => {
                write!(f, "Deserialization error: {}", description)
            },
            &DataParserError::FileSystemError(ref description, _) => {
                write!(f, "File system error: {}", description)
            },
            &DataParserError::GltfError(ref description, _) => {
                write!(f, "Gltf Error: {}", description)
            },
        }
    }
}

impl Error for DataParserError {
    fn description(&self) -> &str {
        match self {
            &DataParserError::SerializationError(_, _) => {
                "SerializationError"
            },
            &DataParserError::DeserializationError(_, _) => {
                "DeserializationError"
            },
            &DataParserError::FileSystemError(_, _) => {
                "FileSystemError"
            },
            &DataParserError::GltfError(_, _) => {
                "GltfError"
            },
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &DataParserError::DeserializationError(_, ref deserialization_error) => {
                Some(deserialization_error)
            },
            &DataParserError::SerializationError(_, ref serialization_error) => {
                Some(serialization_error)
            },
            &DataParserError::FileSystemError(_, ref filesystem_error) => {
                Some(filesystem_error)
            },
            &DataParserError::GltfError(_, ref gltf_error) => {
                Some(gltf_error)
            },
        }
    }
}

pub type DataParserResult<T> = Result<T, DataParserError>;

impl From<SerializationError> for DataParserError {
    fn from(error: SerializationError) -> Self {
        DataParserError::SerializationError(format!("Error while serializing the level as a TOML file."), error)
    }
}

impl From<DeserializationError> for DataParserError {
    fn from(error: DeserializationError) -> Self {
        DataParserError::DeserializationError(format!("Error while deserializing the level as a Rust structure."), error)
    }
}

impl From<FileSystemError> for DataParserError {
    fn from(error: FileSystemError) -> Self {
        DataParserError::FileSystemError(format!("Error while manipulating the file system."), error)
    }
}

impl From<GltfError> for DataParserError {
    fn from(error: GltfError) -> Self {
        DataParserError::GltfError(format!("Error while manipulating gltf data."), error)
    }
}