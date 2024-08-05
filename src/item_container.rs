use crate::item::*;
use bevy::prelude::*;

pub fn build_container<T: Component + Default, U: Component + Default>(
    parent: &mut ChildBuilder,
    x: u16,
    y: u16,
    visibility: Visibility,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px((x * 110 + 10) as f32),
                    height: Val::Px((y * 110 + 10) as f32),
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(x, 1.0),
                    row_gap: Val::Px(10.0),
                    column_gap: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                visibility,
                ..default()
            },
            T::default(),
        ))
        .with_children(|parent| {
            for _ in 0..x * y {
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
                            U::default(),
                        ));
                    });
            }
        });
    // TODO texture
}

pub fn put_in_item<T: Component, U: Event + ItemAndAmount, V: Event + Default + ItemAndAmount>(
    mut query: Query<(&ItemID, &mut Amount), With<T>>,
    mut event_reader: EventReader<U>,
    mut event_writer: EventWriter<V>,
) {
    for event in event_reader.read() {
        // Merge amount
        let mut found = false;
        for (item_id, mut amount) in &mut query {
            if item_id.0 == event.item_id() {
                amount.0 += event.amount();
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
                amount.0 += event.amount();
                found = true;
                break;
            }
        }
        if found {
            continue;
        }
        // Overflow
        let mut v: V = V::default();
        v.set_item_id(event.item_id());
        v.set_amount(event.amount());
        event_writer.send(v);
    }
    // TODO which player?
    // TODO closed chests items is hash map resource?
}
