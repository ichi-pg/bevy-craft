use crate::item_id::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct CraftMaterial {
    pub item_id: u16,
    pub amount: u16,
}

pub struct CraftRecipe {
    pub amount: u16,
    pub materials: Vec<CraftMaterial>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct CraftRecipeMap(HashMap<u16, CraftRecipe>);

fn create_recipes() -> CraftRecipeMap {
    let mut recipes = HashMap::new();
    for (item_id, amount, materials) in [
        (WOOD_PICKAXE_ITEM_ID, 1, vec![(WOOD_ITEM_ID, 1)]),
        (WOOD_SWORD_ITEM_ID, 1, vec![(WOOD_ITEM_ID, 1)]),
        (STORAGE_ITEM_ID, 1, vec![(WOOD_ITEM_ID, 1)]),
        (WORKBENCH_ITEM_ID, 1, vec![(WOOD_ITEM_ID, 1)]),
    ] {
        recipes.insert(
            item_id,
            CraftRecipe {
                amount,
                materials: materials
                    .iter()
                    .map(|(item_id, amount)| CraftMaterial {
                        item_id: *item_id,
                        amount: *amount,
                    })
                    .collect(),
            },
        );
    }
    CraftRecipeMap(recipes)
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
        app.insert_resource(create_recipes());
    }
}
