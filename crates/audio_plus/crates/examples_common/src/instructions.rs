use bevy::prelude::*;

struct Instructions(String);

pub struct InstructionsPlugin(pub String);

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Instructions(self.0.clone()))
            .add_startup_system(instructions_spawn);
    }
}

fn instructions_spawn(
    instructions: Res<Instructions>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(50.),
                        left: Val::Px(50.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    instructions.0.clone(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            });
        });
}
