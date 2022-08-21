use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

const OCEAN_WIDTH: f32 = 320.;
const OCEAN_HEIGHT: f32 = 320.;

pub struct OceanPlugin;

impl Plugin for OceanPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OceanSpawnEvent>()
            .add_system(ocean_spawn)
            .add_system(ocean_update.after(PlayerSystems::Camera))
            .add_system(ocean_overlay_update)
            .add_system(ocean_debug);
    }
}

#[derive(Default, Clone, Copy)]
pub struct OceanSpawnEvent;

#[derive(Component, Clone, Copy)]
pub struct Ocean;

#[derive(Component, Clone, Copy)]
pub struct OceanOverlay {
    time: f32,
    wavey: f32,
    parallax: f32,
}

fn ocean_spawn(
    mut ev_spawn: EventReader<OceanSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(50000., 50000.).into(),
                    color: Color::rgb_u8(0, 167, 217),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Transform2::from_xy(0., 0.).with_depth((DepthLayer::Environment, 0.0)))
            .insert(Ocean)
            .with_children(|parent| {
                for layer in 0..=1 {
                    let (texture, offset, opacity, wavey, parallax) = if layer == 0 {
                        (
                            &asset_library.sprite_water_overlay,
                            Vec2::new(30., 30.),
                            0.14,
                            0.2,
                            0.,
                        )
                    } else {
                        (
                            &asset_library.sprite_water_overlay2,
                            Vec2::new(0., 0.),
                            0.14,
                            0.3,
                            -0.05,
                        )
                    };
                    parent
                        .spawn_bundle(Transform2Bundle::default())
                        .insert_bundle(VisibilityBundle::default())
                        .insert(OceanOverlay {
                            time: 0.,
                            wavey,
                            parallax,
                        })
                        .with_children(|parent| {
                            for x in -10..=10 {
                                for y in -10..=10 {
                                    parent
                                        .spawn_bundle(SpriteBundle {
                                            sprite: Sprite {
                                                custom_size: Vec2::new(OCEAN_WIDTH, OCEAN_HEIGHT)
                                                    .into(),
                                                color: Color::rgba(1., 1., 1., opacity),
                                                ..Default::default()
                                            },
                                            texture: texture.clone(),
                                            ..Default::default()
                                        })
                                        .insert(
                                            Transform2::from_xy(
                                                x as f32 * OCEAN_WIDTH + offset.x,
                                                y as f32 * OCEAN_HEIGHT + offset.y,
                                            )
                                            .with_depth((DepthLayer::Environment, 0.01)),
                                        );
                                }
                            }
                        });
                }
            });
    }
}

fn clamp_vec2(vec: Vec2, size: Vec2) -> Vec2 {
    Vec2::new(
        (vec.x / size.x).floor() * size.x,
        (vec.y / size.y).floor() * size.y,
    )
}

fn ocean_update(
    mut queries: ParamSet<(
        Query<&Transform2, With<Camera>>,
        Query<&mut Transform2, With<Ocean>>,
    )>,
) {
    let camera_translation = if let Ok(camera_transform) = queries.p0().get_single() {
        Some(camera_transform.translation)
    } else {
        None
    };
    if let Some(camera_translation) = camera_translation {
        for mut transform in queries.p1().iter_mut() {
            transform.translation =
                clamp_vec2(camera_translation, Vec2::new(OCEAN_WIDTH, OCEAN_HEIGHT));
        }
    }
}

fn ocean_overlay_update(
    mut queries: ParamSet<(
        Query<&Transform2, With<Camera>>,
        Query<(&mut OceanOverlay, &mut Transform2)>,
    )>,
    time: Res<Time>,
) {
    let camera_translation = if let Ok(camera_transform) = queries.p0().get_single() {
        Some(camera_transform.translation)
    } else {
        None
    };
    for (mut overlay, mut transform) in queries.p1().iter_mut() {
        overlay.time += time.delta_seconds() * overlay.wavey;
        transform.translation =
            Vec2::new(overlay.time.cos() * 20., (overlay.time * 0.22).sin() * 8.);
        if let Some(camera_translation) = camera_translation {
            let mut parallax_offset = camera_translation * overlay.parallax;
            parallax_offset.x = parallax_offset.x % OCEAN_WIDTH;
            parallax_offset.y = parallax_offset.y % OCEAN_HEIGHT;
            transform.translation += parallax_offset;
        }
    }
}

fn ocean_debug(
    mut egui_context: ResMut<EguiContext>,
    mut menu_bar: ResMut<MenuBar>,
    mut ocean_query: Query<&mut Transform2, With<Ocean>>,
    mut overlay_query: Query<(&mut OceanOverlay, &Children)>,
    mut children_query: Query<&mut Sprite>,
) {
    menu_bar.item("Ocean", |open| {
        egui::Window::new("Ocean")
            .open(open)
            .show(egui_context.ctx_mut(), |ui| {
                for mut ocean_transform in ocean_query.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.label("Scale");
                        ui.add(egui::Slider::new(&mut ocean_transform.scale.x, 0.05..=2.0));
                        ocean_transform.scale.y = ocean_transform.scale.x;
                    });
                }
                for (i, (mut overlay, children)) in overlay_query.iter_mut().enumerate() {
                    ui.label(format!("Ocean Overlay {}", i + 1));
                    ui.horizontal(|ui| {
                        ui.label("Wavey");
                        ui.add(egui::Slider::new(&mut overlay.wavey, 0.0..=1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Parallax");
                        ui.add(egui::Slider::new(&mut overlay.parallax, -1.0..=1.0));
                    });
                    let mut opacity = None;
                    for child in children.iter() {
                        if let Ok(mut sprite) = children_query.get_mut(*child) {
                            if opacity.is_none() {
                                let mut o = sprite.color.a();
                                ui.horizontal(|ui| {
                                    ui.label("Opacity");
                                    ui.add(egui::Slider::new(&mut o, 0.0..=1.0));
                                });
                                opacity = Some(o);
                            }
                            sprite.color.set_a(opacity.unwrap());
                        }
                    }
                }
            });
    });
}
