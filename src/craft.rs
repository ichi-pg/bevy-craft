use crate::atlas::*;
use crate::camera::*;
use crate::craft_recipe::*;
use crate::hotbar::*;
use crate::input::*;
use crate::inventory::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::item_dragging::*;
use crate::item_id::*;
use crate::item_node::*;
use crate::ui_parts::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct ProductItem;

#[derive(Component, Default)]
pub struct CraftUI;

fn spawn_items(
    camera_query: Query<Entity, With<PlayerCamera>>,
    mut commands: Commands,
    recipe_map: Res<CraftRecipeMap>,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
) {
    let Some(attribute) = attribute_map.get(&0) else {
        return;
    };
    let Some(atlas) = atlas_map.get(&attribute.atlas_id) else {
        return;
    };
    for entity in &camera_query {
        commands.build_screen(
            entity,
            INVENTORY_Y + 1,
            2,
            JustifyContent::End,
            AlignItems::Center,
            |parent| {
                build_space(parent, INVENTORY_X, 2, JustifyContent::Start, |parent| {
                    build_panel_grid::<CraftUI>(parent, 3, 2, Visibility::Hidden, |parent| {
                        for (index, item_id) in [
                            WOOD_PICKAXE_ITEM_ID,
                            WOOD_SWORD_ITEM_ID,
                            STORAGE_ITEM_ID,
                            WORKBENCH_ITEM_ID,
                        ]
                        .iter()
                        .enumerate()
                        {
                            match recipe_map.get(item_id) {
                                Some(recipe) => build_item::<ProductItem>(
                                    parent,
                                    *item_id,
                                    recipe.amount,
                                    index as u8,
                                    attribute,
                                    atlas,
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
    control: Res<ControlLeft>,
    recipe_map: Res<CraftRecipeMap>,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
) {
    for (intersection, intersection_item_id) in &intersection_query {
        for (item_id, _) in &drag_query {
            if item_id.0 != intersection_item_id.0 {
                return;
            }
        }
        let Some(attribute) = attribute_map.get(&intersection_item_id.0) else {
            return;
        };
        let Some(atlas) = atlas_map.get(&attribute.atlas_id) else {
            return;
        };
        match *intersection {
            Interaction::Pressed => match recipe_map.get(&intersection_item_id.0) {
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
                                attribute,
                                atlas,
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
