pub use global_state_macros::GlobalState;
use bevy::{ecs::schedule::StateData, prelude::*};

pub trait GlobalState {
    fn init_global_state(app: &mut App);
}

pub trait AddGlobalState {
    fn add_global_state<T: GlobalState + Default + StateData>(&mut self) -> &mut Self;
}

impl AddGlobalState for App {
    fn add_global_state<T: GlobalState + Default + StateData>(&mut self) -> &mut Self {
        T::init_global_state(self);
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
