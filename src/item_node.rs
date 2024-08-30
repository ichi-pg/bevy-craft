use crate::item::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ItemNode;

#[derive(Component)]
pub struct ItemSelector;

#[derive(Component)]
pub struct ItemIndex(pub u8);

pub trait NodeItem {
    fn selectable(&self) -> bool;
}

pub const ITEM_SIZE: u16 = 80;

fn item_node<T: Component + Default + NodeItem>(
    item_id: u16,
    amount: u16,
    index: u8,
) -> (
    ImageBundle,
    Interaction,
    ItemNode,
    ItemID,
    ItemAmount,
    ItemIndex,
    T,
) {
    (
        ImageBundle {
            style: Style {
                width: Val::Px(ITEM_SIZE as f32),
                height: Val::Px(ITEM_SIZE as f32),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
        },
        Interaction::None,
        ItemNode,
        ItemID(item_id),
        ItemAmount(amount),
        ItemIndex(index),
        T::default(),
    )
    // TODO texture
}

fn item_text(item_id: u16, amount: u16) -> (TextBundle, ItemID, ItemAmount) {
    (
        TextBundle::from_section("", TextStyle { ..default() }),
        ItemID(item_id),
        ItemAmount(amount),
    )
}

fn item_selected(index: u8) -> (TextBundle, ItemIndex, ItemSelector) {
    (
        TextBundle {
            visibility: Visibility::Hidden,
            text: Text::from_section("Selected", TextStyle { ..default() }),
            ..default()
        },
        ItemIndex(index),
        ItemSelector,
    )
}

pub fn build_item<T: Component + Default + NodeItem>(
    parent: &mut ChildBuilder,
    item_id: u16,
    amount: u16,
    index: u8,
) {
    parent
        .spawn(item_node::<T>(item_id, amount, index))
        .with_children(|parent| {
            if T::default().selectable() {
                parent.spawn(item_selected(index));
            }
            parent.spawn(item_text(item_id, amount));
        });
}

fn sync_children(
    parent_query: Query<
        (&Children, &ItemID, &ItemAmount),
        (With<ItemNode>, Or<(Changed<ItemID>, Changed<ItemAmount>)>),
    >,
    mut child_query: Query<(&mut ItemID, &mut ItemAmount), (With<Node>, Without<ItemNode>)>,
) {
    for (children, parent_item_id, parent_amount) in &parent_query {
        for child in children.iter() {
            match child_query.get_mut(*child) {
                Ok((mut child_item_id, mut child_amount)) => {
                    child_item_id.0 = parent_item_id.0;
                    child_amount.0 = parent_amount.0;
                }
                Err(_) => continue,
            }
        }
    }
    // TODO with children, with parent?
}

fn sync_text(mut query: Query<(&ItemID, &ItemAmount, &mut Text), Changed<ItemAmount>>) {
    for (item_id, amount, mut text) in &mut query {
        for section in &mut text.sections {
            section.value = if amount.0 == 0 {
                String::new()
            } else {
                format!("{} x{}", item_id.0, amount.0)
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

pub struct ItemNodePlugin;

impl Plugin for ItemNodePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (sync_children, sync_text, sync_image));
    }
    // TODO spawn item when inventory overflowed
}
