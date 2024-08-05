use crate::item::*;
use bevy::prelude::*;

#[derive(Component)]
struct Hotbar;

#[derive(Component)]
struct HotbarItem;

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
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(1110.0),
                            height: Val::Px(120.0),
                            display: Display::Grid,
                            grid_template_columns: RepeatedGridTrack::flex(10, 1.0),
                            row_gap: Val::Px(10.0),
                            column_gap: Val::Px(10.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                        ..default()
                    },
                    Hotbar,
                ))
                .with_children(|parent| {
                    for _ in 0..10 {
                        parent
                            .spawn(NodeBundle {
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
                            })
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section("", TextStyle { ..default() }),
                                    ItemID(0),
                                    Amount(0),
                                    HotbarItem,
                                ));
                            });
                    }
                });
        });
    // TODO toggle visible
    // TODO texture
    // TODO using generics
}

fn pick_up_item<T: Component>(
    mut query: Query<(&ItemID, &mut Amount), With<T>>,
    mut event_reader: EventReader<ItemPickedUp>,
    mut event_writer: EventWriter<HotbarOverflowed>,
) {
    for event in event_reader.read() {
        // Merge amount
        let mut found = false;
        for (item_id, mut amount) in &mut query {
            if item_id.0 == event.item_id.0 {
                amount.0 += event.amount.0;
                found = true;
                break;
            }
        }
        if found {
            continue;
        }
        // Empty slot
        let mut found = false;
        for (item_id, mut amount) in &mut query {
            if item_id.0 == 0 {
                amount.0 += event.amount.0;
                found = true;
                break;
            }
        }
        if found {
            continue;
        }
        // Overflow
        event_writer.send(HotbarOverflowed);
    }
    // TODO which player?
    // TODO closed chests items is hash map resource?
}

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HotbarOverflowed>();
        app.add_systems(Startup, spawn_hotbar);
        app.add_systems(Update, pick_up_item::<HotbarItem>);
    }
}
