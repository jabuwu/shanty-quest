use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct TownPlugin;

impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::GameTown).with_system(town_init))
            .add_system_set(SystemSet::on_update(AppState::GameTown).with_system(town_update));
    }
}

fn town_init(mut commands: Commands, asset_library: Res<AssetLibrary>, game_state: Res<GameState>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Relative,
                    position: UiRect {
                        top: Val::Px(-50.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    format!(
                        "In town: {}\n\nPress space to exit",
                        game_state.town.clone().name,
                    ),
                    TextStyle {
                        font: asset_library.font_default.clone(),
                        font_size: 42.0,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                }),
                ..Default::default()
            });
        });
}

fn town_update(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::Space) {
        app_state.set(AppState::GameOverworld).unwrap();
        keys.reset(KeyCode::Space);
    }
}
