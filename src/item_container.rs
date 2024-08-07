use crate::chest::*;
use crate::hotbar::*;
use crate::input::Input;
use crate::inventory::*;
use crate::item::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct UI;

#[derive(Component, Default)]
struct DragItem;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum ItemDragged {
    None,
    Dragged,
}

fn build_item<T: Component + Default>(parent: &mut ChildBuilder, item_id: u16, amount: u16) {
    parent
        .spawn((
            ImageBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ..default()
            },
            Interaction::None,
            ItemID(item_id),
            Amount(amount),
            T::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section("", TextStyle { ..default() }),
                ItemID(item_id),
                Amount(amount),
            ));
        });
    // TODO texture
}

fn build_container<T: Component + Default, U: Component + Default>(
    parent: &mut ChildBuilder,
    x: u16,
    y: u16,
    visibility: Visibility,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px((x * 110 + 10) as f32),
                    height: Val::Px((y * 110 + 10) as f32),
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(x, 1.0),
                    row_gap: Val::Px(10.0),
                    column_gap: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                visibility,
                ..default()
            },
            Interaction::None,
            UI,
            T::default(),
        ))
        .with_children(|parent| {
            for _ in 0..x * y {
                build_item::<U>(parent, 0, 0);
            }
        });
}

fn spawn_containers(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                row_gap: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent: &mut ChildBuilder| {
            build_container::<Chest, ChestItem>(parent, 10, 4, Visibility::Hidden);
            build_container::<Inventory, InventoryItem>(parent, 10, 4, Visibility::Hidden);
            build_container::<Hotbar, HotbarItem>(parent, 10, 1, Visibility::Inherited);
        });
}

fn drag_item(
    query: Query<(&Interaction, &ItemID, &Amount), Changed<Interaction>>,
    mut next_state: ResMut<NextState<ItemDragged>>,
    mut commands: Commands,
) {
    for (intersection, item_id, amount) in &query {
        match intersection {
            Interaction::Pressed => {
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            // align_items: AlignItems::Center,
                            // justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        build_item::<DragItem>(parent, item_id.0, amount.0);
                    });
                next_state.set(ItemDragged::Dragged);
            }
            Interaction::Hovered => continue,
            Interaction::None => continue,
        }
    }
}

fn dragging_item(mut query: Query<&mut Style, With<DragItem>>, input: Res<Input>) {
    for mut style in &mut query {
        style.left = Val::Px(input.window_cursor.x);
        style.top = Val::Px(input.window_cursor.y);
    }
}

fn put_in_item<T: Event + ItemAndAmount, U: Component, V: Event + Default + ItemAndAmount>(
    mut query: Query<(Entity, &mut ItemID, &mut Amount), With<U>>,
    mut event_reader: EventReader<T>,
    mut event_writer: EventWriter<V>,
) {
    for event in event_reader.read() {
        // Merge amount
        let mut found = false;
        let mut empty = None;
        for (entity, item_id, mut amount) in &mut query {
            if item_id.0 == event.item_id() {
                amount.0 += event.amount();
                found = true;
                break;
            }
            if empty.is_none() && item_id.0 == 0 {
                empty = Some(entity);
            }
        }
        if found {
            continue;
        }
        // Empty slot
        match empty {
            Some(entity) => match query.get_mut(entity) {
                Ok((_, mut item_id, mut amount)) => {
                    if item_id.0 == 0 {
                        item_id.0 = event.item_id();
                        amount.0 += event.amount();
                        break;
                    }
                }
                Err(_) => todo!(),
            },
            None => {
                // Overflow
                let mut event: V = V::default();
                event.set_item_id(event.item_id());
                event.set_amount(event.amount());
                event_writer.send(event);
            }
        }
    }
    // TODO which player?
    // TODO closed chests items is hash map resource?
    // TODO using state
}

fn sync_children<T: Component>(
    parent_query: Query<
        (&Children, &ItemID, &Amount),
        (With<T>, Or<(Changed<ItemID>, Changed<Amount>)>),
    >,
    mut child_query: Query<(&mut ItemID, &mut Amount), (With<Node>, Without<T>)>,
) {
    for (children, parent_item_id, parent_amount) in &parent_query {
        for child in children.iter() {
            match child_query.get_mut(*child) {
                Ok((mut child_item_id, mut child_amount)) => {
                    child_item_id.0 = parent_item_id.0;
                    child_amount.0 = parent_amount.0;
                }
                Err(_) => continue,
            }
        }
    }
}

pub struct ItemContainerPlugin;

impl Plugin for ItemContainerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ItemDragged::None);
        app.add_systems(Startup, spawn_containers);
        app.add_systems(
            Update,
            (
                put_in_item::<ItemPickedUp, HotbarItem, HotbarOverflowed>,
                put_in_item::<HotbarOverflowed, InventoryItem, InventoryOverflowed>,
                put_in_item::<ChestOverflowed, InventoryItem, InventoryOverflowed>,
                sync_children::<UiImage>,
                drag_item.run_if(in_state(ItemDragged::None)),
                dragging_item,
                // TODO item push
                // TODO spawn item when inventory overflowed
            ),
        );
    }
}
