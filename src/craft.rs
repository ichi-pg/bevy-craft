use crate::item::*;
use bevy::prelude::*;

#[derive(Component)]
struct CraftRecipe;

#[derive(Component, Default)]
pub struct CraftUI;

#[derive(Component, Default)]
pub struct CraftItem;

fn spawn_recipes(mut commands: Commands) {
    for i in [
        ((101, 1), vec![(11, 1), (12, 1)]),
        ((102, 1), vec![(101, 1), (13, 1)]),
        ((103, 1), vec![(101, 1), (14, 1), (15, 1)]),
    ] {
        commands
            .spawn((CraftRecipe, ItemID(i.0 .0), ItemAmount(i.0 .1)))
            .with_children(|parent| {
                for j in i.1 {
                    parent.spawn((ItemID(j.0), ItemAmount(j.1)));
                }
            });
    }
    // TODO workbench
}

fn spawn_nodes() {}

fn open_recipes() {}

fn close_recipes() {}

pub struct CraftPlugin;

impl Plugin for CraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_recipes, spawn_nodes));
        app.add_systems(Update, (open_recipes, close_recipes));
    }
}
