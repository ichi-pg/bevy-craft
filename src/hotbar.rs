use crate::item::*;
use crate::item_container::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Hotbar;

#[derive(Component, Default)]
pub struct HotbarItem;

#[derive(Event, Default)]
pub struct HotbarOverflowed {
    pub item_id: u16,
    pub amount: u16,
}

impl ItemAndAmount for HotbarOverflowed {
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

fn spawn_hotbar(mut commands: Commands) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                padding: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            build_container::<Hotbar, HotbarItem>(parent, 10, 1, Visibility::Inherited);
        });
}

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HotbarOverflowed>();
        app.add_systems(Startup, spawn_hotbar);
        app.add_systems(
            Update,
            put_in_item::<HotbarItem, ItemPickedUp, HotbarOverflowed>,
        );
    }
}
