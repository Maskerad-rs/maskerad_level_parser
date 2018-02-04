// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use maskerad_gameobject_model::properties::mesh::Mesh;
use data_parser_error::DataParserResult;
use std::path::Path;
use gltf::Gltf;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MeshDescription {
    path: String,
}


impl MeshDescription {
    pub fn new<I: Into<String>>(path: I) -> Self {
        MeshDescription {
            path: path.into(),
        }
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }
}
