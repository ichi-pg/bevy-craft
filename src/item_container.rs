use crate::chest::*;
use crate::hotbar::*;
use crate::inventory::*;
use crate::item::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct UI;

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
                parent
                    .spawn((
                        NodeBundle {
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
                        ItemID(0),
                        U::default(),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section("", TextStyle { ..default() }),
                            Amount(0),
                        ));
                    });
            }
        });
    // TODO texture
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

fn put_in_item<T: Event + ItemAndAmount, U: Component, V: Event + Default + ItemAndAmount>(
    mut parent_query: Query<(Entity, &Children, &mut ItemID), With<U>>,
    mut child_query: Query<&mut Amount, With<Text>>,
    mut event_reader: EventReader<T>,
    mut event_writer: EventWriter<V>,
) {
    for event in event_reader.read() {
        // Merge amount
        let mut found = false;
        let mut empty = None;
        for (entity, children, item_id) in &parent_query {
            if item_id.0 == event.item_id() {
                for &child in children.iter() {
                    match child_query.get_mut(child) {
                        Ok(mut amount) => {
                            amount.0 += event.amount();
                        }
                        Err(_) => continue,
                    }
                }
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
            Some(entity) => match parent_query.get_mut(entity) {
                Ok((_, children, mut item_id)) => {
                    if item_id.0 == 0 {
                        for &child in children.iter() {
                            match child_query.get_mut(child) {
                                Ok(mut amount) => {
                                    amount.0 += event.amount();
                                }
                                Err(_) => continue,
                            }
                        }
                        item_id.0 = event.item_id();
                        break;
                    }
                }
                Err(_) => todo!(),
            },
            None => {
                // Overflow
                let mut v: V = V::default();
                v.set_item_id(event.item_id());
                v.set_amount(event.amount());
                event_writer.send(v);
            }
        }
    }
    // TODO which player?
    // TODO closed chests items is hash map resource?
    // TODO using state
}

pub struct ItemContainerPlugin;

impl Plugin for ItemContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_containers);
        app.add_systems(
            Update,
            (
                put_in_item::<ItemPickedUp, HotbarItem, HotbarOverflowed>,
                put_in_item::<HotbarOverflowed, InventoryItem, InventoryOverflowed>,
                put_in_item::<ChestOverflowed, InventoryItem, InventoryOverflowed>,
                // TODO item push
                // TODO spawn item when inventory overflowed
            ),
        );
    }
}
