use crate::common::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct JagerossaCutsceneState;

pub struct JagerossaCutscenePlugin;

impl Plugin for JagerossaCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<JagerossaCutsceneState>()
            .add_cutscene::<JagerossaCutscene>();
    }
}

#[derive(Default, Debug, Clone)]
pub struct JagerossaCutscene {
    pub boat: Option<Entity>,
    pub from: Vec2,
    pub to: Vec2,
}

impl Cutscene for JagerossaCutscene {
    fn build(cutscene: &mut CutsceneBuilder) {
        cutscene.add_dialogue_step(init1);
    }
}

fn init1(mut dialogue: ResMut<Dialogue>) {
    dialogue.add_text(
        "Ha-ha! Sailed right into me ambush ya bilge rat! I'll paint ya ship back with gunpowder!"
            .to_owned(),
    );
    dialogue.add_text("Then I'll take yer instrument from your scorched corpse!".to_owned());
}
