// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use maskerad_gameobject_model::properties::transform::Transform;

#[derive(Debug, Deserialize, Serialize, PartialOrd, PartialEq, Default, Clone)]
pub struct TransformDescription {
    position: Vec<f64>,
    rotation: Vec<f64>,
    scale: Vec<f64>,
}


impl TransformDescription {
    pub fn new<I, J, K>(position: I, rotation: J, scale: K) -> Self where
        I: Into<Vec<f64>>,
        J: Into<Vec<f64>>,
        K: Into<Vec<f64>>,
    {
        TransformDescription {
            position: position.into(),
            rotation: rotation.into(),
            scale: scale.into(),
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
}
