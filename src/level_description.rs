// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use toml;
use std::collections::HashMap;
use maskerad_filesystem::filesystem as maskerad_filesystem;
use maskerad_filesystem::game_directories::RootDir;
use data_parser_error::{DataParserError, DataParserResult};
use std::path::Path;
use gltf::Gltf;
use std::io::{Write, Read};
use gameobject_description::{GameObjectDescription};

/*
    Level file structure:
    title = "level name"

    gameobjects = [
        "path to gameobject1"
        "path to gameobject2"
        ...
    ]

*/


#[derive(Deserialize, Serialize, Debug)]
pub struct LevelDescription {
    title: String,
    gameobjects: Vec<String>,
}

impl LevelDescription {
    pub fn load_from_toml<P: AsRef<Path>>(path: P) -> DataParserResult<Self> {
        let mut bufreader = maskerad_filesystem::open(path)?;
        let mut content = String::new();
        bufreader.read_to_string(&mut content)?;
        toml::from_str(content.as_ref()).map_err(|deserialization_error| {
            DataParserError::from(deserialization_error)
        })
    }

    fn as_string_toml(&self) -> DataParserResult<String> {
        let toml_string = toml::to_string_pretty(&self)?;
        Ok(toml_string)
    }

    pub fn save_as_toml(&self) -> DataParserResult<()> {
        let toml_string = self.as_string_toml()?;
        let path: &Path = self.title.as_ref();
        let mut bufwriter = maskerad_filesystem::create(path)?;
        bufwriter.write_all(toml_string.as_bytes())?;
        Ok(())
    }

    pub fn generate_gameobject_descriptions(&self) -> DataParserResult<Vec<GameObjectDescription>> {
        let mut vec_go_desc = Vec::new();

        for path in self.gameobjects.iter() {
            vec_go_desc.push(GameObjectDescription::load_from_toml(path)?);
        }

        Ok(vec_go_desc)
    }

    pub fn new<I: ToString>(title: I) -> Self {
        LevelDescription {
            title: title.to_string(),
            gameobjects: Vec::new(),
        }
    }

    pub fn add_gameobject<I: ToString>(&mut self, path: I) {
        self.gameobjects.push(path.to_string());
    }

    pub fn gameobject_description_paths(&self) -> &[String] {
        &self.gameobjects
    }
}


#[cfg(test)]
mod level_file_test {
    use super::*;
    use std::path::PathBuf;
    use maskerad_filesystem::game_directories::GameDirectories;

    #[test]
    fn test_deserialization() {
        let game_dirs = GameDirectories::new("gameobject_file_test", "malkaviel").unwrap();
        let path = game_dirs.construct_path_from_root(RootDir::WorkingDirectory, "data_deserialization_test/level1.toml").unwrap();
        let level_desc = LevelDescription::load_from_toml(path).unwrap();
        assert_eq!(level_desc.title, "level1");
        assert_eq!(level_desc.gameobjects.iter().count(), 2);
    }


    #[test]
    fn test_serialization() {
        let game_dirs = GameDirectories::new("gameobject_file_test", "malkaviel").unwrap();
        let level_path = game_dirs.construct_path_from_root(RootDir::WorkingDirectory, "data_serialization_test/level2.toml").unwrap();

        let mut level_desc = LevelDescription::new(level_path.to_str().unwrap());
        assert_eq!(level_desc.title.as_str(), level_path.to_str().unwrap());
        assert_eq!(level_desc.gameobjects.iter().count(), 0);

        let go4_path = game_dirs.construct_path_from_root(RootDir::WorkingDirectory, "data_serialization_test/gameobject4.toml").unwrap();
        let go5_path = game_dirs.construct_path_from_root(RootDir::WorkingDirectory, "data_serialization_test/gameobject5.toml").unwrap();

        let go4_desc = GameObjectDescription::load_from_toml(go4_path).unwrap();
        let go5_desc = GameObjectDescription::load_from_toml(go5_path).unwrap();

        level_desc.add_gameobject(go4_desc.id());
        level_desc.add_gameobject(go5_desc.id());

        level_desc.save_as_toml().unwrap();
        assert!(level_path.as_path().exists());
    }

}
