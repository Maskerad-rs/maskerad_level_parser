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
use level_parser_error::{LevelParserError, LevelParserResult};
use std::path::Path;
use gltf::Gltf;
use gameobject_description::{GameObjectDescription, /*ComponentDescription*/};

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
pub struct LevelDescription {
    title: String,
    gameobjects: Vec<String>,
}

impl LevelDescription {
    pub fn new(title: &str, gameobjects: Vec<String>) -> Self {
        LevelDescription {
            title: String::from(title),
            gameobjects,
        }
    }

    pub fn load_from_toml(path: &str) -> LevelParserResult<Self> {
        toml::from_str(path).map_err(|deserialization_error| {
            LevelParserError::from(deserialization_error)
        })
    }

    pub fn as_string_toml(&self) -> LevelParserResult<String> {
        let toml_string = toml::to_string(&self)?;
        Ok(toml_string)
    }

    pub fn level_title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn gameobjects(&self) -> &Vec<String> {
        &self.gameobjects
    }
}

/*
#[cfg(test)]
mod level_file_test {
    use super::*;
    use std::path::PathBuf;
    use maskerad_filesystem::game_infos::GameInfos;

    #[test]
    fn test_deserialization() {
        let file_system = FileSystem::new(GameInfos::new("gameobject_file_test", "malkaviel")).unwrap();
        let mut content = String::new();
        let path = file_system.construct_path_from_root(&RootDir::WorkingDirectory, "data_deserialization_test/level1.toml").unwrap();
        let mut bufreader = file_system.open(path.as_path()).unwrap();
        file_system.read_to_string(&mut bufreader, &mut content).unwrap();
        let level_desc = LevelDescription::load_from_toml(content.as_str()).unwrap();
        assert_eq!(level_desc.level_title(), "level1");
        assert_eq!(level_desc.gameobjects().iter().count(), 2);
    }

    #[test]
    fn test_serialization() {
        let file_system = FileSystem::new(GameInfos::new("gameobject_file_test", "malkaviel")).unwrap();
        let path = file_system.construct_path_from_root(&RootDir::WorkingDirectory, "data_serialization_test/level2.toml").unwrap();

        let level_desc = LevelDescription::new(path.to_str().unwrap(), Vec::new());
        assert_eq!(level_desc.level_title(), "/home/malkaviel/Documents/projects/intellij/maskerad_level_parser/data_serialization_test/level2.toml");
        assert_eq!(level_desc.gameobjects().iter().count(), 0);
        let level_toml_string = level_desc.as_string_toml().unwrap();
        let mut writer = file_system.create(path.as_path()).unwrap();
        file_system.write_all(&mut writer, level_toml_string.as_bytes()).unwrap();
    }
}
*/