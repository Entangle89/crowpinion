//! levelで使用する関数

#![allow(unused_imports, dead_code, unused_mut, unused_variables, unused_macros)]

use super::object::Object;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_common_assets::ron::RonAssetPlugin;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Level(i32);

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Lv {
    level: Level,
    name: String,
    asset_path: String,
    translation: Vec3,
    rotation: Vec3,
}

#[derive(serde::Serialize, serde::Deserialize, TypeUuid)]
#[uuid = "113d4e3e-e13b-4bbc-a702-63fea88c4f33"]
pub struct Levels(Vec<Lv>);

impl Levels {
    fn new() -> Self {
        let levels = Levels(Vec::new());
        levels
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test() {
        let lv = Lv {
            level: Level(0),
            name: "test_level".to_string(),
            asset_path: "test".to_string(),
            translation: Vec3::ZERO,
            rotation: Vec3::ZERO,
        };

        let mut lvs = Levels::new();
        lvs.0.push(lv);

        // Serialize the struct to a RON string
        let ron_string = ron::ser::to_string(&lvs).unwrap();

        // Write the RON string to a file
        std::fs::write("lvs.ron", ron_string).expect("Unable to write file");

        // Read the RON string from the file
        let ron_string = std::fs::read_to_string("lvs.ron").expect("Unable to read file");

        // Deserialize the RON string back into an instance of the struct
        let deserialized_lvs: Levels = ron::de::from_str(&ron_string).unwrap();

        println!("{:?}", deserialized_lvs);
    }
}
