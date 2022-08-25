use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
struct DeadState {
    can_respawn: bool,
}

pub struct DeadPlugin;

impl Plugin for DeadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeadState>()
            .add_system_set(SystemSet::on_enter(AppState::Dead).with_system(town_init))
            .add_system_set(SystemSet::on_update(AppState::Dead).with_system(town_update));
    }
}

fn town_init(
    mut state: ResMut<DeadState>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut game_state: ResMut<GameState>,
) {
    state.can_respawn = game_state.restore_checkpoint();
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
                    if state.can_respawn {
                        "You died\n\nPress space to respawn at last town"
                    } else {
                        "You died\n\nPress space to restart"
                    },
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

fn town_update(
    state: Res<DeadState>,
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if state.can_respawn {
            app_state.set(AppState::Overworld).unwrap();
        } else {
            app_state.set(AppState::MainMenu).unwrap();
        }
        keys.reset(KeyCode::Space);
    }
}
