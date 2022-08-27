use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct UpgradesState {
    hovered: Option<UpgradesType>,
}

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UpgradesState>()
            .add_event::<UpgradesSpawnEvent>()
            .add_system(upgrades_spawn)
            .add_system(upgrades_skill_points)
            .add_system(upgrades_ability_bg)
            .add_system(upgrades_buttons)
            .add_system(upgrades_stars);
    }
}

#[derive(Default, Clone, Copy)]
pub struct UpgradesSpawnEvent;

#[derive(Component)]
pub struct UpgradesAbilityBg {
    locked: bool,
    upgrade: UpgradesType,
}

#[derive(Component)]
pub struct UpgradesSkillPoints;

#[derive(Component)]
pub struct UpgradesButton {
    locked: bool,
    upgrade: UpgradesType,
}

#[derive(Component)]
pub struct UpgradesStar {
    level: u32,
    upgrade: UpgradesType,
}

#[derive(Debug, Clone)]
struct UpgradesDisplayInfo {
    name: &'static str,
    texture: Handle<Image>,
    offset: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UpgradesType {
    Guitar,
    Drums,
    Flute,
    Harmonica,
    Accordion,
    Defense,
}

impl UpgradesType {
    fn from_index(i: u32) -> Self {
        match i {
            0 => Self::Guitar,
            1 => Self::Drums,
            2 => Self::Flute,
            3 => Self::Harmonica,
            4 => Self::Accordion,
            5 => Self::Defense,
            _ => unreachable!(),
        }
    }
    fn display_info(&self, asset_library: &AssetLibrary) -> UpgradesDisplayInfo {
        match *self {
            Self::Guitar => UpgradesDisplayInfo {
                name: "Guitar",
                texture: asset_library.sprite_upgrades_ability_guitar.clone(),
                offset: Vec2::new(0., 0.),
            },
            Self::Drums => UpgradesDisplayInfo {
                name: "Drums",
                texture: asset_library.sprite_upgrades_ability_drums.clone(),
                offset: Vec2::new(0., 0.),
            },
            Self::Flute => UpgradesDisplayInfo {
                name: "Flute",
                texture: asset_library.sprite_upgrades_ability_flute.clone(),
                offset: Vec2::new(0., -20.),
            },
            Self::Harmonica => UpgradesDisplayInfo {
                name: "Harmonica",
                texture: asset_library.sprite_upgrades_ability_harmonica.clone(),
                offset: Vec2::new(0., -25.),
            },
            Self::Accordion => UpgradesDisplayInfo {
                name: "Accordion",
                texture: asset_library.sprite_upgrades_ability_accordion.clone(),
                offset: Vec2::new(0., -5.),
            },
            Self::Defense => UpgradesDisplayInfo {
                name: "Defense",
                texture: asset_library.sprite_upgrades_ability_defense.clone(),
                offset: Vec2::new(0., 0.),
            },
        }
    }
    fn current_level(&self, game_state: &GameState) -> u32 {
        match *self {
            Self::Guitar => game_state.attacks.forward_cannons,
            Self::Drums => game_state.attacks.shotgun_cannons,
            Self::Flute => game_state.attacks.shockwave,
            Self::Harmonica => game_state.attacks.bombs,
            Self::Accordion => game_state.attacks.kraken,
            Self::Defense => 0,
        }
    }
    fn increase_level(&self, game_state: &mut GameState) {
        match *self {
            Self::Guitar => game_state.attacks.forward_cannons += 1,
            Self::Drums => game_state.attacks.shotgun_cannons += 1,
            Self::Flute => game_state.attacks.shockwave += 1,
            Self::Harmonica => game_state.attacks.bombs += 1,
            Self::Accordion => game_state.attacks.kraken += 1,
            Self::Defense => {}
        }
    }
}

fn upgrades_spawn(
    mut ev_upgrades_spawn: EventReader<UpgradesSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut state: ResMut<UpgradesState>,
    game_state: Res<GameState>,
) {
    for _ in ev_upgrades_spawn.iter() {
        *state = UpgradesState::default();
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_library.sprite_upgrades_bg.clone(),
                ..Default::default()
            })
            .insert(
                Transform2::new()
                    .with_scale(Vec2::ONE * 0.3)
                    .with_depth(DEPTH_LAYER_UPGRADES_BG),
            )
            .with_children(|parent| {
                parent
                    .spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            "99",
                            TextStyle {
                                font: asset_library.font_bold.clone(),
                                font_size: 100.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment {
                            horizontal: HorizontalAlign::Right,
                            vertical: VerticalAlign::Center,
                        }),
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_xy(940., 740.).with_depth(DEPTH_LAYER_UPGRADES_ABILITY_BG),
                    )
                    .insert(UpgradesSkillPoints);
                parent
                    .spawn_bundle(Transform2Bundle::default())
                    .insert_bundle(VisibilityBundle::default())
                    .with_children(|parent| {
                        for i in 0..6 {
                            let row = (i / 2) as f32;
                            let col = (i % 2) as f32;
                            let x = -625. + col * 1200.;
                            let y = 470. - row * 410.;
                            let upgrade_type = UpgradesType::from_index(i);
                            let display_info = upgrade_type.display_info(asset_library.as_ref());
                            let locked = upgrade_type.current_level(game_state.as_ref()) == 0;
                            parent
                                .spawn_bundle(SpriteSheetBundle {
                                    texture_atlas: asset_library
                                        .sprite_upgrades_ability_bg_atlas
                                        .clone(),
                                    ..Default::default()
                                })
                                .insert(
                                    Transform2::from_xy(x, y)
                                        .with_depth(DEPTH_LAYER_UPGRADES_ABILITY_BG),
                                )
                                .insert(UpgradesAbilityBg {
                                    locked,
                                    upgrade: upgrade_type,
                                })
                                .insert(Clickable {
                                    shape: CollisionShape::Rect {
                                        size: Vec2::new(1050., 400.),
                                    },
                                    use_global: true,
                                    offset: Vec2::new(0., -20.),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    if !locked {
                                        parent
                                            .spawn_bundle(SpriteBundle {
                                                texture: display_info.texture.clone(),
                                                ..Default::default()
                                            })
                                            .insert(Transform2::from_translation(
                                                Vec2::new(-335., 95.) + display_info.offset,
                                            ))
                                            .insert(Label(String::from(display_info.name)));
                                    }
                                    parent
                                        .spawn_bundle(Text2dBundle {
                                            text: Text::from_section(
                                                if locked { "Locked" } else { display_info.name },
                                                TextStyle {
                                                    font: asset_library.font_bold.clone(),
                                                    font_size: 100.0,
                                                    color: Color::rgb_u8(52, 52, 52),
                                                },
                                            )
                                            .with_alignment(TextAlignment {
                                                horizontal: HorizontalAlign::Left,
                                                vertical: VerticalAlign::Center,
                                            }),
                                            ..Default::default()
                                        })
                                        .insert(Transform2::from_xy(-210., 68.));
                                    parent
                                        .spawn_bundle(SpriteSheetBundle {
                                            texture_atlas: asset_library
                                                .sprite_upgrades_button_atlas
                                                .clone(),
                                            ..Default::default()
                                        })
                                        .insert(Transform2::from_xy(420., 68.))
                                        .insert(UpgradesButton {
                                            locked,
                                            upgrade: upgrade_type,
                                        })
                                        .insert(Clickable {
                                            shape: CollisionShape::Rect {
                                                size: Vec2::new(132., 111.),
                                            },
                                            use_global: true,
                                            ..Default::default()
                                        });
                                    if !locked {
                                        for j in 0..5 {
                                            let mut x = -417. + (j as f32) * 212.;
                                            if j > 2 {
                                                x += 9.;
                                            }
                                            if j > 3 {
                                                x += 10.;
                                            }
                                            if col == 1. {
                                                x += 1.;
                                                if j == 1 || j == 3 {
                                                    x += 1.;
                                                }
                                                if j == 2 {
                                                    x += 3.;
                                                }
                                            }
                                            parent
                                                .spawn_bundle(SpriteBundle {
                                                    texture: asset_library
                                                        .sprite_upgrades_star
                                                        .clone(),
                                                    ..Default::default()
                                                })
                                                .insert(Transform2::from_xy(x, -107.))
                                                .insert(UpgradesStar {
                                                    level: j,
                                                    upgrade: upgrade_type,
                                                });
                                        }
                                    }
                                });
                        }
                    });
            });
    }
}

fn upgrades_skill_points(
    mut query: Query<&mut Text, With<UpgradesSkillPoints>>,
    game_state: Res<GameState>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{}", game_state.skill_points);
    }
}

fn upgrades_ability_bg(
    mut query: Query<(&mut TextureAtlasSprite, &Clickable, &UpgradesAbilityBg)>,
    mut state: ResMut<UpgradesState>,
) {
    let mut new_hover = None;
    for (mut sprite, clickable, bg) in query.iter_mut() {
        if bg.locked {
            sprite.index = 2;
            continue;
        }
        sprite.index = 0;
        if clickable.hovered {
            new_hover = Some(bg.upgrade);
        }
        let mut current_hover = false;
        if let Some(hovered) = state.hovered {
            if hovered == bg.upgrade {
                current_hover = true;
            }
        }
        if current_hover {
            sprite.index = 1;
        }
        if current_hover && !clickable.hovered {
            state.hovered = None;
        }
    }
    if let Some(new_hover) = new_hover {
        state.hovered = Some(new_hover);
    }
}

fn upgrades_buttons(
    mut query: Query<(
        &Clickable,
        &mut TextureAtlasSprite,
        &mut Transform2,
        &UpgradesButton,
    )>,
    mut game_state: ResMut<GameState>,
) {
    for (clickable, mut sprite, mut transform, button) in query.iter_mut() {
        if button.locked {
            sprite.index = 3;
            continue;
        }
        if game_state.skill_points == 0 || button.upgrade.current_level(game_state.as_ref()) == 5 {
            sprite.index = 2;
            continue;
        }
        sprite.index = 0;
        transform.translation = Vec2::new(420., 68.);
        if clickable.hovered {
            sprite.index = 1;
            if clickable.clicked {
                transform.translation.x -= 3.;
                transform.translation.y -= 5.;
            }
        }
        if clickable.confirmed {
            button.upgrade.increase_level(game_state.as_mut());
            if game_state.skill_points != 0 {
                game_state.skill_points -= 1;
            }
        }
    }
}

fn upgrades_stars(mut query: Query<(&mut Visibility, &UpgradesStar)>, game_state: Res<GameState>) {
    for (mut visibility, star) in query.iter_mut() {
        visibility.is_visible = star.upgrade.current_level(game_state.as_ref()) > star.level;
    }
}
