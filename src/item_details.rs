use crate::camera::*;
use crate::craft_recipe::*;
use crate::item::*;
use crate::item_dragging::*;
use crate::item_node::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use bevy::prelude::*;
use bevy_craft::*;

#[derive(Component, Default)]
struct ItemDetails;

#[derive(Component, Default, NodeItem)]
struct MaterialItem;

fn spawn_details(camera_query: Query<Entity, With<PlayerCamera>>, commands: Commands) {
    match camera_query.get_single() {
        Ok(entity) => build_grid::<ItemDetails>(
            commands,
            entity,
            0,
            0,
            AlignItems::End,
            3,
            1,
            Visibility::Hidden,
            |parent| {
                for i in 0..3 {
                    build_item::<MaterialItem>(parent, 0, 0, i);
                }
            },
        ),
        Err(_) => todo!(),
    }
}

fn interact_item(
    interaction_query: Query<
        (&Interaction, &ItemID),
        (
            (With<ItemNode>, Without<MaterialItem>),
            Changed<Interaction>,
        ),
    >,
    mut details_query: Query<&mut Visibility, With<ItemDetails>>,
    mut query: Query<
        (&mut ItemID, &mut ItemAmount),
        (
            With<MaterialItem>,
            Without<CraftProduct>,
            Without<CraftMaterial>,
        ),
    >,
    product_query: Query<(&Children, &ItemID), With<CraftProduct>>,
    material_query: Query<(&ItemID, &ItemAmount), With<CraftMaterial>>,
) {
    for (interaction, interaction_item_id) in &interaction_query {
        match *interaction {
            Interaction::Pressed => continue,
            Interaction::Hovered => {
                if interaction_item_id.0 == 0 {
                    for mut visibility in &mut details_query {
                        *visibility = Visibility::Hidden;
                    }
                    continue;
                }
                for mut visibility in &mut details_query {
                    *visibility = Visibility::Inherited;
                }
                for (children, product_item_id) in &product_query {
                    if product_item_id.0 != interaction_item_id.0 {
                        continue;
                    }
                    let mut iter = query.iter_mut();
                    for child in children.iter() {
                        match material_query.get(*child) {
                            Ok((material_item_id, material_amount)) => match iter.next() {
                                Some((mut item_id, mut amount)) => {
                                    item_id.0 = material_item_id.0;
                                    amount.0 = material_amount.0;
                                }
                                None => todo!(),
                            },
                            Err(_) => todo!(),
                        }
                    }
                    for (mut item_id, mut amount) in iter {
                        item_id.0 = 0;
                        amount.0 = 0;
                    }
                }
            }
            Interaction::None => continue,
        }
    }
}

fn interact_grid(
    interaction_query: Query<&Interaction, (With<GridNode>, Changed<Interaction>)>,
    mut details_query: Query<&mut Visibility, With<ItemDetails>>,
) {
    for interaction in &interaction_query {
        match *interaction {
            Interaction::Pressed => continue,
            Interaction::Hovered => continue,
            Interaction::None => {
                for mut visibility in &mut details_query {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

fn sync_hidden(
    details_query: Query<&Visibility, (With<ItemDetails>, Changed<Visibility>)>,
    mut query: Query<(&mut ItemID, &mut ItemAmount), With<MaterialItem>>,
) {
    for visibility in &details_query {
        match *visibility {
            Visibility::Inherited => continue,
            Visibility::Hidden => {
                for (mut item_id, mut amount) in &mut query {
                    item_id.0 = 0;
                    amount.0 = 0;
                }
            }
            Visibility::Visible => continue,
        }
    }
}

pub struct ItemDetailsPlugin;

impl Plugin for ItemDetailsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_details);
        app.add_systems(
            Update,
            (
                (interact_item, interact_grid).run_if(in_state(ItemDragged::None)),
                sync_hidden,
            ),
        );
        app.add_systems(
            OnEnter(ItemDragged::Dragged),
            change_visibility::<ItemDetails, ItemDetails, ItemDetails>(Visibility::Hidden),
        );
    }
}
