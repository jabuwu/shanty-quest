use bevy::prelude::*;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(rubble::RubblePlugin)
            .add_plugin(dangerous_seas_trigger::DangerousSeasTriggerPlugin);
    }
}

pub mod dangerous_seas_trigger;
pub mod rubble;
