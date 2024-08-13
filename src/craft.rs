use bevy::prelude::*;

use crate::item::{ItemAmount, ItemID};

#[derive(Component)]
struct CraftRecipe;

fn spawn_recipes(mut commands: Commands) {
    for i in [
        ((101, 1), vec![(2, 1), (3, 1)]),
        ((102, 1), vec![(101, 1), (4, 1)]),
        ((103, 1), vec![(101, 1), (5, 1), (6, 1)]),
    ] {
        commands
            .spawn((CraftRecipe, ItemID(i.0 .0), ItemAmount(i.0 .1)))
            .with_children(|parent| {
                for j in i.1 {
                    parent.spawn((ItemID(j.0), ItemAmount(j.1)));
                }
            });
    }
}

pub struct CraftPlugin;

impl Plugin for CraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_recipes);
    }
}
