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
mod clock;
mod environment;
mod feedback;
mod loading;
mod menu;
mod mob;
mod paint;
mod paper;
mod pen;
mod text_asset;

pub const LIFETIME: f32 = 200.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    LOADING,
    MENU,
    PLAYING,
    END,
}

#[derive(Resource)]
pub struct CountdownTimer(Timer);

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
        .insert_resource(CountdownTimer(Timer::from_seconds(
            LIFETIME,
            TimerMode::Once,
        )))
        .add_plugins(MeshPickingPlugin)
        .init_state::<GameState>()
        .add_systems(
            Update,
            update_countdown.run_if(in_state(GameState::PLAYING)),
        )
        .add_plugins(audio::plugin)
        .add_plugins(loading::plugin)
        .add_plugins(menu::plugin)
        .add_plugins(paper::plugin)
        .add_plugins(mob::plugin)
        .add_plugins(pen::plugin)
        .add_plugins(clock::plugin)
        .add_plugins(environment::plugin)
        .add_plugins(paint::plugin)
        .add_plugins(feedback::plugin)
        // .add_systems(Update, framerate)
        .run();
}

fn framerate(time: Res<Time>) {
    // println!("{}", 1.0 / time.delta_secs())
}

fn update_countdown(
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<CountdownTimer>,
) {
    if timer.0.tick(time.delta()).is_finished() {
        next_state.set(GameState::END);
    }
}
