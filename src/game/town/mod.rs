use crate::common::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct TownPlugin;

impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((outside::OutsidePlugin, concert_hall::ConcertHallPlugin))
            .add_systems(Update, town_ambience);
    }
}

#[derive(Component)]
pub struct TownAmbience;

#[derive(Default)]
struct TownAmbienceState {
    last_playing: bool,
}

fn town_ambience(
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    app_state: Res<State<AppState>>,
    mut state: Local<TownAmbienceState>,
    query: Query<Entity, With<TownAmbience>>,
) {
    let playing = app_state.get().is_town();
    if playing != state.last_playing {
        state.last_playing = playing;
        if playing {
            commands.spawn((
                AudioPlusSource::new(asset_library.sound_effects.sfx_town_ambient.clone())
                    .as_looping(),
                Persistent,
                TownAmbience,
            ));
            commands.spawn((
                AudioPlusSource::new(asset_library.sound_effects.sfx_town_music.clone())
                    .as_looping(),
                Persistent,
                TownAmbience,
            ));
        } else {
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub mod concert_hall;
pub mod outside;
