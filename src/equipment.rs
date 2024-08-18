use crate::item_container::*;
use crate::ui_parts::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct EquipmentUI;

#[derive(Component, Default)]
pub struct EquipmentItem;

fn spawn_equipments(mut commands: Commands) {
    commands
        .spawn(screen_node(4, 2, AlignItems::Center))
        .with_children(|parent: &mut ChildBuilder| {
            parent
                .spawn(grid_space(10, 2, JustifyContent::End))
                .with_children(|parent| {
                    parent
                        .spawn((grid_node(3, 2, Visibility::Hidden), EquipmentUI))
                        .with_children(|parent| {
                            for i in 0..6 {
                                build_item::<EquipmentItem>(parent, 0, 0, i, false);
                            }
                        });
                });
        });
    // TODO character preview
    // TODO slot categorize
}

pub struct EquipmentPlugin;

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_equipments);
    }
    // TODO stats
    // TODO ability
}
