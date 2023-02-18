use crate::common::prelude::*;
use bevy::{
    ecs::schedule::{IntoSystemDescriptor, ShouldRun},
    prelude::*,
};
use bevy_egui::{egui, EguiContext};
use std::{
    any::{type_name, TypeId},
    collections::VecDeque,
    marker::PhantomData,
};

pub trait CutsceneType: Cutscene + Default + Clone + Send + Sync + 'static {}
impl<T> CutsceneType for T where T: Cutscene + Default + Clone + Send + Sync + 'static {}

const UPDATE_BUFFER: u32 = 3;

#[derive(Default, Debug, Resource)]
pub struct Cutscenes {
    running_cutscene: Option<RunningCutscene>,
    backlog_cutscenes: VecDeque<(String, TypeId)>,
}

impl Cutscenes {
    fn queue(&mut self, name: String, type_id: TypeId) {
        self.backlog_cutscenes.push_back((name, type_id));
    }

    fn try_run_next(&mut self) {
        if matches!(self.running_cutscene, None) {
            if let Some((name, type_id)) = self.backlog_cutscenes.pop_front() {
                self.running_cutscene = Some(RunningCutscene {
                    name,
                    type_id,
                    started: false,
                    init: true,
                    updates: 0,
                    step: 0,
                    skip: false,
                    time: 0.,
                });
            }
        }
    }

    pub fn skipping(&self) -> bool {
        if let Some(running_cutscene) = &self.running_cutscene {
            running_cutscene.skip
        } else {
            false
        }
    }

    pub fn running(&self) -> bool {
        !matches!(self.running_cutscene, None)
    }

    pub fn clear(&mut self) {
        self.running_cutscene = None;
        self.backlog_cutscenes = VecDeque::new();
    }
}

#[derive(Debug)]
struct RunningCutscene {
    name: String,
    type_id: TypeId,
    started: bool,
    init: bool,
    updates: u32,
    step: usize,
    skip: bool,
    time: f32,
}

impl RunningCutscene {
    fn reset(&mut self) {
        self.init = true;
        self.updates = 0;
        self.time = 0.;
    }
}

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cutscenes>()
            .init_resource::<Cutscenes>()
            .add_system(cutscene_debug);
    }
}

pub trait AddAppCutscene {
    fn add_cutscene<T>(&mut self) -> &mut Self
    where
        T: CutsceneType + Resource;
}

impl AddAppCutscene for App {
    fn add_cutscene<T>(&mut self) -> &mut Self
    where
        T: CutsceneType + Resource,
    {
        self.init_resource::<T>();
        self.init_resource::<CutsceneInitialValues<T>>();
        self.add_event::<CutsceneStartEvent<T>>();
        self.add_event::<CutsceneContinueEvent<T>>();
        self.add_event::<CutsceneSkipEvent<T>>();
        self.add_system(cutscene_start::<T>);
        self.add_system(cutscene_continue::<T>);
        self.add_system(cutscene_skip::<T>);
        let mut builder = CutsceneBuilder {
            app: self,
            type_id: TypeId::of::<T>(),
            step: 0,
        };
        T::build(&mut builder);
        let step = builder.step;
        self.add_system(
            move |mut cutscene_state: ResMut<Cutscenes>,
                  mut state: ResMut<T>,
                  mut initial_values: ResMut<CutsceneInitialValues<T>>| {
                if let Some(running_cutscene) = &mut cutscene_state.running_cutscene {
                    if running_cutscene.type_id == TypeId::of::<T>() {
                        if !running_cutscene.started {
                            *state = initial_values.0.pop_front().unwrap();
                            running_cutscene.started = true;
                        } else if running_cutscene.step == step {
                            cutscene_state.running_cutscene = None;
                            cutscene_state.try_run_next();
                        } else if running_cutscene.skip {
                            if running_cutscene.updates == UPDATE_BUFFER {
                                running_cutscene.reset();
                                running_cutscene.step += 1;
                            }
                        }
                    }
                }
            },
        );
        self
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
        init: impl IntoSystemDescriptor<ParamsA> + IntoSystemDescriptor<ParamsA>,
        update: impl IntoSystemDescriptor<ParamsB> + IntoSystemDescriptor<ParamsB>,
    ) -> &mut Self {
        let type_id = self.type_id;
        let step = self.step;
        self.app.add_system(init.with_run_criteria(
            move |mut cutscene_state: ResMut<Cutscenes>| {
                if let Some(running_cutscene) = &mut cutscene_state.running_cutscene {
                    if running_cutscene.type_id == type_id
                        && running_cutscene.init
                        && running_cutscene.started
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
            move |mut cutscene_state: ResMut<Cutscenes>| {
                if let Some(running_cutscene) = &mut cutscene_state.running_cutscene {
                    if running_cutscene.type_id == type_id
                        && !running_cutscene.init
                        && running_cutscene.started
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

    pub fn add_update_step<ParamsA>(
        &mut self,
        update: impl IntoSystemDescriptor<ParamsA> + IntoSystemDescriptor<ParamsA>,
    ) -> &mut Self {
        self.add_step(|| {}, update)
    }

    pub fn add_quick_step<ParamsA>(
        &mut self,
        init: impl IntoSystemDescriptor<ParamsA> + IntoSystemDescriptor<ParamsA>,
    ) -> &mut Self {
        let step = self.step;
        self.add_step(init, move |mut state: ResMut<Cutscenes>| {
            Self::to_step(state.as_mut(), step + 1);
        });
        self
    }

    pub fn add_dialogue_step<ParamsA>(
        &mut self,
        init: impl IntoSystemDescriptor<ParamsA> + IntoSystemDescriptor<ParamsA>,
    ) -> &mut Self {
        let step = self.step;
        self.add_step(
            init,
            move |mut state: ResMut<Cutscenes>, time: Res<Time>, dialogue: Res<Dialogue>| {
                let advance = if let Some(running_cutscene) = &mut state.running_cutscene {
                    running_cutscene.time += time.delta_seconds();
                    running_cutscene.time > 0.2 && !dialogue.visible()
                } else {
                    false
                };
                if advance {
                    Self::to_step(state.as_mut(), step + 1);
                }
            },
        );
        self
    }

    pub fn add_timed_step<ParamsA>(
        &mut self,
        init: impl IntoSystemDescriptor<ParamsA> + IntoSystemDescriptor<ParamsA>,
        seconds: f32,
    ) -> &mut Self {
        let step = self.step;
        self.add_step(
            init,
            move |mut state: ResMut<Cutscenes>, time: Res<Time>| {
                let advance = if let Some(running_cutscene) = &mut state.running_cutscene {
                    running_cutscene.time += time.delta_seconds();
                    running_cutscene.time > seconds
                } else {
                    false
                };
                if advance {
                    Self::to_step(state.as_mut(), step + 1);
                }
            },
        );
        self
    }

    fn to_step(state: &mut Cutscenes, step: usize) {
        if let Some(running_cutscene) = &mut state.running_cutscene {
            running_cutscene.reset();
            running_cutscene.step = step;
        }
    }
}

pub trait Cutscene {
    fn build(cutscene: &mut CutsceneBuilder);
}

#[derive(Default, Clone, Copy)]
pub struct CutsceneStartEvent<T>(pub T)
where
    T: CutsceneType;

#[derive(Default, Clone, Copy)]
pub struct CutsceneContinueEvent<T>
where
    T: CutsceneType,
{
    _phantom: PhantomData<T>,
}

#[derive(Default, Clone, Copy)]
pub struct CutsceneSkipEvent<T>
where
    T: CutsceneType,
{
    _phantom: PhantomData<T>,
}

#[derive(Default, Resource)]
struct CutsceneInitialValues<T>(VecDeque<T>)
where
    T: CutsceneType;

fn cutscene_start<T>(
    mut ev_cutscene_start: EventReader<CutsceneStartEvent<T>>,
    mut cutscene_state: ResMut<Cutscenes>,
    mut initial_values: ResMut<CutsceneInitialValues<T>>,
) where
    T: CutsceneType,
{
    for event in ev_cutscene_start.iter() {
        initial_values.0.push_back(event.0.clone());
        cutscene_state.queue(String::from(type_name::<T>()), TypeId::of::<T>());
        cutscene_state.try_run_next();
    }
}

fn cutscene_continue<T>(
    mut ev_cutscene_continue: EventReader<CutsceneContinueEvent<T>>,
    mut cutscene_state: ResMut<Cutscenes>,
) where
    T: CutsceneType,
{
    let mut continued = false;
    for _ in ev_cutscene_continue.iter() {
        if !continued {
            continued = true;
            if let Some(running_cutscene) = &mut cutscene_state.running_cutscene {
                if running_cutscene.type_id == TypeId::of::<T>()
                    && running_cutscene.updates == UPDATE_BUFFER
                {
                    running_cutscene.reset();
                    running_cutscene.step += 1;
                }
            }
        }
    }
}

fn cutscene_skip<T>(
    mut ev_cutscene_skip: EventReader<CutsceneSkipEvent<T>>,
    mut cutscene_state: ResMut<Cutscenes>,
) where
    T: CutsceneType,
{
    for _ in ev_cutscene_skip.iter() {
        if let Some(running_cutscene) = &mut cutscene_state.running_cutscene {
            if running_cutscene.type_id == TypeId::of::<T>() {
                running_cutscene.skip = true;
            }
        }
    }
}

fn cutscene_debug(
    mut egui_context: ResMut<EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    mut cutscene_state: ResMut<Cutscenes>,
) {
    menu_bar.item("Cutscenes", |open| {
        egui::Window::new("Cutscenes")
            .open(open)
            .show(egui_context.ctx_mut(), |ui| {
                if let Some(cutscene) = &mut cutscene_state.running_cutscene {
                    ui.label(format!("Cutscene running: {}", cutscene.name));
                    ui.label(format!("Step: {}", cutscene.step));
                    if ui.button("Next").clicked() {
                        cutscene.reset();
                        cutscene.step += 1;
                    }
                    if ui.button("Skip All").clicked() {
                        cutscene.skip = true;
                    }
                } else {
                    ui.label("No cutscene running");
                }
            });
    });
}
