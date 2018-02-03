// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use toml;
use maskerad_filesystem::filesystem as maskerad_filesystem;
use maskerad_filesystem::game_directories::RootDir;
use data_parser_error::{DataParserError, DataParserResult};
use std::path::Path;
use std::io::{Read, Write};

use maskerad_gameobject_model::gameobject::GameObject;
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
    pub fn load_from_toml<P: AsRef<Path>>(path: P) -> DataParserResult<Self> {
        let mut bufreader = maskerad_filesystem::open(path)?;
        let mut content = String::new();
        bufreader.read_to_string(&mut content)?;
        toml::from_str(content.as_ref()).map_err(|deserialization_error| {
            DataParserError::from(deserialization_error)
        })
    }

    pub fn save_as_toml(&self) -> DataParserResult<()> {
        let toml_string = self.as_string_toml()?;
        let path: &Path = self.id.as_ref();
        let mut bufwriter = maskerad_filesystem::create(path)?;
        bufwriter.write_all(toml_string.as_bytes())?;
        Ok(())
    }

    fn as_string_toml(&self) -> DataParserResult<String> {
        let toml_string = toml::to_string_pretty(&self)?;
        Ok(toml_string)
    }

    pub fn new<I, M>(id: I, transform_desc: TransformDescription, mesh_desc: M) -> Self where
        M: Into<Option<MeshDescription>>,
        I: ToString
    {
        GameObjectDescription {
            id: id.to_string(),
            transform: transform_desc,
            mesh: mesh_desc.into(),
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn transform(&self) -> &TransformDescription {
        &self.transform
    }

    pub fn mesh(&self) -> &Option<MeshDescription> {
        &self.mesh
    }
}



#[cfg(test)]
mod gameobject_description_test {
    use super::*;
    use maskerad_filesystem::game_directories::GameDirectories;

    #[test]
    fn deserialize() {
        // gameobject2.toml -> has mesh
        let game_dirs = GameDirectories::new("gameobject_file_test", "malkaviel").unwrap();
        let go2_path = game_dirs.construct_path_from_root(RootDir::WorkingDirectory, "data_deserialization_test/gameobject2.toml").unwrap();
        let go2_desc = GameObjectDescription::load_from_toml(go2_path).unwrap();

        assert!(go2_desc.mesh.is_some());
        assert_eq!(go2_desc.id.as_str(), "gameobject2");
        assert_eq!(go2_desc.transform.scale(), vec![1.0, 1.0, 1.0].as_slice());
        assert_eq!(go2_desc.transform.position(), vec![0.0, 0.0, 0.0].as_slice());
        assert_eq!(go2_desc.transform.rotation(), vec![0.0, 0.0, 0.0].as_slice());

        //gameobject1.toml -> no mesh
        let go1_path = game_dirs.construct_path_from_root(RootDir::WorkingDirectory, "data_deserialization_test/gameobject1.toml").unwrap();
        let go1_desc = GameObjectDescription::load_from_toml(go1_path).unwrap();

        assert!(go1_desc.mesh.is_none());
        assert_eq!(go1_desc.id.as_str(), "gameobject1");
        assert_eq!(go1_desc.transform.scale(), vec![1.0, 1.0, 1.0].as_slice());
        assert_eq!(go1_desc.transform.position(), vec![0.0, 0.0, 0.0].as_slice());
        assert_eq!(go1_desc.transform.rotation(), vec![0.0, 0.0, 0.0].as_slice());

    }


    #[test]
    fn serialize() {
        let game_dirs = GameDirectories::new("gameobject_file_test", "malkaviel").unwrap();
        let go4_path = game_dirs.construct_path_from_root(RootDir::WorkingDirectory, "data_serialization_test/gameobject4.toml").expect("Could not construct go4 path");


        let pos = vec![1.0, 2.0, 3.0];
        let rot = vec![0.0, 0.0, 0.0];
        let scale = vec![2.0, 2.0, 2.0];
        let transform_desc = TransformDescription::new(pos, rot, scale);
        let mesh_desc = MeshDescription::new("path_test_mesh");

        let go4_desc = GameObjectDescription::new(go4_path.to_str().unwrap(), transform_desc, mesh_desc);
        go4_desc.save_as_toml().unwrap();
        assert!(go4_path.as_path().exists());

        let pos = vec![5.0, 7.0, 11.0];
        let rot = vec![0.8, 5.2, 1.0];
        let scale = vec![2.4, 2.2, 2.9];
        let transform_desc = TransformDescription::new(pos, rot, scale);

        let go5_path = game_dirs.construct_path_from_root(RootDir::WorkingDirectory, "data_serialization_test/gameobject5.toml").expect("Could not construct go5 path");
        let go5_desc = GameObjectDescription::new(go5_path.to_str().unwrap(), transform_desc, None);
        go5_desc.save_as_toml().unwrap();
        assert!(go5_path.as_path().exists());
    }

}
