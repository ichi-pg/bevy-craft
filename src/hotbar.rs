use crate::item::*;
use bevy::prelude::*;

#[derive(Component)]
struct Hotbar;

#[derive(Component)]
struct HotbarItem;

#[derive(Component)]
struct MaxCount(u8);

#[derive(Event)]
struct HotbarOverflowed;

fn spawn_hotbar(mut commands: Commands) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
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
                MaxCount(10),
            ));
        });
    // TODO toggle visible
}

fn picked_up_item(
    mut hotbar_query: Query<(Entity, Option<&Children>, &MaxCount), With<Hotbar>>,
    mut item_query: Query<(&ItemID, &mut Amount), With<HotbarItem>>,
    mut event_reader: EventReader<ItemPickedUp>,
    mut event_writer: EventWriter<HotbarOverflowed>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        for (entity, children, max_count) in &mut hotbar_query {
            let mut found = false;
            for (item_id, mut amount) in &mut item_query {
                if item_id.0 == event.item_id.0 {
                    amount.0 += event.amount.0;
                    found = true;
                    break;
                }
            }
            if found {
                continue;
            }
            match children {
                Some(v) => {
                    if v.len() >= max_count.0 as usize {
                        event_writer.send(HotbarOverflowed);
                        continue;
                    }
                }
                None => {}
            }
            commands.entity(entity).with_children(|parent| {
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(100.0),
                                height: Val::Px(100.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::End,
                                align_items: AlignItems::End,
                                padding: UiRect::all(Val::Px(4.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.5)),
                            ..default()
                        },
                        event.item_id,
                        event.amount,
                        HotbarItem,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("{}", event.amount.0),
                            TextStyle { ..default() },
                        ));
                    });
            });
        }
    }
    // FIXME double spawn
    // FIXME same time merge
    // FIXME same time overflow
    // TODO texture
    // TODO commonalize hotbar, inventory, and chest
}

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HotbarOverflowed>();
        app.add_systems(Startup, spawn_hotbar);
        app.add_systems(Update, picked_up_item);
    }
}
