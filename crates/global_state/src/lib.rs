use bevy::{
    ecs::{schedule::StateData, system::Resource},
    prelude::*,
};
pub use global_state_macros::GlobalState;
use std::marker::PhantomData;

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

#[derive(Default)]
pub struct StateTime<T>
where
    T: Resource,
{
    pub time: f32,
    _phantom: PhantomData<T>,
}

impl<T> StateTime<T>
where
    T: Resource,
{
    pub fn just_entered(&self) -> bool {
        self.time < 0.1
    }
}

pub fn reset_state_time<T>(mut state_time: ResMut<StateTime<T>>)
where
    T: Resource,
{
    state_time.time = 0.;
}

pub fn update_state_time<T>(mut state_time: ResMut<StateTime<T>>, time: Res<Time>)
where
    T: Resource,
{
    state_time.time += time.delta_seconds();
}

pub fn cleanup_entities(
    mut commands: Commands,
    query: Query<Entity, (Without<Persistent>, Without<Parent>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub mod prelude;
