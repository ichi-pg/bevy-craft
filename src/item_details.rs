use crate::craft_recipe::*;
use crate::item::*;
use crate::item_container::*;
use crate::item_dragging::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use bevy::prelude::*;

#[derive(Component)]
struct ItemDetails;

#[derive(Component, Default)]
struct MaterialItem;

fn spawn_details(mut commands: Commands) {
    commands
        .spawn(screen_node(0, 0, AlignItems::End))
        .with_children(|parent: &mut ChildBuilder| {
            parent
                .spawn(NodeBundle { ..default() })
                .with_children(|parent: &mut ChildBuilder| {
                    parent
                        .spawn((grid_node(3, 1, Visibility::Hidden), ItemDetails))
                        .with_children(|parent| {
                            for i in 0..3 {
                                build_item::<MaterialItem>(parent, 0, 0, i, false);
                            }
                        });
                });
        });
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
