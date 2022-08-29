use crate::common::prelude::*;
use crate::game::overworld::player::PLAYER_ATTACK_COOLDOWN;
use crate::game::prelude::*;
use bevy::prelude::*;

use super::upgrades::UpgradesState;

const PREVIEW_POSITION: Vec2 = Vec2::new(390., 102.);

pub struct BoatPreviewPlugin;

impl Plugin for BoatPreviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BoatPreviewSpawnEvent>()
            .add_system(boat_preview_spawn.before(BoatSystems::Spawn))
            .add_system(
                boat_preview_update
                    .after(BoatSystems::Update)
                    .before(OverworldCameraSystems::Update),
            );
    }
}

#[derive(Default, Clone, Copy)]
pub struct BoatPreviewSpawnEvent;

#[derive(Component)]
pub struct BoatPreview;

fn boat_preview_spawn(
    mut ev_boat_preview_spawn: EventReader<BoatPreviewSpawnEvent>,
    mut commands: Commands,
    mut ev_boat_spawn: EventWriter<BoatSpawnEvent>,
    mut ev_ocean_spawn: EventWriter<OceanSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_boat_preview_spawn.iter() {
        commands
            .spawn_bundle(VisibilityBundle::default())
            .insert_bundle(TransformBundle::default())
            .insert(Transform2::from_translation(PREVIEW_POSITION).with_scale(Vec2::ONE * 0.75))
            .with_children(|parent| {
                let boat_entity = parent.spawn().insert(BoatPreview).id();
                ev_boat_spawn.send(BoatSpawnEvent {
                    entity: Some(boat_entity),
                    position: Vec2::ZERO,
                    attack: Attacks {
                        forward_cannons: 1,
                        shotgun_cannons: 1,
                        shockwave: 1,
                        bombs: 1,
                        kraken: 1,
                    },
                    healthbar: false,
                    player: true,
                    health: 30.,
                    health_max: 30.,
                    speed: 100.,
                    attack_cooldown: PLAYER_ATTACK_COOLDOWN,
                    knockback_resistance: 0.,
                    texture_atlas: asset_library.sprite_ship_purple_atlas.clone(),
                });
                let ocean_entity = parent.spawn().id();
                ev_ocean_spawn.send(OceanSpawnEvent {
                    entity: Some(ocean_entity),
                });
            });
    }
}

fn boat_preview_update(
    mut query: Query<&mut Boat, With<BoatPreview>>,
    upgrades_state: Res<UpgradesState>,
) {
    for mut boat in query.iter_mut() {
        if let Some(hovered) = upgrades_state.hovered {
            boat.attacks = hovered.attacks(upgrades_state.preview_level);
        } else {
            boat.attacks = Attacks {
                forward_cannons: 0,
                shotgun_cannons: 0,
                shockwave: 0,
                bombs: 0,
                kraken: 0,
            };
        }
        boat.shoot = true;
    }
}
