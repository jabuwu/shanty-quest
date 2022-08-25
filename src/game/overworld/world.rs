use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldLoadEvent>().add_system(world_spawn);
    }
}

#[derive(Default, Clone, Copy)]
pub struct WorldLoadEvent;

#[derive(Component)]
pub struct World;

fn world_spawn(
    mut ev_spawn: EventReader<WorldLoadEvent>,
    mut commands: Commands,
    mut ev_ldtk_spawn: EventWriter<LdtkSpawnEvent>,
    mut ev_ocean_spawn: EventWriter<OceanSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        ev_ocean_spawn.send_default();
        ev_ldtk_spawn.send(LdtkSpawnEvent {
            entity: None,
            asset: asset_library.level.clone(),
            position: Vec2::new(0., 0.),
        });

        commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    "Hold left mouse button to move\nPress F to fire your cannons\nPress D to use your magical ability",
                    TextStyle {
                        font: asset_library.font_default.clone(),
                        font_size: 48.0,
                        color: Color::BLACK,
                    },
                )
                .with_alignment(TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                }),
                ..Default::default()
            })
            .insert(Transform2::from_xy(800., -350.).with_depth(DEPTH_LAYER_CONTROLS));
    }
}
