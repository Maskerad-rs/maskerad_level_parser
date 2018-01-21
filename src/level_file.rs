// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use toml;
use std::collections::HashMap;
use maskerad_filesystem::filesystem::FileSystem;
use maskerad_filesystem::game_directories::RootDir;
use maskerad_filesystem::game_infos::GameInfos;
use level_parser_error::{LevelParserError, LevelParserResult};
use std::path::Path;

/*
    Level file structure:
    title = "level name"

    gameobjects = [
        "path to gameobject1"
        "path to gameobject2"
        ...
    ]
*/

/*
    Gameobject file structure:
    title = "game object name"

    [transform]
    position = [x, y, z]
    rot = [x, y, z]
    scale = [x, y, z]

    [mesh filter]
    mesh = "path to mesh file"

    [...]
    ...
*/


#[derive(Deserialize, Serialize, Debug)]
pub struct Level {
    title: String,
    gameobjects: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GameObject {
    title: String,
}

impl Level {
    pub fn from_str(path: &str) -> LevelParserResult<Self> {
        toml::from_str(path).map_err(|deserialization_error| {
            LevelParserError::from(deserialization_error)
        })
    }
}

#[cfg(test)]
mod level_file_test {
    use super::*;

    #[test]
    fn test_deserialization() {
        let file_system = FileSystem::new(GameInfos::new("level_file_test", "malkaviel")).unwrap();
        let level_path = file_system.construct_path_from_root(&RootDir::WorkingDirectory, "data_serialization_deserialization_test/level1.toml").unwrap();
        let mut reader = file_system.open(level_path.as_path()).unwrap();
        let mut content = String::new();
        file_system.read_to_string(&mut reader, &mut content).unwrap();

        let level = Level::from_str(content.as_str()).unwrap();
        println!("{:#?}", content);
        println!("{:#?}", level);
        println!("\n\n\n{:#?}", level.gameobjects);
        panic!();
    }

    #[test]
    fn test_serialization() {

    }
}