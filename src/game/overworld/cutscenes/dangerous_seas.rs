use crate::{common::prelude::*, game::prelude::DANGEROUS_SEAS};
use bevy::prelude::*;

pub struct DangerousSeasCutscenePlugin;

impl Plugin for DangerousSeasCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_cutscene::<DangerousSeasCutscene>();
    }
}

#[derive(Default, Debug, Clone)]
pub struct DangerousSeasCutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for DangerousSeasCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_dialogue_step(init1);
    }
}

fn init1(mut dialogue: ResMut<Dialogue>) {
    for (p, t) in DANGEROUS_SEAS.iter() {
        dialogue.add_text(*p, String::from(*t));
    }
}
