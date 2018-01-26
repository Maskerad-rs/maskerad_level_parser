// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use maskerad_gameobject_model::properties::mesh::Mesh;
use maskerad_filesystem::filesystem::FileSystem;
use data_parser_error::DataParserResult;

use gltf::Gltf;

#[derive(Debug, Deserialize, Serialize)]
pub struct MeshDescription {
    path: String,
}

impl MeshDescription {
    pub fn new(path: &str) -> Self {
        MeshDescription {
            path: String::from(path),
        }
    }

    pub fn generate_mesh(&self, file_system: &FileSystem) -> DataParserResult<Mesh> {
        let gltf_reader = file_system.open(self.path.as_ref())?;
        let gltf_data = Gltf::from_reader(gltf_reader)?.validate_completely()?;

        Ok(Mesh::new(gltf_data))
    }
}