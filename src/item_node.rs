use crate::atlas::*;
use crate::item::*;
use crate::item_attribute::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ItemNode;

#[derive(Component)]
pub struct ItemSelector;

#[derive(Component)]
pub struct ItemIndex(pub u8);

pub const ITEM_SIZE: f32 = 80.0;

fn item_node<T: Component + Default>(
    item_id: u16,
    amount: u16,
    index: u8,
    attribute: &ItemAttribute,
    atlas: &Atlas,
) -> (
    ImageBundle,
    TextureAtlas,
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
                width: Val::Px(ITEM_SIZE),
                height: Val::Px(ITEM_SIZE),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::End,
                ..default()
            },
            image: UiImage {
                texture: atlas.texture.clone(),
                ..default()
            },
            background_color: BackgroundColor(atlas.color),
            ..default()
        },
        TextureAtlas {
            layout: atlas.layout.clone(),
            index: attribute.atlas_index as usize,
        },
        Interaction::None,
        ItemNode,
        ItemID(item_id),
        ItemAmount(amount),
        ItemIndex(index),
        T::default(),
    )
    // TODO layout of TRANSPARENT_IMAGE_HANDLE
}

fn item_text(item_id: u16, amount: u16) -> (TextBundle, ItemID, ItemAmount) {
    (
        TextBundle::from_section("", TextStyle::default()),
        ItemID(item_id),
        ItemAmount(amount),
    )
}

fn item_selector(
    index: u8,
    atlas: &Atlas,
    atlas_index: u8,
) -> (ImageBundle, TextureAtlas, ItemIndex, ItemSelector) {
    (
        ImageBundle {
            style: Style {
                width: Val::Px(ITEM_SIZE),
                height: Val::Px(ITEM_SIZE),
                position_type: PositionType::Absolute,
                ..default()
            },
            image: UiImage {
                color: Color::srgb(0.9, 0.3, 0.3),
                texture: atlas.texture.clone(),
                ..default()
            },
            background_color: BackgroundColor(atlas.color),
            visibility: Visibility::Hidden,
            ..default()
        },
        TextureAtlas {
            layout: atlas.layout.clone(),
            index: atlas_index as usize,
        },
        ItemIndex(index),
        ItemSelector,
    )
    // TODO colored texture
}

pub fn build_item<T: Component + Default>(
    parent: &mut ChildBuilder,
    item_id: u16,
    amount: u16,
    index: u8,
    attribute: &ItemAttribute,
    atlas: &Atlas,
) {
    parent
        .spawn(item_node::<T>(item_id, amount, index, attribute, atlas))
        .with_children(|parent| {
            parent.spawn(item_text(item_id, amount));
        });
}

pub fn build_hotbar_item<T: Component + Default>(
    parent: &mut ChildBuilder,
    item_id: u16,
    amount: u16,
    index: u8,
    attribute: &ItemAttribute,
    atlas: &Atlas,
    selector_atlas: &Atlas,
    selector_index: u8,
) {
    parent
        .spawn(item_node::<T>(item_id, amount, index, attribute, atlas))
        .with_children(|parent| {
            parent.spawn(item_selector(index, selector_atlas, selector_index));
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

fn sync_image(
    mut query: Query<
        (
            &ItemID,
            &mut BackgroundColor,
            &mut UiImage,
            &mut TextureAtlas,
        ),
        Changed<ItemID>,
    >,
    attribute_map: Res<ItemAttributeMap>,
    atlas_map: Res<AtlasMap>,
) {
    for (item_id, mut color, mut image, mut texture_atlas) in &mut query {
        let Some(attribute) = attribute_map.get(&item_id.0) else {
            continue;
        };
        let Some(atlas) = atlas_map.get(&attribute.atlas_id) else {
            continue;
        };
        color.0 = atlas.color;
        image.texture = atlas.texture.clone();
        texture_atlas.layout = atlas.layout.clone();
        texture_atlas.index = attribute.atlas_index as usize;
    }
}

pub struct ItemNodePlugin;

impl Plugin for ItemNodePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (sync_children, sync_text, sync_image));
    }
    // TODO spawn item when inventory overflowed
}
