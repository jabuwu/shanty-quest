use crate::common::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct TownPlugin;

impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(outside::OutsidePlugin)
            .add_plugin(concert_hall::ConcertHallPlugin)
            .add_plugin(mayor::MayorPlugin)
            .add_plugin(tavern::TavernPlugin)
            .add_system(town_ambience);
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
    let playing = app_state.0.is_town();
    if playing != state.last_playing {
        state.last_playing = playing;
        if playing {
            commands
                .spawn_empty()
                .insert(
                    AudioPlusSource::new(asset_library.sound_effects.sfx_town_ambient.clone())
                        .as_looping(),
                )
                .insert(Persistent)
                .insert(TownAmbience);
            commands
                .spawn_empty()
                .insert(
                    AudioPlusSource::new(asset_library.sound_effects.sfx_town_music.clone())
                        .as_looping(),
                )
                .insert(Persistent)
                .insert(TownAmbience);
        } else {
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub mod concert_hall;
pub mod mayor;
pub mod outside;
pub mod tavern;
