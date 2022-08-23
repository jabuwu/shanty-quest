use asset_struct::prelude::*;
use bevy::prelude::*;
use jam::{
    common::prelude::*,
    game::overworld::character_controller::{CharacterController, CharacterControllerPlugin},
};

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dialogue".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CommonPlugin)
        .add_plugin(CharacterControllerPlugin)
        .add_startup_system(init)
        .add_system(player_move)
        .run();
}

#[derive(Component)]
pub struct Player;

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
    mut dialogue: ResMut<Dialogue>,
    mut ev_dialogue_init: EventWriter<DialogueInitEvent>,
) {
    asset_library.load_assets(&asset_server);
    ev_dialogue_init.send_default();
    commands.spawn_bundle(Camera2dBundle::default());
    dialogue.add_text("hi".to_owned());
    dialogue.add_text("this is example dialogue text".to_owned());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(32., 32.).into(),
                color: Color::GREEN,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., 0.))
        .insert(Collision {
            shape: CollisionShape::Rect {
                size: Vec2::new(32., 32.),
            },
            flags: 1,
        })
        .insert(CharacterController {
            movement: Vec2::ZERO,
            speed: 300.,
        })
        .insert(Player);
}

fn player_move(
    mut query: Query<&mut CharacterController, With<Player>>,
    input: Res<Input<KeyCode>>,
    dialogue: Res<Dialogue>,
) {
    for mut character_controller in query.iter_mut() {
        character_controller.movement = Vec2::ZERO;
        if dialogue.visible() {
            continue;
        }
        if input.pressed(KeyCode::W) {
            character_controller.movement.y += 1.;
        }
        if input.pressed(KeyCode::S) {
            character_controller.movement.y -= 1.;
        }
        if input.pressed(KeyCode::A) {
            character_controller.movement.x -= 1.;
        }
        if input.pressed(KeyCode::D) {
            character_controller.movement.x += 1.;
        }
    }
}
