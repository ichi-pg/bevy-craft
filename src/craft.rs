use crate::input::*;
use crate::item::*;
use crate::item_container::*;
use crate::ui_parts::*;
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

fn spawn_nodes(mut commands: Commands) {
    commands
        .spawn(screen_node(600.0))
        .with_children(|parent: &mut ChildBuilder| {
            parent
                .spawn(colored_grid::<CraftUI>(10, 4, Visibility::Hidden))
                .with_children(|parent| {
                    build_item::<CraftItem>(parent, 101, 1, 0, false);
                    build_item::<CraftItem>(parent, 102, 1, 1, false);
                    build_item::<CraftItem>(parent, 103, 1, 2, false);
                });
        });
}

fn open_recipes(
    mut query: Query<&mut Visibility, With<CraftUI>>,
    input: Res<Input>,
) {
    if !input.v {
        return;
    }
    for mut visibility in &mut query {
        *visibility = match *visibility {
            Visibility::Inherited => Visibility::Hidden,
            Visibility::Hidden => Visibility::Inherited,
            Visibility::Visible => todo!(),
        }
    }
}

fn close_recipes() {}

pub struct CraftPlugin;

impl Plugin for CraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_recipes, spawn_nodes));
        app.add_systems(Update, (open_recipes, close_recipes));
    }
}
