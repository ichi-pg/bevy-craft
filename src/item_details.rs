use crate::atlas::*;
use crate::camera::*;
use crate::craft_recipe::*;
use crate::item::*;
use crate::item_attribute::*;
use crate::item_dragging::*;
use crate::item_node::*;
use crate::localization::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use bevy::prelude::*;

#[derive(Component, Default)]
struct ItemDetails;

#[derive(Component, Default)]
struct MaterialItem;

#[derive(Component)]
struct ItemName;

impl ItemText for ItemName {
    fn local_text_id(attribute: &ItemAttribute) -> u16 {
        attribute.name_id
    }
}

#[derive(Component)]
struct ItemDescription;

impl ItemText for ItemDescription {
    fn local_text_id(attribute: &ItemAttribute) -> u16 {
        attribute.description_id
    }
}

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
                build_panel::<ItemDetails>(parent, 3, 2, Visibility::Hidden, |parent| {
                    parent.spawn((
                        TextBundle::from_section("Item Name", TextStyle::default()),
                        ItemName,
                        ItemID(0),
                    ));
                    parent.spawn((
                        TextBundle::from_section("Item Description", TextStyle::default()),
                        ItemDescription,
                        ItemID(0),
                    ));
                    build_flex(parent, JustifyContent::End, AlignItems::Start, |parent| {
                        build_grid(parent, 3, 1, |parent| {
                            for i in 0..3 {
                                build_item::<MaterialItem>(parent, 0, 0, i, attribute, atlas);
                            }
                        });
                    });
                });
            },
        );
    }
}

fn interact_item(
    interaction_query: Query<
        (&Interaction, &ItemID),
        (With<ItemNode>, Without<MaterialItem>, Changed<Interaction>),
    >,
    mut details_query: Query<&mut Visibility, With<ItemDetails>>,
    mut material_query: Query<(&mut ItemID, &mut ItemAmount), With<MaterialItem>>,
    mut text_query: Query<
        &mut ItemID,
        (
            Or<(With<ItemName>, With<ItemDescription>)>,
            Without<ItemNode>,
            Without<MaterialItem>,
        ),
    >,
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
                for mut item_id in &mut text_query {
                    item_id.0 = interaction_item_id.0;
                }
                match recipe_map.get(&interaction_item_id.0) {
                    Some(recipe) => {
                        let mut iter = material_query.iter_mut();
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

fn sync_text<T: Component + ItemText>(
    mut query: Query<(&ItemID, &mut Text), (With<T>, Changed<ItemID>)>,
    attribute_map: Res<ItemAttributeMap>,
    text_map: Res<LocalTextMap>,
) {
    for (item_id, mut text) in &mut query {
        let Some(attribute) = attribute_map.get(&item_id.0) else {
            return;
        };
        let Some(local_text) = text_map.get(&T::local_text_id(attribute)) else {
            return;
        };
        for section in &mut text.sections {
            section.value = format!("{}", local_text.text);
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
                sync_text::<ItemName>,
                sync_text::<ItemDescription>,
            ),
        );
        app.add_systems(
            OnEnter(ItemDragged::Dragged),
            change_visibility::<ItemDetails, ItemDetails, ItemDetails>(Visibility::Hidden),
        );
    }
}
