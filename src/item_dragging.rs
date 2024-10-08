use crate::atlas::*;
use crate::camera::*;
use crate::equipment::*;
use crate::hotbar::*;
use crate::input::*;
use crate::inventory::*;
use crate::item::*;
use crate::item_attribute::ItemAttributeMap;
use crate::item_node::*;
use crate::player::*;
use crate::storage::*;
use crate::ui_hovered::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct DragItem;

#[derive(Component)]
pub struct DragArea;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ItemDragged {
    None,
    PreNone,
    Dragged,
    PreDragged,
}

fn spawn_drag_area(camera_query: Query<Entity, With<PlayerCamera>>, mut commands: Commands) {
    for entity in &camera_query {
        commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            DragArea,
            TargetCamera(entity),
        ));
    }
}

fn drag_item<T: Component>(
    area_query: Query<Entity, With<DragArea>>,
    mut query: Query<(&Interaction, &mut ItemID, &mut ItemAmount), (With<T>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<ItemDragged>>,
    mut commands: Commands,
    shift: Res<ShiftLeft>,
    control: Res<ControlLeft>,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
) {
    if shift.pressed {
        return;
    }
    for (intersection, mut item_id, mut amount) in &mut query {
        if item_id.0 == 0 {
            continue;
        }
        let Some(attribute) = attribute_map.get(&item_id.0) else {
            return;
        };
        let Some(atlas) = atlas_map.get(&attribute.atlas_id) else {
            return;
        };
        match intersection {
            Interaction::Pressed => {
                let remain_amount = if control.pressed {
                    (amount.0 as f32 * 0.5).floor() as u16
                } else {
                    0
                };
                for entity in &area_query {
                    commands.entity(entity).with_children(|parent| {
                        build_item::<DragItem>(
                            parent,
                            item_id.0,
                            amount.0 - remain_amount,
                            0,
                            attribute,
                            atlas,
                        );
                    });
                }
                if remain_amount == 0 {
                    item_id.0 = 0;
                }
                amount.0 = remain_amount;
                next_state.set(ItemDragged::PreDragged);
            }
            Interaction::Hovered => continue,
            Interaction::None => continue,
        }
    }
    // TODO take half when shift + right click
    // TODO take ten when control + right click
    // TODO take one when right click
}

fn dragging_item(mut query: Query<&mut Style, With<DragItem>>, window_cursor: Res<WindowCursor>) {
    for mut style in &mut query {
        style.left = Val::Px(window_cursor.position.x);
        style.top = Val::Px(window_cursor.position.y);
    }
}

fn put_in_item<T: Component>(
    mut slot_query: Query<
        (&Interaction, &mut ItemID, &mut ItemAmount),
        (With<T>, Without<DragItem>, Changed<Interaction>),
    >,
    mut drag_query: Query<(Entity, &mut ItemID, &mut ItemAmount), With<DragItem>>,
    mut next_state: ResMut<NextState<ItemDragged>>,
    mut commands: Commands,
) {
    for (intersection, mut slot_item_id, mut slot_amount) in &mut slot_query {
        match intersection {
            Interaction::Pressed => {
                for (entity, mut drag_item_id, mut drag_amount) in &mut drag_query {
                    if slot_item_id.0 == 0 || slot_item_id.0 == drag_item_id.0 {
                        // Overwrite or Merge
                        slot_item_id.0 = drag_item_id.0;
                        slot_amount.0 += drag_amount.0;
                        commands.entity(entity).despawn_recursive();
                        next_state.set(ItemDragged::PreNone);
                    } else {
                        // Swap
                        let item_id = slot_item_id.0;
                        let amount = slot_amount.0;
                        slot_item_id.0 = drag_item_id.0;
                        slot_amount.0 = drag_amount.0;
                        drag_item_id.0 = item_id;
                        drag_amount.0 = amount;
                    }
                }
            }
            Interaction::Hovered => continue,
            Interaction::None => continue,
        }
    }
    // FIXME b0003 when into other container? (fixed by in_state dragged?)
    // FIXME sometimes item is dropped when after pushed out? (fixed by in_state dragged?)
    // FIXME sometimes item is placed? (fixed by in_state dragged?)
    // TODO put half when shift + right click
    // TODO put ten when control + right click
    // TODO put one when right click
}

fn drop_item(
    player_query: Query<(&Transform, &Direction2), With<PlayerController>>,
    query: Query<(Entity, &ItemID, &ItemAmount), With<DragItem>>,
    left_click: Res<LeftClick>,
    mut commands: Commands,
    mut event_writer: EventWriter<ItemDropped>,
    mut next_state: ResMut<NextState<ItemDragged>>,
) {
    if !left_click.just_pressed {
        return;
    }
    for (entity, item_id, amount) in &query {
        for (transform, direction) in &player_query {
            event_writer.send(ItemDropped {
                position: Vec2::new(
                    transform.translation.x + direction.x * 200.0,
                    transform.translation.y + direction.y * 200.0,
                ),
                item_id: item_id.0,
                amount: amount.0,
            });
            commands.entity(entity).despawn_recursive();
            next_state.set(ItemDragged::PreNone);
        }
    }
}

pub fn change_state(
    state: ItemDragged,
) -> impl FnMut(Res<LeftClick>, ResMut<NextState<ItemDragged>>) {
    move |left_click, mut next_state| {
        if left_click.pressed {
            return;
        }
        next_state.set(state);
    }
}

pub struct ItemDraggingPlugin;

impl Plugin for ItemDraggingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ItemDragged::None);
        app.add_systems(Startup, spawn_drag_area);
        app.add_systems(
            Update,
            (
                (
                    drag_item::<HotbarItem>,
                    drag_item::<InventoryItem>,
                    drag_item::<StorageItem>,
                    drag_item::<EquipmentItem>,
                )
                    .run_if(in_state(ItemDragged::None)),
                dragging_item,
                (
                    put_in_item::<HotbarItem>,
                    put_in_item::<InventoryItem>,
                    put_in_item::<StorageItem>,
                    put_in_item::<EquipmentItem>,
                    drop_item.run_if(in_state(UIHovered::None)),
                )
                    .run_if(in_state(ItemDragged::Dragged)),
                change_state(ItemDragged::Dragged).run_if(in_state(ItemDragged::PreDragged)),
                change_state(ItemDragged::None).run_if(in_state(ItemDragged::PreNone)),
            ),
        );
    }
}
