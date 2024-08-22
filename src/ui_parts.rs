use crate::item_node::*;
use crate::ui_hovered::*;
use bevy::prelude::*;
use std::slice::*;

#[derive(Component)]
pub struct GridNode;

const MARGIN: u16 = 10;
pub const BACKGROUND_COLOR: BackgroundColor = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));

fn screen_node(
    camera: Entity,
    y: u16,
    grids: u16,
    align_items: AlignItems,
) -> (NodeBundle, TargetCamera) {
    (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items,
                row_gap: Val::Px(MARGIN as f32),
                padding: UiRect::new(
                    Val::Px(MARGIN as f32),
                    Val::Px(MARGIN as f32),
                    Val::Px(MARGIN as f32),
                    Val::Px((MARGIN + grids * MARGIN * 2 + y * MARGIN + y * ITEM_SIZE) as f32),
                ),
                ..default()
            },
            ..default()
        },
        TargetCamera(camera),
    )
}

fn grid_space(x: u16, y: u16, justify_content: JustifyContent) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Px((x * (ITEM_SIZE + MARGIN) + MARGIN) as f32),
            height: Val::Px((y * (ITEM_SIZE + MARGIN) + MARGIN) as f32),
            flex_direction: FlexDirection::Row,
            justify_content,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}

fn grid_node(x: u16, y: u16, visibility: Visibility) -> (NodeBundle, Interaction, UI, GridNode) {
    (
        NodeBundle {
            style: Style {
                width: Val::Px((x * (ITEM_SIZE + MARGIN) + MARGIN) as f32),
                height: Val::Px((y * (ITEM_SIZE + MARGIN) + MARGIN) as f32),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(x, 1.0),
                row_gap: Val::Px(MARGIN as f32),
                column_gap: Val::Px(MARGIN as f32),
                padding: UiRect::all(Val::Px(MARGIN as f32)),
                ..default()
            },
            background_color: BACKGROUND_COLOR,
            visibility,
            ..default()
        },
        Interaction::None,
        UI,
        GridNode,
    )
    // TODO split background and grid
}

pub fn build_grid<T: Component + Default>(
    mut commands: Commands,
    camera: Entity,
    margin_x: u16,
    margin_y: u16,
    align_items: AlignItems,
    size_x: u16,
    size_y: u16,
    visibility: Visibility,
    with_children: impl FnOnce(&mut ChildBuilder),
) {
    commands
        .spawn(screen_node(camera, margin_x, margin_y, align_items))
        .with_children(|parent| {
            parent
                .spawn((grid_node(size_x, size_y, visibility), T::default()))
                .with_children(with_children);
        });
}

pub fn build_spaced<T: Component + Default>(
    mut commands: Commands,
    camera: Entity,
    margin_y: u16,
    margin_grids: u16,
    align_items: AlignItems,
    space_x: u16,
    space_y: u16,
    justify_content: JustifyContent,
    size_x: u16,
    size_y: u16,
    visibility: Visibility,
    with_children: impl FnOnce(&mut ChildBuilder),
) {
    commands
        .spawn(screen_node(camera, margin_y, margin_grids, align_items))
        .with_children(|parent| {
            parent
                .spawn(grid_space(space_x, space_y, justify_content))
                .with_children(|parent| {
                    parent
                        .spawn((grid_node(size_x, size_y, visibility), T::default()))
                        .with_children(with_children);
                });
        });
}

pub fn build_iter<T: Component + Default, U>(
    mut commands: Commands,
    camera: Entity,
    margin_y: u16,
    margin_grids: u16,
    align_items: AlignItems,
    space_x: u16,
    space_y: u16,
    justify_content: JustifyContent,
    iter: Iter<'_, U>,
    size_x: u16,
    size_y: u16,
    visibility: Visibility,
    with_children: impl Fn(&mut ChildBuilder, &U),
) {
    commands
        .spawn(screen_node(camera, margin_y, margin_grids, align_items))
        .with_children(|parent| {
            parent
                .spawn(grid_space(space_x, space_y, justify_content))
                .with_children(|parent| {
                    for i in iter {
                        parent
                            .spawn((grid_node(size_x, size_y, visibility), T::default()))
                            .with_children(|parent| {
                                with_children(parent, i);
                            });
                    }
                });
        });
}
