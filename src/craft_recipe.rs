use crate::item::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct CraftProduct;

#[derive(Component)]
pub struct CraftMaterial;

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

pub struct CraftRecipePlugin;

impl Plugin for CraftRecipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_recipes);
    }
}
