use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldLoadEvent>()
            .add_systems(Update, world_spawn);
    }
}

#[derive(Event, Default, Clone, Copy)]
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

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    "Hold left mouse button to move!\nPress space to dash!\nPress F to start jamming!",
                    TextStyle {
                        font: asset_library.font_bold.clone(),
                        font_size: 48.0,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::Center),
                text_anchor: Anchor::Center,
                ..Default::default()
            },
            Transform2::from_xy(700., -350.).with_depth(DEPTH_LAYER_CONTROLS),
        ));
    }
}
