use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct BandSelectionPlugin;

impl Plugin for BandSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BandSelectionSpawnEvent>()
            .add_system(band_selection_spawn)
            .add_system(band_selection_drag);
    }
}

#[derive(Default, Clone, Copy)]
pub struct BandSelectionSpawnEvent;

#[derive(Component)]
pub struct BandSelection;

#[derive(Component)]
pub struct BandSelectionSlot {
    index: usize,
    shape: CollisionShape,
}

#[derive(Component)]
pub struct BandSelectionDraggable {
    member: BandMember,
    shape: CollisionShape,
}

fn band_selection_spawn(
    mut ev_band_selection_spawn: EventReader<BandSelectionSpawnEvent>,
    mut commands: Commands,
    band_selection_query: Query<Entity, With<BandSelection>>,
    asset_library: Res<AssetLibrary>,
    game_state: Res<GameState>,
) {
    for _ in ev_band_selection_spawn.iter() {
        for entity in band_selection_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        let slot_shape = CollisionShape::Rect {
            size: Vec2::new(121., 129.),
        };
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_library.sprite_band_selection_bg.clone(),
                ..Default::default()
            })
            .insert(Transform2::new().with_depth((DepthLayer::Front, 0.)))
            .insert(BandSelection)
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: game_state.band_members[0]
                            .selection_active_image(asset_library.as_ref()),
                        ..Default::default()
                    })
                    .insert(Transform2::from_xy(-276., 79.).with_depth((DepthLayer::Front, 0.1)))
                    .insert(BandSelectionSlot {
                        index: 0,
                        shape: slot_shape.clone(),
                    })
                    .insert(Label("Band Slot 1".to_owned()));
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: game_state.band_members[1]
                            .selection_active_image(asset_library.as_ref()),
                        ..Default::default()
                    })
                    .insert(Transform2::from_xy(-96., 79.).with_depth((DepthLayer::Front, 0.1)))
                    .insert(BandSelectionSlot {
                        index: 1,
                        shape: slot_shape.clone(),
                    })
                    .insert(Label("Band Slot 2".to_owned()));
                for slot in 0..BandMember::len() {
                    let x = -276. + 138. * slot as f32;
                    let member = BandMember::from_index(slot);
                    let transform2 =
                        Transform2::from_xy(x, -115.).with_depth((DepthLayer::Front, 0.1));
                    let label = Label(format!("Band Draggable ({:?})", member));
                    if game_state.member_in_band(member) {
                        parent
                            .spawn_bundle(SpriteBundle {
                                texture: member.selection_inactive_image(asset_library.as_ref()),
                                ..Default::default()
                            })
                            .insert(transform2)
                            .insert(label);
                    } else if slot < game_state.band_unlocked_count {
                        parent
                            .spawn_bundle(SpriteBundle {
                                texture: member.selection_active_image(asset_library.as_ref()),
                                ..Default::default()
                            })
                            .insert(transform2)
                            .insert(label)
                            .insert(BandSelectionDraggable {
                                member,
                                shape: slot_shape.clone(),
                            });
                    } else {
                        parent
                            .spawn_bundle(SpriteBundle {
                                texture: asset_library.sprite_band_selection_slot_locked.clone(),
                                ..Default::default()
                            })
                            .insert(transform2)
                            .insert(label);
                    }
                }
            });
    }
}

struct BandSelectionDrag {
    entity: Entity,
    member: BandMember,
    offset: Vec2,
}

#[derive(Default)]
struct BandSelectionDragState {
    drag: Option<BandSelectionDrag>,
}

fn band_selection_drag(
    click_query: Query<(
        Entity,
        &GlobalTransform,
        Option<&BandSelectionDraggable>,
        Option<&BandSelectionSlot>,
    )>,
    mut drag_query: Query<&mut Transform2>,
    mouse: Res<Mouse>,
    input: Res<Input<MouseButton>>,
    mut state: Local<BandSelectionDragState>,
    mut ev_band_selection_spawn: EventWriter<BandSelectionSpawnEvent>,
    mut game_state: ResMut<GameState>,
) {
    if let Some(drag) = &state.drag {
        if input.just_released(MouseButton::Left) && state.drag.is_some() {
            for (_, transform, _, slot) in click_query.iter() {
                if let Some(slot) = slot {
                    if slot.shape.overlaps(
                        transform.translation().truncate(),
                        CollisionShape::Point,
                        mouse.position,
                    ) {
                        game_state.band_members[slot.index] = drag.member;
                    }
                }
            }
            ev_band_selection_spawn.send_default();
            state.drag = None;
            return;
        }
        if let Ok(mut drag_transform) = drag_query.get_mut(drag.entity) {
            drag_transform.translation = mouse.position + drag.offset * 0.7;
            drag_transform.scale = Vec2::ONE * 0.7;
            drag_transform.depth = 0.2;
        }
    } else {
        if input.just_pressed(MouseButton::Left) {
            for (entity, transform, draggable, _) in click_query.iter() {
                if let Some(draggable) = draggable {
                    if draggable.shape.overlaps(
                        transform.translation().truncate(),
                        CollisionShape::Point,
                        mouse.position,
                    ) {
                        let offset = transform.translation().truncate() - mouse.position;
                        state.drag = Some(BandSelectionDrag {
                            entity,
                            member: draggable.member,
                            offset,
                        });
                        break;
                    }
                }
            }
        }
    }
}
