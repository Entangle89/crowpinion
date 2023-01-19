#![allow(unused_imports, dead_code, unused_mut, unused_variables)]

use bevy::{app::*, prelude::*, render::camera::*};
use bevy_common_assets::ron::RonAssetPlugin;

pub mod action;
pub mod battle;
pub mod character;
pub mod level;
pub mod object;

pub fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let camera = arrange_camera();
    let light = arrange_light();

    commands.spawn(camera);
    commands.spawn(light);
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
}

fn arrange_camera() -> Camera3dBundle {
    let position = Vec3::new(7.0, 20.0, 7.0);
    let height = position.y;
    let camera_bundle = Camera3dBundle {
        projection: OrthographicProjection {
            scale: 10.0,
            scaling_mode: ScalingMode::FixedVertical(5.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(-height, height, -height).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    camera_bundle
}

//fn arrange_light() -> DirectionalLightBundle {
//let light_bundle = DirectionalLightBundle {
//directional_light: DirectionalLight {
//illuminance: 500.0,
//..Default::default()
//},
//transform: Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
//..Default::default()
//};

//light_bundle
//}

fn arrange_light() -> PointLightBundle {
    let light_bundle = PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..Default::default()
    };

    light_bundle
}
