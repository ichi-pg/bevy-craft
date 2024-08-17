use crate::craft::*;
use crate::item::*;
use crate::item_container::*;
use crate::ui_parts::*;
use bevy::prelude::*;

#[derive(Component)]
struct ItemDetails;

#[derive(Component, Default)]
struct MaterialItem;

fn spawn_details(mut commands: Commands) {
    commands
        .spawn(screen_node(10.0, AlignItems::End))
        .with_children(|parent: &mut ChildBuilder| {
            parent
                .spawn((grid_node(3, 1, Visibility::Hidden), ItemDetails))
                .with_children(|parent| {
                    for i in 0..3 {
                        build_item::<MaterialItem>(parent, 0, 0, i, false);
                    }
                });
        });
}

fn sync_details(
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
        if interaction_item_id.0 == 0 {
            continue;
        }
        for mut visibility in &mut details_query {
            *visibility = match *interaction {
                Interaction::Pressed => *visibility,
                Interaction::Hovered => Visibility::Inherited,
                Interaction::None => Visibility::Hidden,
            }
        }
        let mut iter = query.iter_mut();
        for (children, product_item_id) in &product_query {
            if product_item_id.0 == interaction_item_id.0 {
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
            }
        }
        for (mut item_id, mut amount) in iter {
            item_id.0 = 0;
            amount.0 = 0;
        }
    }
    // TODO fix blinking
    // TODO hide while dragging
}

pub struct ItemDetailsPlugin;

impl Plugin for ItemDetailsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_details);
        app.add_systems(Update, sync_details);
    }
}
