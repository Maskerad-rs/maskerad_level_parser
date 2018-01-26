// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use maskerad_gameobject_model::properties::transform::Transform;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransformDescription {
    position: Vec<f64>,
    rotation: Vec<f64>,
    scale: Vec<f64>,
}

impl TransformDescription {
    pub fn new(position: &[f64], rotation: &[f64], scale: &[f64]) -> Self {
        TransformDescription {
            position: Vec::from(position),
            rotation: Vec::from(rotation),
            scale: Vec::from(scale),
        }
    }

    pub fn position(&self) -> &[f64] {
        &self.position
    }

    pub fn rotation(&self) -> &[f64] {
        &self.rotation
    }

    pub fn scale(&self) -> &[f64] {
        &self.scale
    }

    pub fn generate_transform(&self) -> Transform {
        let pos = (self.position[0], self.position[1], self.position[2]);
        let rot = (self.rotation[0], self.rotation[1], self.rotation[2]);
        let scale = (self.scale[0], self.scale[1], self.scale[2]);

        Transform::new(pos, rot, scale)
    }
}