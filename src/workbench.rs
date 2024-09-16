use crate::camera::*;
use crate::inventory::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct WorkbenchUI;

#[derive(Event)]
pub struct WorkbenchClicked;

fn spawn_items(camera_query: Query<Entity, With<PlayerCamera>>, mut commands: Commands) {
    for entity in &camera_query {
        commands.build_screen(
            entity,
            INVENTORY_Y + 1,
            2,
            JustifyContent::End,
            AlignItems::Center,
            |parent| {
                build_space(
                    parent,
                    INVENTORY_X,
                    2,
                    JustifyContent::SpaceBetween,
                    |parent| {
                        for _ in 0..3 {
                            build_panel_grid::<WorkbenchUI>(
                                parent,
                                3,
                                2,
                                Visibility::Hidden,
                                |_| {},
                            );
                        }
                    },
                );
            },
        );
    }
}

fn open_workbench(
    event_reader: EventReader<WorkbenchClicked>,
    mut next_state: ResMut<NextState<UIStates>>,
) {
    if event_reader.is_empty() {
        return;
    }
    next_state.set(UIStates::Workbench);
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
