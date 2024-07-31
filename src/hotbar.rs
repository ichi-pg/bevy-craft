use crate::item::*;
use bevy::prelude::*;

#[derive(Component)]
struct Hotbar;

fn spawn_hotbar(mut commands: Commands) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::End,
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(1110.0),
                                height: Val::Px(120.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Start,
                                column_gap: Val::Px(10.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                            ..default()
                        },
                        Hotbar,
                    ));
                });
        });
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
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.5)),
                        ..default()
                    },
                    event.item_id,
                    event.amount,
                ));
            });
        }
    }
    // FIXME double spawn
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
