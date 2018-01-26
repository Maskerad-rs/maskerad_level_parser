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
use data_parser_error::{DataParserError, DataParserResult};
use std::path::Path;
use gltf::Gltf;
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
    pub fn load_from_toml(path: &str) -> DataParserResult<Self> {
        toml::from_str(path).map_err(|deserialization_error| {
            DataParserError::from(deserialization_error)
        })
    }

    fn as_string_toml(&self) -> DataParserResult<String> {
        let toml_string = toml::to_string_pretty(&self)?;
        Ok(toml_string)
    }

    pub fn save_as_toml(&self, file_system: &FileSystem) -> DataParserResult<()> {
        let toml_string = self.as_string_toml()?;
        let mut bufwriter = file_system.create(self.title.as_ref())?;
        file_system.write_all(&mut bufwriter, toml_string.as_bytes())?;
        Ok(())
    }

    pub fn generate_gameobject_descriptions(&self, file_system: &FileSystem) -> DataParserResult<Vec<GameObjectDescription>> {
        let mut vec_go_desc = Vec::new();
        let mut content = String::new();

        for path in self.gameobjects.iter() {
            content.clear();
            let mut reader = file_system.open(path.as_ref())?;
            file_system.read_to_string(&mut reader, &mut content)?;
            vec_go_desc.push(GameObjectDescription::load_from_toml(content.as_str())?);
        }

        Ok(vec_go_desc)
    }

    pub fn new(title: &str) -> Self {
        LevelDescription {
            title: String::from(title),
            gameobjects: Vec::new(),
        }
    }

    pub fn add_gameobject(&mut self, path: &str) {
        self.gameobjects.push(String::from(path));
    }
}


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
        assert_eq!(level_desc.title, "level1");
        assert_eq!(level_desc.gameobjects.iter().count(), 2);
    }


    #[test]
    fn test_serialization() {
        let file_system = FileSystem::new(GameInfos::new("gameobject_file_test", "malkaviel")).unwrap();
        let level_path = file_system.construct_path_from_root(&RootDir::WorkingDirectory, "data_serialization_test/level2.toml").unwrap();

        let mut level_desc = LevelDescription::new(level_path.to_str().unwrap());
        assert_eq!(level_desc.title, "/home/malkaviel/Documents/projects/intellij/maskerad_level_parser/data_serialization_test/level2.toml");
        assert_eq!(level_desc.gameobjects.iter().count(), 0);

        let go4_path = file_system.construct_path_from_root(&RootDir::WorkingDirectory, "data_serialization_test/gameobject4.toml").unwrap();
        let go5_path = file_system.construct_path_from_root(&RootDir::WorkingDirectory, "data_serialization_test/gameobject5.toml").unwrap();
        let mut content = String::new();

        let mut reader_go4 = file_system.open(go4_path.as_path()).unwrap();
        file_system.read_to_string(&mut reader_go4, &mut content).unwrap();
        let go4_desc = GameObjectDescription::load_from_toml(content.as_str()).unwrap();

        content.clear();

        let mut reader_go5 = file_system.open(go5_path.as_path()).unwrap();
        file_system.read_to_string(&mut reader_go5, &mut content).unwrap();
        let go5_desc = GameObjectDescription::load_from_toml(content.as_str()).unwrap();

        level_desc.add_gameobject(go4_desc.id());
        level_desc.add_gameobject(go5_desc.id());

        level_desc.save_as_toml(&file_system).unwrap();
        assert!(file_system.exists(level_path.as_path()));
    }

}
