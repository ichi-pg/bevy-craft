use crate::item::*;
use bevy::prelude::*;

#[derive(Component)]
struct Hotbar;

fn spawn_hotbar(mut commands: Commands) {
    commands.spawn((NodeBundle { ..default() }, Hotbar));
    // TODO toggle visible
    // TODO layout
}

fn spawn_item(
    mut query: Query<Entity, With<Hotbar>>,
    mut event_reader: EventReader<ItemPickedUp>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        for entity in &mut query {
            commands.entity(entity).with_children(|parent| {
                parent.spawn((NodeBundle { ..default() }, event.item_id, event.amount));
            });
        }
    }
    // TODO texture
    // TODO merge amount
}

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hotbar);
        app.add_systems(Update, spawn_item);
    }
}
