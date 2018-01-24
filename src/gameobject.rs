// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use toml;
use gltf::Gltf;
use level_parser_error::{LevelParserError, LevelParserResult};
use gameobject_description::{GameObjectDescription, /*ComponentDescription*/};
use maskerad_filesystem::filesystem::FileSystem;
use std::path::{Path, PathBuf};

/*
#[derive(Debug)]
pub struct Components {
    mesh: GltfResource,
}


impl Components {
    pub fn new(mesh: GltfResource) -> Self {
        Components {
            mesh,
        }
    }

    pub fn mesh(&self) -> &GltfResource {
        &self.mesh
    }
}
*/

#[derive(Debug)]
pub struct GameObject {
    id: PathBuf,
    //components: Components, //TODO: temporary.
}

impl GameObject {
    fn new(id: &Path /*components: Components*/) -> Self {
        GameObject {
            id: PathBuf::from(id),
            //components,
        }
    }

    pub fn from_gameobject_description(description: GameObjectDescription, file_system: &FileSystem) -> LevelParserResult<Self> {
        /*
        let gltf_path = PathBuf::from(description.components().mesh_path());
        let gltf_reader = file_system.open(gltf_path.as_path())?;
        let gltf_data = Gltf::from_reader(gltf_reader)?.validate_completely()?;

        let gltf_resource = GltfResource::new(gltf_path.as_path(), gltf_data);

        let components = Components::new(gltf_resource);
        */

        Ok(GameObject::new(Path::new(description.id()) /*components*/))
    }

    pub fn save_as_toml(&self, file_system: &FileSystem) -> LevelParserResult<()> {
        //let component_description = ComponentDescription::new(self.components().mesh().id().to_str().unwrap());

        let go_desc = GameObjectDescription::new(self.id().to_str().unwrap() /*component_description*/);

        let toml_string = go_desc.as_string_toml()?;
        let mut bufwriter = file_system.create(self.id.as_path())?;
        file_system.write_all(&mut bufwriter, toml_string.as_bytes())?;
        Ok(())
    }

    pub fn id(&self) -> &Path {
        self.id.as_path()
    }

    /*
    pub fn components(&self) -> &Components {
        &self.components
    }
    */
}

/*
#[cfg(test)]
mod gameobject_test {
    use super::*;
    use maskerad_filesystem::game_infos::GameInfos;
    use maskerad_filesystem::game_directories::RootDir;

    #[test]
    fn deserialization_gameobject() {
        let file_system = FileSystem::new(GameInfos::new("gameobject_file_test", "malkaviel")).unwrap();
        let path = file_system.construct_path_from_root(&RootDir::WorkingDirectory, "data_deserialization_test/gameobject1.toml").unwrap();
        let mut bufreader = file_system.open(path.as_path()).unwrap();
        let mut content = String::new();
        file_system.read_to_string(&mut bufreader, &mut content).unwrap();
        let go_desc = GameObjectDescription::load_from_toml(content.as_ref()).unwrap();
        let go = GameObject::from_gameobject_description(go_desc, &file_system).unwrap();
        assert_eq!(go.id.to_str().unwrap(), "gameobject1");
    }
}
*/