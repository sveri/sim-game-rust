mod components;
mod fps;
mod systems;

use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};

use bevy_inspector_egui::WorldInspectorPlugin;

// use smooth_bevy_cameras::{LookTransform, LookTransformPlugin, controllers::orbit::{OrbitCameraPlugin}};

// pub struct TestPlugin;

// impl Plugin for TestPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_startup_system(systems::setup)
//         .add_system(systems::pan_orbit_camera)
//         // .add_plugin(OrbitCameraPlugin::default())
//         // .add_system(move_camera_system)
//             // .add_system(greet_galaxy)
//             ;
//     }
// }

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 800.0,
            height: 800.0,
            title: "Sim Game".to_string(),
            ..Default::default()
        },
        ..default()
    }))
    .add_plugin(fps::ScreenDiagsPlugin)
        // .add_plugin(systems::PanOrbitCameraPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(systems::setup)
        .add_system(systems::pan_orbit_camera)
        // .add_system(systems::pan_orbit_camera);
        ;
    app.run();
}
