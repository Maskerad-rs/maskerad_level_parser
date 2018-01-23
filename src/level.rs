// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use level_description::LevelDescription;
use std::path::{Path, PathBuf};
use gameobject::GameObject;
use gameobject_description::{GameObjectDescription, ComponentDescription};
use level_parser_error::{LevelParserError, LevelParserResult};
use maskerad_filesystem::filesystem::FileSystem;

#[derive(Debug)]
pub struct Level {
    title: PathBuf,
    gameobjects: Vec<GameObject>,
}

impl Level {
    pub fn new(title: &Path, gameobjects: Vec<GameObject>) -> Self {
        Level {
            title: title.to_path_buf(),
            gameobjects,
        }
    }

    pub fn from_level_description(description: LevelDescription, file_system: &FileSystem) -> LevelParserResult<Self> {
        let level_title = PathBuf::from(description.level_title());
        let mut gameobjects = Vec::new();

        let mut content = String::new();

        for paths in description.gameobjects().into_iter() {
            //create the game objects descriptions
            content.clear();
            let mut reader = file_system.open(Path::new(paths.as_str()))?;
            file_system.read_to_string(&mut reader, &mut content);
            let go_desc = GameObjectDescription::load_from_toml(content.as_str())?;
            //create the gameobjects
            let go = GameObject::from_gameobject_description(go_desc, &file_system)?;
            //Add them to the vec
            gameobjects.push(go);
        }

        //create the level
        Ok(Level {
            title: level_title,
            gameobjects
        })
    }

    pub fn level_title(&self) -> &Path {
        self.title.as_path()
    }

    pub fn gameobjects(&self) -> &Vec<GameObject> {
        &self.gameobjects
    }

    pub fn save_as_toml(&self, file_system: &FileSystem) -> LevelParserResult<()> {

        let mut paths = Vec::with_capacity(self.gameobjects().len());
        for gameobject in self.gameobjects().iter() {
            gameobject.save_as_toml(file_system)?;
            paths.push(String::from(gameobject.id().to_str().unwrap()))
        }

        let level_desc = LevelDescription::new(self.level_title().to_str().unwrap(), paths);
        let toml_string = level_desc.as_string_toml()?;
        let mut bufwriter = file_system.create(self.level_title())?;
        file_system.write_all(&mut bufwriter, toml_string.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod level_test {
    use super::*;
    use maskerad_filesystem::game_infos::GameInfos;
    use maskerad_filesystem::game_directories::RootDir;

    #[test]
    fn deserialize_level() {
        let file_system = FileSystem::new(GameInfos::new("gameobject_file_test", "malkaviel")).unwrap();
        let path = file_system.construct_path_from_root(&RootDir::WorkingDirectory, "data_deserialization_test/level1.toml").unwrap();
        let mut level_reader = file_system.open(path.as_path()).unwrap();
        let mut content = String::new();

        file_system.read_to_string(&mut level_reader, &mut content).unwrap();

        let level_desc = LevelDescription::load_from_toml(content.as_str()).unwrap();
        assert_eq!(level_desc.level_title(), "level1");
        assert_eq!(level_desc.gameobjects().iter().count(), 2);

        let level = Level::from_level_description(level_desc, &file_system).unwrap();
        assert_eq!(level.level_title(), Path::new("level1"));
        assert_eq!(level.gameobjects().iter().count(), 2);
    }

    #[test]
    fn serialize_level() {
        let file_system = FileSystem::new(GameInfos::new("gameobject_file_test", "malkaviel")).unwrap();
        let path = file_system.construct_path_from_root(&RootDir::WorkingDirectory, "data_serialization_test/level3.toml").unwrap();

        let level = Level::new(path.as_path(), Vec::new());
        assert_eq!(level.level_title(), path.as_path());
        assert_eq!(level.gameobjects().iter().count(), 0);
        level.save_as_toml(&file_system).unwrap();
    }
}