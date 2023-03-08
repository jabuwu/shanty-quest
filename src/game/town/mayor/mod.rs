use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct MayorPlugin;

impl Plugin for MayorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(town_init.in_schedule(OnEnter(AppState::TownMayor)))
            .add_system(town_update.in_set(OnUpdate(AppState::TownMayor)));
    }
}

fn town_init(mut commands: Commands, asset_library: Res<AssetLibrary>, game_state: Res<GameState>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
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
                        "{}'s Mayor\n\nPress space to exit",
                        game_state.town.clone().name,
                    ),
                    TextStyle {
                        font: asset_library.font_default.clone(),
                        font_size: 42.0,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..Default::default()
            });
        });
}

fn town_update(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Space) {
        app_state.set(AppState::TownOutside);
        keys.reset(KeyCode::Space);
    }
}
