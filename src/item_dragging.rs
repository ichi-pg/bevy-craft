use crate::input::Input;
use crate::item::*;
use crate::item_container::*;
use bevy::prelude::*;

#[derive(Component, Default)]
struct DragItem;

#[derive(Component)]
struct DragArea;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum ItemDragged {
    None,
    Dragged,
}

fn spawn_drag_area(mut commands: Commands) {
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
    ));
}

fn drag_item(
    area_query: Query<Entity, With<DragArea>>,
    query: Query<(&Interaction, &ItemID, &Amount), Changed<Interaction>>,
    mut next_state: ResMut<NextState<ItemDragged>>,
    mut commands: Commands,
) {
    for entity in &area_query {
        for (intersection, item_id, amount) in &query {
            match intersection {
                Interaction::Pressed => {
                    commands.entity(entity).with_children(|parent| {
                        build_item::<DragItem>(parent, item_id.0, amount.0);
                    });
                    next_state.set(ItemDragged::Dragged);
                }
                Interaction::Hovered => continue,
                Interaction::None => continue,
            }
        }
    }
}

fn dragging_item(mut query: Query<&mut Style, With<DragItem>>, input: Res<Input>) {
    for mut style in &mut query {
        style.left = Val::Px(input.window_cursor.x);
        style.top = Val::Px(input.window_cursor.y);
    }
}

fn drop_item(
    mut drop_query: Query<
        (&Interaction, &mut ItemID, &mut Amount),
        (Without<DragItem>, Changed<Interaction>),
    >,
    mut drag_query: Query<(Entity, &mut ItemID, &mut Amount), With<DragItem>>,
    mut next_state: ResMut<NextState<ItemDragged>>,
    mut commands: Commands,
) {
    for (intersection, mut drop_item_id, mut drop_amount) in &mut drop_query {
        match intersection {
            Interaction::Pressed => {
                for (entity, mut drag_item_id, mut drag_amount) in &mut drag_query {
                    if drop_item_id.0 == 0 {
                        drop_item_id.0 = drag_item_id.0;
                        drop_amount.0 = drag_amount.0;
                        commands.entity(entity).despawn_recursive();
                        next_state.set(ItemDragged::None);
                    } else if drop_item_id.0 == drag_item_id.0 {
                        drop_item_id.0 += drag_item_id.0;
                        drop_amount.0 += drag_amount.0;
                        commands.entity(entity).despawn_recursive();
                        next_state.set(ItemDragged::None);
                    } else {
                        let item_id = drop_item_id.0;
                        let amount = drop_amount.0;
                        drop_item_id.0 = drag_item_id.0;
                        drop_amount.0 = drag_amount.0;
                        drag_item_id.0 = item_id;
                        drag_amount.0 = amount;
                    }
                }
            }
            Interaction::Hovered => continue,
            Interaction::None => continue,
        }
    }
    // FIXME spawn at the same time
}

pub struct ItemDraggingPlugin;

impl Plugin for ItemDraggingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ItemDragged::None);
        app.add_systems(Startup, spawn_drag_area);
        app.add_systems(
            Update,
            (
                drag_item.run_if(in_state(ItemDragged::None)),
                dragging_item,
                drop_item,
            ),
        );
    }
}
