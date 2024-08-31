use bevy::prelude::*;
use std::collections::HashMap;

pub struct CraftMaterial {
    pub item_id: u16,
    pub amount: u16,
}

pub struct CraftRecipe {
    pub amount: u16,
    pub materials: Vec<CraftMaterial>,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CraftRecipeMap(HashMap<u16, CraftRecipe>);

fn spawn_recipes(mut recipe_map: ResMut<CraftRecipeMap>) {
    for item in [
        (101, 1, vec![(1, 1)]),
        (102, 1, vec![(2, 1)]),
        (103, 1, vec![(3, 1)]),
        (104, 1, vec![(4, 1)]),
    ] {
        recipe_map.insert(
            item.0,
            CraftRecipe {
                amount: item.1,
                materials: item
                    .2
                    .iter()
                    .map(|(item_id, amount)| CraftMaterial {
                        item_id: *item_id,
                        amount: *amount,
                    })
                    .collect(),
            },
        );
    }
    // TODO timber
    // TODO flint
    // TODO coal
    // TODO charcoal
    // TODO copper
    // TODO iron
    // TODO silver
    // TODO gold
    // TODO crystal
    // TODO machine parts

    // TODO pickaxe
    // TODO sword
    // TODO bow
}

pub struct CraftRecipePlugin;

impl Plugin for CraftRecipePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CraftRecipeMap::default());
        app.add_systems(PreStartup, spawn_recipes);
    }
    // TODO using after?
}
