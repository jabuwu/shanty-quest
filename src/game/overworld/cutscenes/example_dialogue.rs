use crate::common::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct ExampleDialogueCutsceneState;

pub struct ExampleDialogueCutscenePlugin;

impl Plugin for ExampleDialogueCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExampleDialogueCutsceneState>()
            .add_cutscene::<ExampleDialogueCutscene>();
    }
}

#[derive(Default, Debug, Clone)]
pub struct ExampleDialogueCutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for ExampleDialogueCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_dialogue_step(init1);
    }
}

fn init1(mut dialogue: ResMut<Dialogue>) {
    dialogue.add_text("Hello world\nPress space to continue".to_owned());
    dialogue.add_text("Move by clicking left mouse button\nShoot with F and D".to_owned());
}
