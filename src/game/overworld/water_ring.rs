use crate::common::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct WaterRingPlugin;

#[derive(Default)]
pub struct WaterRingSettings {
    pub start_scale: f32,
    pub max_life_time: f32,
    pub spawn_offset: Vec2,
}

impl Plugin for WaterRingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaterRingSettings {
            start_scale: 0.45,
            max_life_time: 0.8,
            spawn_offset: Vec2::new(-10.0, -20.0),
        })
        .add_event::<WaterRingSpawnEvent>()
        .add_system(water_ring_spawn)
        .add_system(water_ring_update)
        .add_system(water_ring_debug);
    }
}

#[derive(Clone, Copy)]
pub struct WaterRingSpawnEvent {
    pub entity: Option<Entity>,
    pub position: Vec2,
    pub scale: Vec2,
    pub angle: f32,
    pub face: Facing,
}

#[derive(Component)]
pub struct WaterRing {
    pub start_scale: f32,
    pub life_time: f32,
    pub max_life_time: f32,
}

fn water_ring_spawn(
    mut ev_spawn: EventReader<WaterRingSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    water_ring_settings: ResMut<WaterRingSettings>,
) {
    for event in ev_spawn.iter() {
        let offset = match event.face {
            Facing::West => water_ring_settings.spawn_offset,
            Facing::East => water_ring_settings.spawn_offset,
            _ => Vec2::ZERO,
        };
        let mut entity = if let Some(entity) = event.entity {
            commands.entity(entity)
        } else {
            commands.spawn()
        };
        entity
            .insert_bundle(SpriteBundle {
                texture: asset_library.sprite_water_ring_vfx.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::from_translation(event.position + (event.scale * offset))
                    .with_scale(Vec2::new(
                        water_ring_settings.start_scale,
                        water_ring_settings.start_scale,
                    ))
                    .with_rotation(event.angle)
                    .with_depth(DEPTH_LAYER_BOAT_TRAIL),
            )
            .insert(WaterRing {
                start_scale: water_ring_settings.start_scale,
                life_time: water_ring_settings.max_life_time,
                max_life_time: water_ring_settings.max_life_time,
            });
    }
}

fn water_ring_update(
    mut query: Query<(&mut Transform2, &mut WaterRing, Entity, &mut Sprite)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut transform, mut water_ring, entity, mut sprite) in query.iter_mut() {
        water_ring.life_time -= time.delta_seconds();
        if water_ring.life_time <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        let interp = water_ring.life_time / water_ring.max_life_time;
        sprite.color.set_a(interp);

        let start_scale = Vec2::new(water_ring.start_scale, water_ring.start_scale);
        transform.scale = water_ring.start_scale + (1.0 - start_scale) * (1.0 - interp);
    }
}

fn water_ring_debug(
    mut egui_context: ResMut<EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    mut water_ring_settings: ResMut<WaterRingSettings>,
) {
    menu_bar.item("Water Ring VFX", |open| {
        egui::Window::new("Water Ring VFX")
            .open(open)
            .show(egui_context.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Life Time");
                    ui.add(egui::Slider::new(
                        &mut water_ring_settings.max_life_time,
                        0.0..=10.0,
                    ));
                });
                ui.horizontal(|ui| {
                    ui.label("Start Scale");
                    ui.add(egui::Slider::new(
                        &mut water_ring_settings.start_scale,
                        0.0..=1.0,
                    ));
                });
                ui.horizontal(|ui| {
                    ui.label("Offset");
                    ui.add(
                        egui::DragValue::new(&mut water_ring_settings.spawn_offset.x)
                            .speed(1)
                            .clamp_range(-100..=100),
                    );
                    ui.add(
                        egui::DragValue::new(&mut water_ring_settings.spawn_offset.y)
                            .speed(1)
                            .clamp_range(-100..=100),
                    );
                });
            });
    });
}
