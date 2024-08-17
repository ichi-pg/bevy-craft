use crate::hotbar::*;
use crate::inventory::*;
use crate::item::*;
use crate::storage::*;
use crate::ui_parts::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ItemNode;

#[derive(Component)]
pub struct ItemIndex(pub u8);

pub fn build_item<T: Component + Default>(
    parent: &mut ChildBuilder,
    item_id: u16,
    amount: u16,
    index: u8,
    selectable: bool,
) -> Entity {
    parent
        .spawn((
            ImageBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    padding: UiRect::all(Val::Px(4.0)),
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
        ))
        .with_children(|parent| {
            if selectable {
                parent.spawn((
                    TextBundle {
                        visibility: Visibility::Hidden,
                        text: Text::from_section("Selected", TextStyle { ..default() }),
                        ..default()
                    },
                    ItemIndex(index),
                ));
            }
            parent.spawn((
                TextBundle::from_section("", TextStyle { ..default() }),
                ItemID(item_id),
                ItemAmount(amount),
            ));
        })
        .id()
    // TODO texture
    // TODO remove selectable
}

fn build_container<T: Component + Default, U: Component + Default>(
    parent: &mut ChildBuilder,
    x: u16,
    y: u16,
    visibility: Visibility,
    selectable: bool,
) {
    parent
        .spawn((grid_node(x, y, visibility), T::default()))
        .with_children(|parent| {
            for i in 0..x * y {
                build_item::<U>(parent, 0, 0, i as u8, selectable);
            }
        });
}

fn spawn_containers(mut commands: Commands) {
    commands
        .spawn(screen_node(10.0, AlignItems::Center))
        .with_children(|parent: &mut ChildBuilder| {
            build_container::<Storage, StorageItem>(parent, 10, 4, Visibility::Hidden, false);
            build_container::<Inventory, InventoryItem>(parent, 10, 4, Visibility::Hidden, false);
            build_container::<Hotbar, HotbarItem>(parent, 10, 1, Visibility::Inherited, true);
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
}

pub struct ItemContainerPlugin;

impl Plugin for ItemContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_containers);
        app.add_systems(Update, sync_children);
    }
    // TODO spawn item when inventory overflowed
}
