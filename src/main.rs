//use bevy::prelude::*;
use bevy::{
    pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        RenderPlugin,
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
    },
};

mod audio;
mod environment;
mod menu;
mod paint;
mod paper;
mod pen;
mod mob;
mod loading;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    LOADING,
    MENU,
    PAGETEST,
    PLAYING,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins((
        //     DefaultPlugins.set(RenderPlugin {
        //         render_creation: RenderCreation::Automatic(WgpuSettings {
        //             // WARN this is a native only feature. It will not work with webgl or webgpu
        //             features: WgpuFeatures::POLYGON_MODE_LINE,
        //             ..default()
        //         }),
        //         ..default()
        //     }),
        //     // You need to add this plugin to enable wireframe rendering
        //     WireframePlugin::default(),
        // ))
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: Color::WHITE.into(),
        })
        .add_plugins(MeshPickingPlugin)
        .init_state::<GameState>()
        .add_plugins(audio::plugin)
        .add_plugins(loading::plugin)
        .add_plugins(menu::plugin)
        .add_plugins(paper::plugin)
        .add_plugins(mob::plugin)
        .add_plugins(pen::plugin)
        .add_plugins(environment::plugin)
        .add_plugins(paint::plugin)
        // .add_systems(Update, framerate)
        .run();
}


fn framerate(time: Res<Time>){
    println!("{}", 1.0/time.delta_secs())
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}
