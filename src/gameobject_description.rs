// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use toml;
use maskerad_filesystem::filesystem::FileSystem;
use maskerad_filesystem::game_directories::RootDir;
use data_parser_error::{DataParserError, DataParserResult};
use std::path::Path;

use maskerad_gameobject_model::gameobject::GameObject;
use maskerad_gameobject_model::properties_map::PropertiesMap;
use mesh_description::MeshDescription;
use transform_description::TransformDescription;

/*
    Gameobject file structure:
    id = "game object name"

    [transform]
    position = [x, y, z]
    rot = [x, y, z]
    scale = [x, y, z]

    [mesh]
    path = "path to mesh"
    ...
*/

#[derive(Deserialize, Serialize, Debug)]
pub struct GameObjectDescription {
    id: String,
    transform: TransformDescription,
    mesh: Option<MeshDescription>,
}



impl GameObjectDescription {
    pub fn load_from_toml(path: &str) -> DataParserResult<Self> {
        toml::from_str(path).map_err(|deserialization_error| {
            DataParserError::from(deserialization_error)
        })
    }

    pub fn save_as_toml(&self, file_system: &FileSystem) -> DataParserResult<()> {
        let toml_string = self.as_string_toml()?;
        let mut bufwriter = file_system.create(self.id.as_ref())?;
        file_system.write_all(&mut bufwriter, toml_string.as_bytes())?;
        Ok(())
    }

    fn as_string_toml(&self) -> DataParserResult<String> {
        let toml_string = toml::to_string(&self)?;
        Ok(toml_string)
    }

    pub fn generate_gameobject(&self, properties_map: &mut PropertiesMap, file_system: &FileSystem) -> DataParserResult<GameObject> {
        //Read all the properties of the gameobject and add them to the PropertiesMap
        //transform
        properties_map.add_transform(self.id.as_ref(), self.transform.generate_transform());
        //mesh if any
        if let Some(ref mesh) = self.mesh {
            properties_map.add_mesh(self.id.as_ref(), mesh.generate_mesh(file_system)?)
        }

        //create the game object
        Ok(GameObject::new(self.id.as_ref()))
    }
}



#[cfg(test)]
mod gameobject_description_test {
    use super::*;
    use maskerad_filesystem::game_infos::GameInfos;

    #[test]
    fn deserialize() {
        // gameobject2.toml -> has mesh
        let file_system = FileSystem::new(GameInfos::new("gameobject_file_test", "malkaviel")).unwrap();
        let go2_path = file_system.construct_path_from_root(&RootDir::WorkingDirectory, "data_deserialization_test/gameobject2.toml").unwrap();
        let mut go2_reader = file_system.open(go2_path.as_path()).unwrap();
        let mut content = String::new();
        file_system.read_to_string(&mut go2_reader, &mut content).unwrap();
        let go2_desc = GameObjectDescription::load_from_toml(content.as_str()).unwrap();

        assert!(go2_desc.mesh.is_some());
        assert_eq!(go2_desc.id.as_str(), "gameobject2");
        assert_eq!(go2_desc.transform.scale(), vec![1.0, 1.0, 1.0].as_slice());
        assert_eq!(go2_desc.transform.position(), vec![0.0, 0.0, 0.0].as_slice());
        assert_eq!(go2_desc.transform.rotation(), vec![0.0, 0.0, 0.0].as_slice());

        //gameobject1.toml -> no mesh
        let go1_path = file_system.construct_path_from_root(&RootDir::WorkingDirectory, "data_deserialization_test/gameobject1.toml").unwrap();
        let mut go1_reader = file_system.open(go1_path.as_path()).unwrap();
        content.clear();
        file_system.read_to_string(&mut go1_reader, &mut content).unwrap();
        let go1_desc = GameObjectDescription::load_from_toml(content.as_str()).unwrap();

        assert!(go1_desc.mesh.is_none());
        assert_eq!(go1_desc.id.as_str(), "gameobject1");
        assert_eq!(go1_desc.transform.scale(), vec![1.0, 1.0, 1.0].as_slice());
        assert_eq!(go1_desc.transform.position(), vec![0.0, 0.0, 0.0].as_slice());
        assert_eq!(go1_desc.transform.rotation(), vec![0.0, 0.0, 0.0].as_slice());

    }

    /*
    #[test]
    fn serialize() {
        let file_system = FileSystem::new(GameInfos::new("gameobject_file_test", "malkaviel")).unwrap();
    }
    */
}
