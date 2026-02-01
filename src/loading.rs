use bevy::prelude::*;

use crate::GameState;

#[derive(Resource, Default, Debug)]
pub struct GameAssets {
    pub wall: Handle<Image>,
    pub glass_cracks: Vec<Handle<Image>>,
    pub mob_sprites: Vec<Handle<Image>>
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, load)
        .add_systems(Update, check_ready.run_if(in_state(GameState::LOADING)));
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        wall: asset_server.load("textures/wall.png"),
        glass_cracks: vec![
            asset_server.load("textures/glass/glass1.png"),
            asset_server.load("textures/glass/glass2.png"),
            asset_server.load("textures/glass/glass3.png"),
            asset_server.load("textures/glass/glass4.png"),
            asset_server.load("textures/glass/glass5.png"),
            asset_server.load("textures/glass/glass6.png"),
            asset_server.load("textures/glass/glass7.png"),
            asset_server.load("textures/glass/glass8.png"),
            asset_server.load("textures/glass/glass9.png"),
            asset_server.load("textures/glass/glass10.png"),
            asset_server.load("textures/glass/glass11.png"),
        ],
        mob_sprites: vec![
            asset_server.load("textures/mob/mob1.png"),
            asset_server.load("textures/mob/mob2.png"),
            asset_server.load("textures/mob/mob3.png"),
            asset_server.load("textures/mob/mob4.png"),
        ],
    });
}

fn check_ready(
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
) {
    if asset_server
        .get_load_state(assets.wall.id())
        .is_some_and(|asset| asset.is_loaded())
    {
        next_state.set(GameState::MENU);
    }
}

// fn check_textures(
//     mut next_state: ResMut<NextState<GameState>>,
//     rpg_sprite_folder: Res<GlassCracksFolder>,
//     mut events: MessageReader<AssetEvent<LoadedFolder>>,
// ) {
//     // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
//     for event in events.read() {
//         if event.is_loaded_with_dependencies(&rpg_sprite_folder.0) {
//             println!("Moving to game state...");
//             next_state.set(GameState::PLAYING);
//         }
//     }
// }
