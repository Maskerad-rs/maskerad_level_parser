// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


extern crate maskerad_filesystem;
extern crate maskerad_data_parser;

use maskerad_filesystem::filesystem::Filesystem;
use maskerad_filesystem::game_directories::RootDir;
use maskerad_data_parser::gameobject_builder::GameObjectBuilder;
use maskerad_data_parser::level_description::LevelDescription;
use maskerad_data_parser::transform_description::TransformDescription;
use maskerad_data_parser::mesh_description::MeshDescription;
use std::io::Write;
use std::io::Read;

#[test]
fn test_serialization() {
    let fs = Filesystem::new("gameobject_file_test", "malkaviel").unwrap();
    let level_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "data_serialization_test/level2.toml").unwrap();

    let mut level_desc = LevelDescription::new(level_path.to_str().unwrap());
    assert_eq!(level_desc.title(), level_path.to_str().unwrap());
    assert_eq!(level_desc.slice().iter().count(), 0);

    let go4_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "data_serialization_test/gameobject4.toml").unwrap();
    let go5_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "data_serialization_test/gameobject5.toml").unwrap();
    let mut go4_content = fs.open(go4_path).unwrap();
    let mut go5_content = fs.open(go5_path).unwrap();

    let go4_desc = GameObjectBuilder::load_from_toml(&mut go4_content).unwrap();
    let go5_desc = GameObjectBuilder::load_from_toml(&mut go5_content).unwrap();

    level_desc.add_gameobject(go4_desc);
    level_desc.add_gameobject(go5_desc);

    let mut level_writer = fs.create(level_path.as_path()).unwrap();
    level_writer.write_all(level_desc.as_string_toml().unwrap().as_ref()).unwrap();
    assert!(level_path.as_path().exists());
}

#[test]
fn deserialize_gameobject_builder() {
    // gameobject2.toml -> has mesh
    let fs = Filesystem::new("gameobject_file_test", "malkaviel").unwrap();
    let go2_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "data_deserialization_test/gameobject2.toml").unwrap();
    let mut go2_content = fs.open(go2_path.as_path()).unwrap();
    let go2_desc = GameObjectBuilder::load_from_toml(&mut go2_content).unwrap();

    assert!(go2_desc.get_mesh_resource().is_some());
    assert_eq!(go2_desc.id(), "gameobject2");
    assert_eq!(go2_desc.transform().scale(), vec![1.0, 1.0, 1.0].as_slice());
    assert_eq!(go2_desc.transform().position(), vec![0.0, 0.0, 0.0].as_slice());
    assert_eq!(go2_desc.transform().rotation(), vec![0.0, 0.0, 0.0].as_slice());

    //gameobject1.toml -> no mesh
    let go1_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "data_deserialization_test/gameobject1.toml").unwrap();
    let mut go1_content = fs.open(go1_path.as_path()).unwrap();
    let go1_desc = GameObjectBuilder::load_from_toml(&mut go1_content).unwrap();

    assert!(go1_desc.get_mesh_resource().is_none());
    assert_eq!(go1_desc.id(), "gameobject1");
    assert_eq!(go1_desc.transform().scale(), vec![1.0, 1.0, 1.0].as_slice());
    assert_eq!(go1_desc.transform().position(), vec![0.0, 0.0, 0.0].as_slice());
    assert_eq!(go1_desc.transform().rotation(), vec![0.0, 0.0, 0.0].as_slice());

}


#[test]
fn serialize_gameobjectbuilder() {
    let fs = Filesystem::new("gameobject_file_test", "malkaviel").unwrap();
    let go4_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "data_serialization_test/gameobject4.toml").expect("Could not construct go4 path");
    let mut go4_content = fs.open(go4_path.as_path()).unwrap();

    let pos = vec![1.0, 2.0, 3.0];
    let rot = vec![0.0, 0.0, 0.0];
    let scale = vec![2.0, 2.0, 2.0];
    let transform_desc = TransformDescription::new(pos, rot, scale);
    let mesh_desc = MeshDescription::new("path_test_mesh");

    let mut go4_desc = GameObjectBuilder::new(go4_path.to_str().unwrap());
    go4_desc
        .add_transform(transform_desc)
        .add_mesh(mesh_desc);

    let mut writer = fs.create(go4_path.as_path()).unwrap();
    writer.write_all(go4_desc.as_string_toml().unwrap().as_ref());


    assert!(go4_path.as_path().exists());

    let pos = vec![5.0, 7.0, 11.0];
    let rot = vec![0.8, 5.2, 1.0];
    let scale = vec![2.4, 2.2, 2.9];
    let transform_desc = TransformDescription::new(pos, rot, scale);

    let go5_path = fs.construct_path_from_root(RootDir::WorkingDirectory, "data_serialization_test/gameobject5.toml").expect("Could not construct go5 path");
    let mut go5_desc = GameObjectBuilder::new(go5_path.to_str().unwrap());
    go5_desc.add_transform(transform_desc);

    let mut writer = fs.create(go5_path.as_path()).unwrap();
    writer.write_all(go5_desc.as_string_toml().unwrap().as_ref());

    assert!(go5_path.as_path().exists());
}