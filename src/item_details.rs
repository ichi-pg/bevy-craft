use crate::atlas::*;
use crate::camera::*;
use crate::craft_recipe::*;
use crate::item::*;
use crate::item_attribute::*;
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

fn spawn_details(
    camera_query: Query<Entity, With<PlayerCamera>>,
    mut commands: Commands,
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
            0,
            0,
            JustifyContent::End,
            AlignItems::End,
            |parent| {
                build_grid::<ItemDetails>(parent, 3, 1, Visibility::Hidden, |parent| {
                    for i in 0..3 {
                        build_item::<MaterialItem>(parent, 0, 0, i, attribute, atlas);
                    }
                });
            },
        );
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
    mut query: Query<(&mut ItemID, &mut ItemAmount), With<MaterialItem>>,
    recipe_map: Res<CraftRecipeMap>,
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
                match recipe_map.get(&interaction_item_id.0) {
                    Some(recipe) => {
                        let mut iter = query.iter_mut();
                        for material in &recipe.materials {
                            match iter.next() {
                                Some((mut item_id, mut amount)) => {
                                    item_id.0 = material.item_id;
                                    amount.0 = material.amount;
                                }
                                None => todo!(),
                            }
                        }
                        for (mut item_id, mut amount) in iter {
                            item_id.0 = 0;
                            amount.0 = 0;
                        }
                    }
                    None => continue,
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
