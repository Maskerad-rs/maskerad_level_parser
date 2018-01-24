// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use toml;
use maskerad_filesystem::filesystem::FileSystem;
use maskerad_filesystem::game_directories::RootDir;
use level_parser_error::{LevelParserError, LevelParserResult};
use std::path::Path;

/*
#[derive(Deserialize, Serialize, Debug)]
pub struct ComponentDescription {
    mesh: String,
}

impl ComponentDescription {
    pub fn new(mesh_path: &str) -> Self {
        ComponentDescription {
            mesh: String::from(mesh_path),
        }
    }

    pub fn mesh_path(&self) -> &str {
        self.mesh.as_ref()
    }
}
*/
#[derive(Deserialize, Serialize, Debug)]
pub struct GameObjectDescription {
    id: String,
    //components: ComponentDescription, //TODO: temporary.
}

impl GameObjectDescription {
    pub fn new(id: &str /*components: ComponentDescription*/) -> Self {
        GameObjectDescription {
            id: String::from(id),
            //components,
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

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    /*
    pub fn components(&self) -> &ComponentDescription {
        &self.components
    }
    */
}


/*
#[cfg(test)]
mod gameobject_description_test {
    use super::*;
    use maskerad_filesystem::game_infos::GameInfos;

    #[test]
    fn deserialize() {
        let file_system = FileSystem::new(GameInfos::new("gameobject_file_test", "malkaviel")).unwrap();
        let go1_path = file_system.construct_path_from_root(&RootDir::WorkingDirectory, "data_deserialization_test/gameobject1.toml").unwrap();
        let mut go1_reader = file_system.open(go1_path.as_path()).unwrap();
        let mut content = String::new();
        file_system.read_to_string(&mut go1_reader, &mut content).unwrap();
        let go_desc = GameObjectDescription::load_from_toml(content.as_str()).unwrap();
    }

    #[test]
    fn serialize() {
        let file_system = FileSystem::new(GameInfos::new("gameobject_file_test", "malkaviel")).unwrap();
    }
}
*/