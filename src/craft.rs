use crate::hotbar::*;
use crate::input::*;
use crate::inventory::*;
use crate::item::*;
use crate::item_container::*;
use crate::item_dragging::*;
use crate::ui_parts::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct ProductItem;

#[derive(Component)]
pub struct CraftProduct;

#[derive(Component)]
pub struct CraftMaterial;

#[derive(Component, Default)]
pub struct CraftUI;

fn spawn_recipes(mut commands: Commands) {
    for item in [
        (101, 1, vec![(2, 1), (3, 1)]),
        (102, 1, vec![(101, 1), (4, 1)]),
        (103, 1, vec![(101, 1), (5, 1), (6, 1)]),
    ] {
        commands
            .spawn((CraftProduct, ItemID(item.0), ItemAmount(item.1)))
            .with_children(|parent| {
                for material in item.2 {
                    parent.spawn((CraftMaterial, ItemID(material.0), ItemAmount(material.1)));
                }
            });
    }
    // TODO hash map with product id?
}

fn spawn_items(query: Query<(&ItemID, &ItemAmount), With<CraftProduct>>, mut commands: Commands) {
    commands
        .spawn(screen_node(4, 2, AlignItems::Center))
        .with_children(|parent: &mut ChildBuilder| {
            parent
                .spawn(grid_space(10, 2, JustifyContent::Start))
                .with_children(|parent| {
                    parent
                        .spawn((grid_node(3, 2, Visibility::Hidden), CraftUI))
                        .with_children(|parent| {
                            for (index, (item_id, amount)) in query.iter().enumerate() {
                                build_item::<ProductItem>(
                                    parent,
                                    item_id.0,
                                    amount.0,
                                    index as u8,
                                    false,
                                );
                            }
                        });
                });
        });
    // TODO workbench
    // TODO hand craft recipes
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
                                false,
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
}

pub struct CraftPlugin;

impl Plugin for CraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_recipes, spawn_items).chain());
        app.add_systems(Update, click_recipe);
    }
}
