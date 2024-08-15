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

fn spawn_nodes(query: Query<(&ItemID, &ItemAmount), With<CraftRecipe>>, mut commands: Commands) {
    commands
        .spawn(screen_node(600.0))
        .with_children(|parent: &mut ChildBuilder| {
            parent
                .spawn(colored_grid::<CraftUI>(10, 4, Visibility::Hidden))
                .with_children(|parent| {
                    let mut index = 0;
                    for (item_id, amount) in &query {
                        build_item::<CraftItem>(parent, item_id.0, amount.0, index, false);
                        index += 1;
                    }
                });
        });
}

fn toggle_recipes(mut query: Query<&mut Visibility, With<CraftUI>>, input: Res<Input>) {
    if input.c {
        for mut visibility in &mut query {
            *visibility = match *visibility {
                Visibility::Inherited => Visibility::Hidden,
                Visibility::Hidden => Visibility::Inherited,
                Visibility::Visible => todo!(),
            }
        }
    } else if input.escape {
        for mut visibility in &mut query {
            *visibility = Visibility::Hidden;
        }
    }
}

fn close_recipes(mut storage_query: Query<&mut Visibility, With<CraftUI>>, input: Res<Input>) {
    if !input.tab {
        return;
    }
    for mut visibility in &mut storage_query {
        *visibility = Visibility::Hidden;
    }
}

pub struct CraftPlugin;

impl Plugin for CraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_recipes, spawn_nodes).chain());
        app.add_systems(Update, (toggle_recipes, close_recipes));
    }
}
