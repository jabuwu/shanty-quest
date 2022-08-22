use std::marker::PhantomData;

use bevy::prelude::*;

pub trait ComponentChild {
    fn add_component_child<C, M>(&mut self) -> &mut Self
    where
        C: Component,
        M: Component + Default;
}

impl ComponentChild for App {
    fn add_component_child<C, M>(&mut self) -> &mut Self
    where
        C: Component,
        M: Component + Default,
    {
        self.add_event::<ComponentChildCreatedEvent<M>>()
            .add_system(component_child_attach::<C, M>)
            .add_system_to_stage(CoreStage::PostUpdate, component_child_detach::<C, M>)
    }
}

#[derive(Clone, Copy)]
pub struct ComponentChildCreatedEvent<T> {
    pub entity: Entity,
    _phantom: PhantomData<T>,
}

fn component_child_attach<C, M>(
    mut commands: Commands,
    query: Query<Entity, Added<C>>,
    mut ev_created: EventWriter<ComponentChildCreatedEvent<M>>,
) where
    C: Component,
    M: Component + Default,
{
    for entity in query.iter() {
        let child = commands.spawn().insert(M::default()).id();
        commands.entity(entity).add_child(child);
        ev_created.send(ComponentChildCreatedEvent {
            entity: child,
            _phantom: PhantomData::default(),
        });
    }
}

fn component_child_detach<C, M>(
    mut commands: Commands,
    removals: RemovedComponents<C>,
    children_query: Query<&Children>,
    child_query: Query<Entity, With<M>>,
) where
    C: Component,
    M: Component + Default,
{
    for entity in removals.iter() {
        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                if child_query.contains(*child) {
                    commands.entity(entity).remove_children(&[*child]);
                    commands.entity(*child).despawn();
                }
            }
        }
    }
}
