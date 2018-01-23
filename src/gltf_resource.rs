// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use gltf::Gltf;
use std::path::{PathBuf, Path};

#[derive(Debug)]
pub struct GltfResource {
    id: PathBuf,
    gltf_data: Gltf,
}

impl GltfResource {
    pub fn new(path: &Path, data: Gltf) -> Self {
        GltfResource {
            id: PathBuf::from(path),
            gltf_data: data,
        }
    }

    pub fn id(&self) -> &Path {
        self.id.as_path()
    }

    pub fn gltf_data(&self) -> &Gltf {
        &self.gltf_data
    }
}