use crate::common::prelude::*;
use crate::game::prelude::*;
use band_selection::BandSelectionSpawnEvent;
use bevy::prelude::*;

const PREVIEW_POSITION: Vec2 = Vec2::new(183., 102.);

pub struct ConcertHallPlugin;

impl Plugin for ConcertHallPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(band_selection::BandSelectionPlugin)
            .add_system_set(
                SystemSet::on_enter(AppState::TownConcertHall).with_system(concert_hall_init),
            )
            .add_system_set(
                SystemSet::on_update(AppState::TownConcertHall).with_system(concert_hall_leave),
            )
            .add_system(concert_hall_boat_preview);
    }
}

#[derive(Component)]
struct BoatPreviewParent;

#[derive(Component)]
struct BoatPreview;

fn concert_hall_init(
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut ev_band_selection_spawn: EventWriter<BandSelectionSpawnEvent>,
    mut ev_ocean_spawn: EventWriter<OceanSpawnEvent>,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    ev_band_selection_spawn.send_default();
    commands
        .spawn_bundle(VisibilityBundle::default())
        .insert_bundle(TransformBundle::default())
        .insert(Transform2::from_translation(PREVIEW_POSITION).with_scale(Vec2::ONE * 0.5))
        .insert(Label("Preview".to_owned()))
        .insert(BoatPreviewParent)
        .with_children(|parent| {
            let boat_entity = parent.spawn().insert(BoatPreview).id();
            ev_boat_spawn.send(BoatSpawnEvent {
                entity: Some(boat_entity),
                position: Vec2::ZERO,
                attack: Attack::Shockwave,
                healthbar: false,
            });
            let ocean_entity = parent.spawn().id();
            ev_ocean_spawn.send(OceanSpawnEvent {
                entity: Some(ocean_entity),
            });
        });
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.sprite_town_bg_hole.clone(),
            ..Default::default()
        })
        .insert(
            Transform2::new()
                .with_depth((DepthLayer::Front, 0.))
                .with_scale(Vec2::ONE * 0.5),
        );
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
                        top: Val::Px(300.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    "Press space to exit".to_owned(),
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

fn concert_hall_leave(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::Space) {
        app_state.set(AppState::TownOutside).unwrap();
        keys.reset(KeyCode::Space);
    }
}

#[derive(Default)]
pub struct ConcertHallBoatPreviewState {
    spawn_time: f32,
}

fn concert_hall_boat_preview(
    mut query: Query<
        (
            Entity,
            &Parent,
            &mut ShotgunCannons,
            &mut Shockwave,
            &mut DashAttack,
        ),
        With<BoatPreview>,
    >,
    mut state: Local<ConcertHallBoatPreviewState>,
    mut transform_query: Query<&mut Transform2>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    state.spawn_time += time.delta_seconds();
    let spawn = if state.spawn_time > 0.8 {
        state.spawn_time = 0.;
        true
    } else {
        false
    };
    for (entity, parent, mut shotgun_cannons, mut shockwave, mut dash_attack) in query.iter_mut() {
        if spawn {
            if let Ok(mut transform) = transform_query.get_mut(entity) {
                transform.translation = Vec2::ZERO;
            }
            match game_state.band_attack_type() {
                Attack::ShotgunCannons => {
                    shotgun_cannons.shoot = true;
                }
                Attack::Shockwave => {
                    shockwave.shoot = true;
                }
                Attack::DashAttack => {
                    dash_attack.shoot = true;
                }
            }
        }
        let translation = if spawn {
            Vec2::ZERO
        } else if let Ok(transform) = transform_query.get(entity) {
            transform.translation
        } else {
            Vec2::ZERO
        };
        if let Ok(mut parent_transform) = transform_query.get_mut(parent.get()) {
            parent_transform.translation = PREVIEW_POSITION + -translation * 0.5;
        }
    }
}

pub mod band_selection;
