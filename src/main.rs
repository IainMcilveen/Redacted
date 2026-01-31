use bevy::prelude::*;

mod menu;
mod paper;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    MENU,
    #[default]
    PAGETEST,
    PLAYING,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(menu::plugin)
        .add_plugins(paper::plugin)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}
