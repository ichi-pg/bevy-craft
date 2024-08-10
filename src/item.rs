use crate::collision::*;
use crate::hit_test::*;
use crate::velocity::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Item;

#[derive(Component, Clone, Copy)]
pub struct ItemID(pub u16);

#[derive(Component, Clone, Copy)]
pub struct ItemAmount(pub u16);

#[derive(Event)]
pub struct ItemDropped {
    pub translation: Vec3,
    pub item_id: u16,
    pub amount: u16,
}

pub trait ItemAndAmount {
    fn item_id(&self) -> u16;
    fn amount(&self) -> u16;
    fn set_item_id(&mut self, item_id: u16);
    fn set_amount(&mut self, item_id: u16);
}

#[derive(Event)]
pub struct ItemPickedUp {
    pub item_id: u16,
    pub amount: u16,
}

impl ItemAndAmount for ItemPickedUp {
    fn item_id(&self) -> u16 {
        self.item_id
    }
    fn amount(&self) -> u16 {
        self.amount
    }
    fn set_item_id(&mut self, item_id: u16) {
        self.item_id = item_id;
    }
    fn set_amount(&mut self, amount: u16) {
        self.amount = amount;
    }
}

pub fn item_color(item_id: u16) -> Color {
    match item_id {
        0 => Color::srgb(0.1, 0.1, 0.1),
        1 => Color::srgb(0.5, 0.4, 0.3),
        _ => {
            let a = item_id as f32 * 0.02 + 0.3;
            let b = a * 0.5;
            match item_id % 3 {
                0 => Color::srgb(a, a, a),
                _ => Color::srgb(b, a, b),
            }
        }
    }
}

fn spawn_item(mut event_reader: EventReader<ItemDropped>, mut commands: Commands) {
    for event in event_reader.read() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: item_color(event.item_id),
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                transform: Transform::from_translation(event.translation),
                ..default()
            },
            Shape::Circle(32.0),
            Velocity2::default(),
            Item,
            ItemID(event.item_id),
            ItemAmount(event.amount),
        ));
    }
    // TODO texture
}

fn pick_up_item(
    query: Query<(Entity, &ItemID, &ItemAmount), With<Collided>>,
    mut event_writer: EventWriter<ItemPickedUp>,
    mut commands: Commands,
) {
    for (entity, item_id, amount) in &query {
        commands.entity(entity).despawn();
        event_writer.send(ItemPickedUp {
            item_id: item_id.0,
            amount: amount.0,
        });
    }
}

fn sync_text(mut query: Query<(&ItemAmount, &mut Text), Changed<ItemAmount>>) {
    for (amount, mut text) in &mut query {
        for section in &mut text.sections {
            section.value = if amount.0 == 0 {
                String::new()
            } else {
                format!("{}", amount.0)
            };
        }
    }
}

fn sync_image(mut query: Query<(&ItemID, &mut BackgroundColor), (With<UiImage>, Changed<ItemID>)>) {
    for (item_id, mut color) in &mut query {
        color.0 = item_color(item_id.0);
    }
    // TODO texture
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemDropped>();
        app.add_event::<ItemPickedUp>();
        app.add_systems(Update, (spawn_item, sync_text, sync_image));
        app.add_systems(PostUpdate, pick_up_item);
    }
}
