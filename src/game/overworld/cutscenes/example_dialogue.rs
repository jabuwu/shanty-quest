use crate::common::prelude::*;
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct ExampleDialogueCutsceneState;

pub struct ExampleDialogueCutscenePlugin;

impl Plugin for ExampleDialogueCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExampleDialogueCutsceneState>()
            .add_cutscene::<ExampleDialogueCutscene>();
    }
}

#[derive(Default, Debug, Clone, Resource)]
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
    dialogue.add_text(
        DialoguePortrait::None,
        "Hello world\nPress space to continue".to_owned(),
    );
    dialogue.add_text(
        DialoguePortrait::None,
        "Move by clicking left mouse button\nShoot with F and D".to_owned(),
    );
}
