use crate::hotbar::*;
use crate::input::*;
use crate::item::*;
use crate::item_container::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Inventory;

#[derive(Component, Default)]
pub struct InventoryItem;

#[derive(Event, Default)]
pub struct InventoryOverflowed {
    pub item_id: u16,
    pub amount: u16,
}

impl ItemAndAmount for InventoryOverflowed {
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

fn spawn_inventory(mut commands: Commands) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                padding: UiRect::bottom(Val::Px(140.0)),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            build_container::<Inventory, InventoryItem>(parent, 10, 4, Visibility::Hidden);
        });
}

fn toggle_inventory(mut query: Query<&mut Visibility, With<Inventory>>, input: Res<Input>) {
    if !input.tab {
        return;
    }
    for mut visibility in &mut query {
        *visibility = match *visibility {
            Visibility::Inherited => Visibility::Hidden,
            Visibility::Hidden => Visibility::Inherited,
            Visibility::Visible => todo!(),
        };
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InventoryOverflowed>();
        app.add_systems(Startup, spawn_inventory);
        app.add_systems(
            Update,
            (
                put_in_item::<InventoryItem, HotbarOverflowed, InventoryOverflowed>,
                toggle_inventory,
            ),
        );
    }
}
