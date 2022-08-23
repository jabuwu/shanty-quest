use bevy::prelude::*;
use jam::common::prelude::*;

pub struct Q1A1Plugin;

impl Plugin for Q1A1Plugin {
    fn build(&self, app: &mut App) {
        app.add_cutscene::<Q1A1Cutscene>();
    }
}

#[derive(Default, Debug, Clone)]
pub struct Q1A1Cutscene {
    pub birdup: f32,
    pub time: f32,
}

impl Cutscene for Q1A1Cutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_step(start1, update1);
        cutscene.add_step(start2, update2);
        cutscene.add_quick_step(end1);
        cutscene.add_quick_step(end2);
    }
}

fn start1(cutscene: Res<Q1A1Cutscene>) {
    println!("start1 {:?}", cutscene);
}

fn update1(mut ev_continue: EventWriter<CutsceneContinueEvent<Q1A1Cutscene>>) {
    println!("update1");
    ev_continue.send_default();
}

fn start2(cutscene: Res<Q1A1Cutscene>) {
    println!("start2 {:?}", cutscene);
}

fn update2(
    mut cutscene: ResMut<Q1A1Cutscene>,
    mut ev_continue: EventWriter<CutsceneContinueEvent<Q1A1Cutscene>>,
    time: Res<Time>,
) {
    cutscene.time += time.delta_seconds();
    if cutscene.time > 2. {
        ev_continue.send_default();
    }
}

fn end1(cutscene: Res<Q1A1Cutscene>) {
    println!("end1 {:?}", cutscene);
}

fn end2(cutscene: Res<Q1A1Cutscene>) {
    println!("end2 {:?}", cutscene);
}
