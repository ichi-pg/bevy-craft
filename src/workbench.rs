use crate::craft::*;
use crate::craft_recipe::*;
use crate::inventory::*;
use crate::item::*;
use crate::item_node::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use bevy::prelude::*;
use std::collections::*;

#[derive(Component, Default)]
pub struct WorkbenchUI;

#[derive(Event)]
pub struct WorkbenchClicked;

fn spawn_items(query: Query<(&ItemID, &ItemAmount), With<CraftProduct>>, mut commands: Commands) {
    commands
        .spawn(screen_node(INVENTORY_Y + 1, 2, AlignItems::Center))
        .with_children(|parent: &mut ChildBuilder| {
            parent
                .spawn(grid_space(INVENTORY_X, 2, JustifyContent::SpaceBetween))
                .with_children(|parent| {
                    for item_ids in [
                        HashSet::<u16>::from_iter([101, 102, 103]),
                        HashSet::<u16>::from_iter([]),
                        HashSet::<u16>::from_iter([]),
                    ] {
                        parent
                            .spawn((grid_node(3, 2, Visibility::Hidden), WorkbenchUI))
                            .with_children(|parent| {
                                for (index, (item_id, amount)) in query.iter().enumerate() {
                                    if !item_ids.contains(&item_id.0) {
                                        continue;
                                    }
                                    build_item::<ProductItem>(
                                        parent,
                                        item_id.0,
                                        amount.0,
                                        index as u8,
                                    );
                                }
                            });
                    }
                });
        });
}

fn open_workbench(
    mut event_reader: EventReader<WorkbenchClicked>,
    mut next_state: ResMut<NextState<UIStates>>,
) {
    for _ in event_reader.read() {
        next_state.set(UIStates::Workbench);
    }
    // TODO replace item_id and amount by workbench id
    // TODO enable distance
}

pub struct WorkbenchPlugin;

impl Plugin for WorkbenchPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorkbenchClicked>();
        app.add_systems(Startup, spawn_items);
        app.add_systems(Update, open_workbench);
        app.add_systems(
            OnEnter(UIStates::Workbench),
            change_visibility::<WorkbenchUI, Inventory, Inventory>(Visibility::Inherited),
        );
        app.add_systems(
            OnExit(UIStates::Workbench),
            change_visibility::<WorkbenchUI, Inventory, Inventory>(Visibility::Hidden),
        );
    }
}
