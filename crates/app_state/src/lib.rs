pub use app_state_macros::AppState;
use bevy::{ecs::schedule::StateData, prelude::*};

pub trait AppState {
    fn init_app_state(app: &mut App);
}

pub trait AddAppState {
    fn add_app_state<T: AppState + Default + StateData>(&mut self) -> &mut Self;
}

impl AddAppState for App {
    fn add_app_state<T: AppState + Default + StateData>(&mut self) -> &mut Self {
        T::init_app_state(self);
        self
    }
}

#[derive(Component)]
pub struct Persistent;

pub fn cleanup_entities(
    mut commands: Commands,
    query: Query<Entity, (Without<Persistent>, Without<Parent>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub mod prelude;
