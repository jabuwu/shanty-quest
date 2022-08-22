use crate::common::prelude::*;
use audio_plus::source::AudioPlusSource;
use bevy::prelude::*;

pub struct BandJamPlugin;

impl Plugin for BandJamPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BandJamSpawnEvent>()
            .add_system(band_jam_spawn)
            .add_system(band_jam_update);
    }
}

#[derive(Default, Clone, Copy)]
pub struct BandJamSpawnEvent {
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct BandJam {
    pub jamming: bool,
    pub intensity: f32,
    pub cannons: bool,
    last_jamming: bool,
    jam_time: f32,
}

fn band_jam_spawn(
    mut ev_spawn: EventReader<BandJamSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_spawn.iter() {
        let mut entity = if let Some(entity) = event.entity {
            commands.entity(entity)
        } else {
            commands.spawn()
        };
        entity
            .insert(BandJam {
                jamming: false,
                intensity: 0.,
                cannons: false,
                last_jamming: false,
                jam_time: 0.,
            })
            .insert(AudioPlusSource::new(
                asset_library.sound_effects.sfx_jam_test.clone(),
            ));
    }
}

fn band_jam_update(mut query: Query<(&mut BandJam, &mut AudioPlusSource)>, time: Res<Time>) {
    for (mut jam, mut audio_source) in query.iter_mut() {
        if jam.last_jamming != jam.jamming {
            jam.last_jamming = jam.jamming;
            if jam.jamming {
                audio_source.play_looped();
                jam.jam_time = 1.;
            } else {
                audio_source.stop();
                jam.intensity = 0.;
            }
        }
        if jam.jam_time > 0.4588 {
            jam.cannons = true;
            jam.intensity = 1.;
            jam.jam_time = 0.;
        }
        if jam.jamming {
            jam.jam_time += time.delta_seconds();
        }
        jam.intensity *= 0.9;
    }
}
