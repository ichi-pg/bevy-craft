use crate::craft_recipe::*;
use crate::hotbar::*;
use crate::input::*;
use crate::inventory::*;
use crate::item::*;
use crate::item_dragging::*;
use crate::item_node::*;
use crate::ui_parts::*;
use bevy::prelude::*;
use bevy_craft::*;
use std::collections::*;

#[derive(Component, Default, NodeItem)]
pub struct ProductItem;

#[derive(Component, Default)]
pub struct CraftUI;

fn spawn_items(query: Query<(&ItemID, &ItemAmount), With<CraftProduct>>, commands: Commands) {
    build_spaced::<CraftUI>(
        commands,
        INVENTORY_Y + 1,
        2,
        AlignItems::Center,
        INVENTORY_X,
        2,
        JustifyContent::Start,
        3,
        2,
        Visibility::Hidden,
        |parent| {
            let item_ids = HashSet::<u16>::from_iter([101, 102, 103]);
            for (index, (item_id, amount)) in query.iter().enumerate() {
                if !item_ids.contains(&item_id.0) {
                    continue;
                }
                build_item::<ProductItem>(parent, item_id.0, amount.0, index as u8);
            }
        },
    );
}

fn click_recipe(
    intersection_query: Query<(&Interaction, &ItemID), (With<ProductItem>, Changed<Interaction>)>,
    product_query: Query<(&Children, &ItemID, &ItemAmount), With<CraftProduct>>,
    material_query: Query<(&ItemID, &ItemAmount), With<CraftMaterial>>,
    mut query: Query<
        (&mut ItemID, &mut ItemAmount),
        (
            Or<(With<HotbarItem>, With<InventoryItem>)>,
            Without<ProductItem>,
            Without<CraftProduct>,
            Without<CraftMaterial>,
            Without<DragItem>,
        ),
    >,
    area_query: Query<Entity, With<DragArea>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<ItemDragged>>,
    mut drag_query: Query<
        (&ItemID, &mut ItemAmount),
        (
            With<DragItem>,
            Without<ProductItem>,
            Without<CraftProduct>,
            Without<CraftMaterial>,
            Without<HotbarItem>,
            Without<InventoryItem>,
        ),
    >,
    control: Res<Control>,
) {
    for (intersection, intersection_item_id) in &intersection_query {
        for (item_id, _) in &drag_query {
            if item_id.0 != intersection_item_id.0 {
                return;
            }
        }
        match *intersection {
            Interaction::Pressed => {
                for (children, product_item_id, product_amount) in &product_query {
                    if product_item_id.0 != intersection_item_id.0 {
                        continue;
                    }
                    let mut times = if control.pressed { 10 } else { 1 };
                    for child in children.iter() {
                        match material_query.get(*child) {
                            Ok((material_item_id, material_amount)) => {
                                let mut sum = 0;
                                for (item_id, amount) in &query {
                                    if item_id.0 == material_item_id.0 {
                                        sum += amount.0;
                                    }
                                }
                                times = times.min(sum / material_amount.0);
                                if times == 0 {
                                    break;
                                }
                            }
                            Err(_) => todo!(),
                        }
                    }
                    if times == 0 {
                        continue;
                    }
                    for child in children.iter() {
                        match material_query.get(*child) {
                            Ok((material_item_id, material_amount)) => {
                                let mut consume_amount = material_amount.0 * times;
                                for (mut item_id, mut amount) in &mut query {
                                    if item_id.0 == material_item_id.0 {
                                        if amount.0 > consume_amount {
                                            amount.0 -= consume_amount;
                                            break;
                                        } else {
                                            consume_amount -= amount.0;
                                            item_id.0 = 0;
                                            amount.0 = 0;
                                        }
                                    }
                                }
                            }
                            Err(_) => todo!(),
                        }
                    }
                    for (item_id, mut amount) in &mut drag_query {
                        if item_id.0 == product_item_id.0 {
                            amount.0 += product_amount.0 * times;
                            return;
                        }
                    }
                    for entity in &area_query {
                        commands.entity(entity).with_children(|parent| {
                            build_item::<DragItem>(
                                parent,
                                product_item_id.0,
                                product_amount.0 * times,
                                0,
                            );
                        });
                    }
                    next_state.set(ItemDragged::PreDragged);
                }
            }
            Interaction::Hovered => continue,
            Interaction::None => continue,
        }
    }
    // TODO optimize recipe query
    // TODO optimize sum
    // TODO storage items
    // TODO commonize drag item
    // TODO max item amount
}

pub struct CraftPlugin;

impl Plugin for CraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_items);
        app.add_systems(Update, click_recipe);
    }
}
