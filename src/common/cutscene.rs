use crate::common::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use std::{
    any::{type_name, TypeId},
    collections::VecDeque,
    marker::PhantomData,
    sync::{RwLock, RwLockWriteGuard},
};

pub trait CutsceneType: Cutscene + Default + Clone + Send + Sync + 'static {}
impl<T> CutsceneType for T where T: Cutscene + Default + Clone + Send + Sync + 'static {}

const UPDATE_BUFFER: u32 = 3;

#[derive(Default, Debug, Resource)]
pub struct Cutscenes {
    // FIXME: the way cutscenes works is dreadful. it was a quick hack during the jam, made worse
    // by Bevy's API forbidding our awful practices, but us doing it anyway. this class needs a
    // serious overhaul, and it shouldn't be too difficult to do!
    data: RwLock<CutscenesData>,
}

#[derive(Default, Debug)]
pub struct CutscenesData {
    running_cutscene: Option<RunningCutscene>,
    backlog_cutscenes: VecDeque<(String, TypeId)>,
}

impl Cutscenes {
    fn queue(&mut self, name: String, type_id: TypeId) {
        let mut lock = self.data.write().unwrap();
        lock.backlog_cutscenes.push_back((name, type_id));
    }

    fn try_run_next(&mut self) {
        let mut lock = self.data.write().unwrap();
        if matches!(lock.running_cutscene, None) {
            if let Some((name, type_id)) = lock.backlog_cutscenes.pop_front() {
                lock.running_cutscene = Some(RunningCutscene {
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
        let lock = self.data.read().unwrap();
        if let Some(running_cutscene) = &lock.running_cutscene {
            running_cutscene.skip
        } else {
            false
        }
    }

    pub fn running(&self) -> bool {
        let lock = self.data.read().unwrap();
        !matches!(lock.running_cutscene, None)
    }

    pub fn clear(&mut self) {
        let mut lock = self.data.write().unwrap();
        lock.running_cutscene = None;
        lock.backlog_cutscenes = VecDeque::new();
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
            .add_systems(Update, cutscene_debug);
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
        self.add_systems(
            Update,
            (
                cutscene_start::<T>,
                cutscene_continue::<T>,
                cutscene_skip::<T>,
            ),
        );
        let mut builder = CutsceneBuilder {
            app: self,
            type_id: TypeId::of::<T>(),
            step: 0,
        };
        T::build(&mut builder);
        let step = builder.step;
        self.add_systems(
            Update,
            move |mut cutscene_state: ResMut<Cutscenes>,
                  mut state: ResMut<T>,
                  mut initial_values: ResMut<CutsceneInitialValues<T>>| {
                let mut try_next_run = false;
                let mut reset = false;
                {
                    let mut lock = cutscene_state.data.write().unwrap();
                    if let Some(running_cutscene) = &mut lock.running_cutscene {
                        if running_cutscene.type_id == TypeId::of::<T>() {
                            if !running_cutscene.started {
                                *state = initial_values.0.pop_front().unwrap();
                                running_cutscene.started = true;
                            } else if running_cutscene.step == step {
                                lock.running_cutscene = None;
                                try_next_run = true;
                            } else if running_cutscene.skip {
                                if running_cutscene.updates == UPDATE_BUFFER {
                                    reset = true;
                                    running_cutscene.step += 1;
                                }
                            }
                        }
                    }
                }
                if try_next_run {
                    cutscene_state.try_run_next();
                }
                if reset {
                    let mut lock = cutscene_state.data.write().unwrap();
                    if let Some(running_cutscene) = &mut lock.running_cutscene {
                        running_cutscene.reset();
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
        init: impl IntoSystemConfigs<ParamsA> + IntoSystemConfigs<ParamsA>,
        update: impl IntoSystemConfigs<ParamsB> + IntoSystemConfigs<ParamsB>,
    ) -> &mut Self {
        let type_id = self.type_id;
        let step = self.step;
        self.app.add_systems(
            Update,
            init.run_if(move |cutscene_state: Res<Cutscenes>| {
                let mut lock = cutscene_state.data.write().unwrap();
                if let Some(running_cutscene) = &mut lock.running_cutscene {
                    if running_cutscene.type_id == type_id
                        && running_cutscene.init
                        && running_cutscene.started
                        && running_cutscene.step == step
                    {
                        running_cutscene.init = false;
                        return true;
                    }
                }
                false
            }),
        );
        self.app.add_systems(
            Update,
            update.run_if(move |cutscene_state: Res<Cutscenes>| {
                let mut lock = cutscene_state.data.write().unwrap();
                if let Some(running_cutscene) = &mut lock.running_cutscene {
                    if running_cutscene.type_id == type_id
                        && !running_cutscene.init
                        && running_cutscene.started
                        && running_cutscene.step == step
                    {
                        if running_cutscene.updates == UPDATE_BUFFER {
                            return true;
                        } else {
                            running_cutscene.updates += 1;
                        }
                    }
                }
                false
            }),
        );
        self.step += 1;
        self
    }

    pub fn add_update_step<ParamsA>(
        &mut self,
        update: impl IntoSystemConfigs<ParamsA> + IntoSystemConfigs<ParamsA>,
    ) -> &mut Self {
        self.add_step(|| {}, update)
    }

    pub fn add_quick_step<ParamsA>(
        &mut self,
        init: impl IntoSystemConfigs<ParamsA> + IntoSystemConfigs<ParamsA>,
    ) -> &mut Self {
        let step = self.step;
        self.add_step(init, move |state: Res<Cutscenes>| {
            let mut lock = state.data.write().unwrap();
            Self::to_step(&mut lock, step + 1);
        });
        self
    }

    pub fn add_dialogue_step<ParamsA>(
        &mut self,
        init: impl IntoSystemConfigs<ParamsA> + IntoSystemConfigs<ParamsA>,
    ) -> &mut Self {
        let step = self.step;
        self.add_step(
            init,
            move |state: Res<Cutscenes>, time: Res<Time>, dialogue: Res<Dialogue>| {
                let mut lock = state.data.write().unwrap();
                let advance = if let Some(running_cutscene) = &mut lock.running_cutscene {
                    running_cutscene.time += time.delta_seconds();
                    running_cutscene.time > 0.2 && !dialogue.visible()
                } else {
                    false
                };
                if advance {
                    Self::to_step(&mut lock, step + 1);
                }
            },
        );
        self
    }

    pub fn add_timed_step<ParamsA>(
        &mut self,
        init: impl IntoSystemConfigs<ParamsA> + IntoSystemConfigs<ParamsA>,
        seconds: f32,
    ) -> &mut Self {
        let step = self.step;
        self.add_step(init, move |state: Res<Cutscenes>, time: Res<Time>| {
            let mut lock = state.data.write().unwrap();
            let advance = if let Some(running_cutscene) = &mut lock.running_cutscene {
                running_cutscene.time += time.delta_seconds();
                running_cutscene.time > seconds
            } else {
                false
            };
            if advance {
                Self::to_step(&mut lock, step + 1);
            }
        });
        self
    }

    fn to_step(lock: &mut RwLockWriteGuard<CutscenesData>, step: usize) {
        if let Some(running_cutscene) = &mut lock.running_cutscene {
            running_cutscene.reset();
            running_cutscene.step = step;
        }
    }
}

pub trait Cutscene {
    fn build(cutscene: &mut CutsceneBuilder);
}

#[derive(Event, Default, Clone, Copy)]
pub struct CutsceneStartEvent<T>(pub T)
where
    T: CutsceneType;

#[derive(Event, Default, Clone, Copy)]
pub struct CutsceneContinueEvent<T>
where
    T: CutsceneType,
{
    _phantom: PhantomData<T>,
}

#[derive(Event, Default, Clone, Copy)]
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
    cutscene_state: Res<Cutscenes>,
) where
    T: CutsceneType,
{
    let mut lock = cutscene_state.data.write().unwrap();
    let mut continued = false;
    for _ in ev_cutscene_continue.iter() {
        if !continued {
            continued = true;
            if let Some(running_cutscene) = &mut lock.running_cutscene {
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
    cutscene_state: Res<Cutscenes>,
) where
    T: CutsceneType,
{
    let mut lock = cutscene_state.data.write().unwrap();
    for _ in ev_cutscene_skip.iter() {
        if let Some(running_cutscene) = &mut lock.running_cutscene {
            if running_cutscene.type_id == TypeId::of::<T>() {
                running_cutscene.skip = true;
            }
        }
    }
}

fn cutscene_debug(
    mut egui_query: Query<&mut EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    cutscene_state: Res<Cutscenes>,
) {
    menu_bar.item("Cutscenes", |open| {
        let Some(mut egui_context) = egui_query.get_single_mut().ok() else { return };
        egui::Window::new("Cutscenes")
            .open(open)
            .show(egui_context.get_mut(), |ui| {
                let mut lock = cutscene_state.data.write().unwrap();
                if let Some(cutscene) = &mut lock.running_cutscene {
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
