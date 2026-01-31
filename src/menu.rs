use bevy::{app::AppExit, prelude::*};

use super::GameState;

#[derive(Component)]
enum MenuAction {
    Play,
    Quit,
}

// Tags menu items
#[derive(Component)]
struct Menu;

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// Colours
const BG_COLOR: Color = Color::srgb(0.25, 0.5, 0.9);
const SUB_COLOR: Color = Color::srgb(0.65, 0.7, 0.8);
const TITLE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MENU), menu_setup)
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::MENU)),
        );
}

fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((DespawnOnExit(GameState::MENU), Camera2d));

    // Common style for all buttons on the screen
    let button_node = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: px(30),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: px(10),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    // load button icons
    let right_icon = asset_server.load("menu/right.png");
    let exit_icon = asset_server.load("menu/exit.png");

    commands.spawn((
        DespawnOnExit(GameState::MENU),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(BG_COLOR),
        Menu,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(SUB_COLOR),
            children![
                // Display the game name
                (
                    Text::new("Some Mask Themed Game"),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(TITLE_COLOR),
                    Node {
                        margin: UiRect::all(px(50)),
                        ..default()
                    },
                ),
                // Display Buttons
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuAction::Play,
                    children![
                        (ImageNode::new(right_icon), button_icon_node.clone()),
                        (
                            Text::new("Play"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuAction::Quit,
                    children![
                        (ImageNode::new(exit_icon), button_icon_node),
                        (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),
                    ]
                ),
            ]
        )],
    ));
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

// handle selecting one of the menu options
fn menu_action(
    interaction_query: Query<(&Interaction, &MenuAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuAction::Quit => {
                    // exiting is broken on MacOS
                    panic!();
                    //app_exit_writer.write(AppExit::Success);
                }
                MenuAction::Play => {
                    game_state.set(GameState::PLAYING);
                }
            }
        }
    }
}
