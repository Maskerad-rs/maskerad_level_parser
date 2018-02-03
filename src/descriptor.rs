// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use transform_description::TransformDescription;
use mesh_description::MeshDescription;

pub trait Descriptor {
    fn id(&self) -> &str;
    fn transform_description(&self) -> &TransformDescription;
    fn mesh_description(&self) -> &Option<MeshDescription>;
}