use crate::camera::*;
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

#[derive(Component, Default, NodeItem)]
pub struct ProductItem;

#[derive(Component, Default)]
pub struct CraftUI;

fn spawn_items(
    camera_query: Query<Entity, With<PlayerCamera>>,
    mut commands: Commands,
    recipes: Res<CraftRecipes>,
) {
    for entity in &camera_query {
        commands.build_screen(
            entity,
            INVENTORY_Y + 1,
            2,
            JustifyContent::End,
            AlignItems::Center,
            |parent| {
                build_space(parent, INVENTORY_X, 2, JustifyContent::Start, |parent| {
                    build_grid::<CraftUI>(parent, 3, 2, Visibility::Hidden, |parent| {
                        for (index, item_id) in [101, 102, 103, 104].iter().enumerate() {
                            match recipes.get(item_id) {
                                Some(recipe) => build_item::<ProductItem>(
                                    parent,
                                    *item_id,
                                    recipe.amount,
                                    index as u8,
                                ),
                                None => todo!(),
                            }
                        }
                    });
                });
            },
        );
    }
}

fn click_recipe(
    intersection_query: Query<(&Interaction, &ItemID), (With<ProductItem>, Changed<Interaction>)>,
    mut query: Query<
        (&mut ItemID, &mut ItemAmount),
        (
            Or<(With<HotbarItem>, With<InventoryItem>)>,
            Without<ProductItem>,
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
            Without<HotbarItem>,
            Without<InventoryItem>,
        ),
    >,
    control: Res<Control>,
    recipes: Res<CraftRecipes>,
) {
    for (intersection, intersection_item_id) in &intersection_query {
        for (item_id, _) in &drag_query {
            if item_id.0 != intersection_item_id.0 {
                return;
            }
        }
        match *intersection {
            Interaction::Pressed => match recipes.get(&intersection_item_id.0) {
                Some(recipe) => {
                    let mut times = if control.pressed { 10 } else { 1 };
                    for material in &recipe.materials {
                        let mut sum = 0;
                        for (item_id, amount) in &query {
                            if item_id.0 == material.item_id {
                                sum += amount.0;
                            }
                        }
                        times = times.min(sum / material.amount);
                        if times == 0 {
                            break;
                        }
                    }
                    if times == 0 {
                        continue;
                    }
                    for material in &recipe.materials {
                        let mut consume_amount = material.amount * times;
                        for (mut item_id, mut amount) in &mut query {
                            if item_id.0 == material.item_id {
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
                    for (item_id, mut amount) in &mut drag_query {
                        if item_id.0 == intersection_item_id.0 {
                            amount.0 += recipe.amount * times;
                            return;
                        }
                    }
                    for entity in &area_query {
                        commands.entity(entity).with_children(|parent| {
                            build_item::<DragItem>(
                                parent,
                                intersection_item_id.0,
                                recipe.amount * times,
                                0,
                            );
                        });
                    }
                    next_state.set(ItemDragged::PreDragged);
                }
                None => todo!(),
            },
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
