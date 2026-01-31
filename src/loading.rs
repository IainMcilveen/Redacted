use bevy::{asset::LoadedFolder, image::ImageSampler, prelude::*};

use crate::GameState;

#[derive(Resource, Default)]
pub struct GlassCracksFolder(pub Handle<LoadedFolder>);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::LOADING), load)
        .add_systems(Update, check_textures.run_if(in_state(GameState::LOADING)));
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Loading...");
    let glass_cracks_folder = asset_server.load_folder("textures/glass_cracks");
    commands.insert_resource(GlassCracksFolder(glass_cracks_folder));
}

fn check_textures(
    mut next_state: ResMut<NextState<GameState>>,
    rpg_sprite_folder: Res<GlassCracksFolder>,
    mut events: MessageReader<AssetEvent<LoadedFolder>>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for event in events.read() {
        if event.is_loaded_with_dependencies(&rpg_sprite_folder.0) {
            println!("Moving to game state...");
            next_state.set(GameState::PAGETEST);
        }
    }
}
