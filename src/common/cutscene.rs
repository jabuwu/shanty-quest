use bevy::{
    ecs::schedule::{IntoSystemDescriptor, ShouldRun},
    prelude::*,
};
use std::{any::TypeId, collections::VecDeque, marker::PhantomData};

const UPDATE_BUFFER: u32 = 3;

#[derive(Default, Debug)]
struct CutsceneState {
    running_cutscene: Option<RunningCutscene>,
    backlog_cutscenes: VecDeque<TypeId>,
}

impl CutsceneState {
    fn queue(&mut self, type_id: TypeId) {
        self.backlog_cutscenes.push_back(type_id);
    }

    fn try_run_next(&mut self) {
        if matches!(self.running_cutscene, None) {
            if let Some(type_id) = self.backlog_cutscenes.pop_front() {
                self.running_cutscene = Some(RunningCutscene {
                    type_id,
                    init: true,
                    updates: 0,
                    step: 0,
                });
            }
        }
    }
}

#[derive(Debug)]
struct RunningCutscene {
    type_id: TypeId,
    init: bool,
    updates: u32,
    step: usize,
}

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CutsceneState>()
            .add_system(cutscene_debug);
    }
}

pub trait AddAppCutscene {
    fn add_cutscene<T>(&mut self)
    where
        T: Cutscene + Default + Send + Sync + 'static;
}

impl AddAppCutscene for App {
    fn add_cutscene<T>(&mut self)
    where
        T: Cutscene + Default + Send + Sync + 'static,
    {
        self.init_resource::<T>();
        self.add_event::<CutsceneStartEvent<T>>();
        self.add_event::<CutsceneContinueEvent<T>>();
        self.add_system(cutscene_start::<T>);
        self.add_system(cutscene_continue::<T>);
        let mut builder = CutsceneBuilder {
            app: self,
            type_id: TypeId::of::<T>(),
            step: 0,
        };
        T::build(&mut builder);
        let step = builder.step;
        self.add_system(
            move |mut cutscene_state: ResMut<CutsceneState>, mut state: ResMut<T>| {
                if let Some(running_cutscene) = &mut cutscene_state.running_cutscene {
                    if running_cutscene.type_id == TypeId::of::<T>()
                        && running_cutscene.step == step
                    {
                        *state = T::default();
                        cutscene_state.running_cutscene = None;
                        cutscene_state.try_run_next();
                    }
                }
            },
        );
    }
}

pub struct CutsceneBuilder<'a> {
    app: &'a mut App,
    type_id: TypeId,
    step: usize,
}

impl<'a> CutsceneBuilder<'a> {
    pub fn add_step<ParamsA, ParamsB>(
        &mut self,
        init: impl IntoSystemDescriptor<ParamsA> + ParallelSystemDescriptorCoercion<ParamsA>,
        update: impl IntoSystemDescriptor<ParamsB> + ParallelSystemDescriptorCoercion<ParamsB>,
    ) -> &mut Self {
        let type_id = self.type_id;
        let step = self.step;
        self.app.add_system(init.with_run_criteria(
            move |mut cutscene_state: ResMut<CutsceneState>| {
                if let Some(running_cutscene) = &mut cutscene_state.running_cutscene {
                    if running_cutscene.type_id == type_id
                        && running_cutscene.init
                        && running_cutscene.step == step
                    {
                        running_cutscene.init = false;
                        return ShouldRun::Yes;
                    }
                }
                ShouldRun::No
            },
        ));
        self.app.add_system(update.with_run_criteria(
            move |mut cutscene_state: ResMut<CutsceneState>| {
                if let Some(running_cutscene) = &mut cutscene_state.running_cutscene {
                    if running_cutscene.type_id == type_id
                        && !running_cutscene.init
                        && running_cutscene.step == step
                    {
                        if running_cutscene.updates == UPDATE_BUFFER {
                            return ShouldRun::Yes;
                        } else {
                            running_cutscene.updates += 1;
                        }
                    }
                }
                ShouldRun::No
            },
        ));
        self.step += 1;
        self
    }

    pub fn add_quick_step<ParamsA>(
        &mut self,
        init: impl IntoSystemDescriptor<ParamsA> + ParallelSystemDescriptorCoercion<ParamsA>,
    ) -> &mut Self {
        let step = self.step;
        self.add_step(init, move |mut cutscene_state: ResMut<CutsceneState>| {
            if let Some(running_cutscene) = &mut cutscene_state.running_cutscene {
                running_cutscene.init = true;
                running_cutscene.updates = 0;
                running_cutscene.step = step + 1;
            }
        });
        self
    }
}

pub trait Cutscene {
    fn build(cutscene: &mut CutsceneBuilder);
}

#[derive(Default, Clone, Copy)]
pub struct CutsceneStartEvent<T>
where
    T: Cutscene + Default + Send + Sync + 'static,
{
    _phantom: PhantomData<T>,
}

#[derive(Default, Clone, Copy)]
pub struct CutsceneContinueEvent<T>
where
    T: Cutscene + Default + Send + Sync + 'static,
{
    _phantom: PhantomData<T>,
}

fn cutscene_start<T>(
    mut ev_cutscene_start: EventReader<CutsceneStartEvent<T>>,
    mut cutscene_state: ResMut<CutsceneState>,
) where
    T: Cutscene + Default + Send + Sync + 'static,
{
    for _ in ev_cutscene_start.iter() {
        cutscene_state.queue(TypeId::of::<T>());
        cutscene_state.try_run_next();
    }
}

fn cutscene_continue<T>(
    mut ev_cutscene_continue: EventReader<CutsceneContinueEvent<T>>,
    mut cutscene_state: ResMut<CutsceneState>,
) where
    T: Cutscene + Default + Send + Sync + 'static,
{
    let mut continued = false;
    for _ in ev_cutscene_continue.iter() {
        if !continued {
            continued = true;
            if let Some(running_cutscene) = &mut cutscene_state.running_cutscene {
                if running_cutscene.type_id == TypeId::of::<T>()
                    && running_cutscene.updates == UPDATE_BUFFER
                {
                    running_cutscene.init = true;
                    running_cutscene.updates = 0;
                    running_cutscene.step += 1;
                }
            }
        }
    }
}

fn cutscene_debug(cutscene_state: ResMut<CutsceneState>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::D) {
        println!("{:?}", cutscene_state.as_ref());
    }
}
