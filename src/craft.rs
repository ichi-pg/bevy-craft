use crate::input::*;
use crate::inventory::*;
use crate::item::*;
use crate::item_container::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use bevy::prelude::*;

#[derive(Component)]
struct CraftRecipe;

#[derive(Component, Default)]
struct CraftUI;

#[derive(Component, Default)]
struct CraftItem;

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

fn open_craft(key_c: Res<KeyC>, mut next_state: ResMut<NextState<UIStates>>) {
    if !key_c.0 {
        return;
    }
    next_state.set(UIStates::Craft);
}

fn close_craft(key_c: Res<KeyC>, mut next_state: ResMut<NextState<UIStates>>) {
    if !key_c.0 {
        return;
    }
    next_state.set(UIStates::None);
}

fn on_open_craft(mut query: Query<&mut Visibility, Or<(With<CraftUI>, With<Inventory>)>>) {
    for mut visibility in &mut query {
        *visibility = Visibility::Inherited;
    }
}

fn on_close_craft(mut query: Query<&mut Visibility, Or<(With<CraftUI>, With<Inventory>)>>) {
    for mut visibility in &mut query {
        *visibility = Visibility::Hidden;
    }
}

pub struct CraftPlugin;

impl Plugin for CraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_recipes, spawn_nodes).chain());
        app.add_systems(
            Update,
            (
                open_craft.run_if(not(in_state(UIStates::Craft))),
                close_craft.run_if(in_state(UIStates::Craft)),
            ),
        );
        app.add_systems(OnEnter(UIStates::Craft), on_open_craft);
        app.add_systems(OnExit(UIStates::Craft), on_close_craft);
    }
}
