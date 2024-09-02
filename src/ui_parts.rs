use crate::item_node::*;
use crate::stats::*;
use crate::ui_hovered::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct GridNode;

const MARGIN: u16 = 10;
pub const BACKGROUND_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

pub trait BuildScreen {
    fn build_screen(
        &mut self,
        camera: Entity,
        y: u16,
        grids: u16,
        justify_content: JustifyContent,
        align_items: AlignItems,
        build_children: impl FnOnce(&mut ChildBuilder),
    );
}

impl<'w, 's> BuildScreen for Commands<'w, 's> {
    fn build_screen(
        &mut self,
        camera: Entity,
        y: u16,
        grids: u16,
        justify_content: JustifyContent,
        align_items: AlignItems,
        build_children: impl FnOnce(&mut ChildBuilder),
    ) {
        let padding = Val::Px((MARGIN + grids * MARGIN * 2 + y * MARGIN + y * ITEM_SIZE) as f32);
        self.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content,
                    align_items,
                    row_gap: Val::Px(MARGIN as f32),
                    padding: match justify_content {
                        JustifyContent::Default => todo!(),
                        JustifyContent::Start => UiRect::new(
                            Val::Px(MARGIN as f32),
                            Val::Px(MARGIN as f32),
                            padding,
                            Val::Px(MARGIN as f32),
                        ),
                        JustifyContent::End => UiRect::new(
                            Val::Px(MARGIN as f32),
                            Val::Px(MARGIN as f32),
                            Val::Px(MARGIN as f32),
                            padding,
                        ),
                        JustifyContent::FlexStart => todo!(),
                        JustifyContent::FlexEnd => todo!(),
                        JustifyContent::Center => todo!(),
                        JustifyContent::Stretch => todo!(),
                        JustifyContent::SpaceBetween => todo!(),
                        JustifyContent::SpaceEvenly => todo!(),
                        JustifyContent::SpaceAround => todo!(),
                    },
                    ..default()
                },
                ..default()
            },
            TargetCamera(camera),
        ))
        .with_children(|parent| {
            build_children(parent);
        });
    }
}

pub fn build_space(
    parent: &mut ChildBuilder,
    x: u16,
    y: u16,
    justify_content: JustifyContent,
    build_children: impl FnOnce(&mut ChildBuilder),
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px((x * (ITEM_SIZE + MARGIN) + MARGIN) as f32),
                height: Val::Px((y * (ITEM_SIZE + MARGIN) + MARGIN) as f32),
                flex_direction: FlexDirection::Row,
                justify_content,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            build_children(parent);
        });
}

pub fn build_panel<T: Component + Default>(
    parent: &mut ChildBuilder,
    x: u16,
    y: u16,
    visibility: Visibility,
    build_children: impl FnOnce(&mut ChildBuilder),
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px((x * (ITEM_SIZE + MARGIN) + MARGIN) as f32),
                    height: Val::Px((y * (ITEM_SIZE + MARGIN) + MARGIN) as f32),
                    padding: UiRect::all(Val::Px(MARGIN as f32)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    ..default()
                },
                background_color: BackgroundColor(BACKGROUND_COLOR),
                visibility,
                ..default()
            },
            Interaction::None,
            UI,
            GridNode,
            T::default(),
        ))
        .with_children(|parent| {
            build_children(parent);
        });
}

pub fn build_grid(
    parent: &mut ChildBuilder,
    x: u16,
    y: u16,
    build_children: impl FnOnce(&mut ChildBuilder),
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px((x * ITEM_SIZE + (x - 1) * MARGIN) as f32),
                height: Val::Px((y * ITEM_SIZE + (y - 1) * MARGIN) as f32),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(x, 1.0),
                row_gap: Val::Px(MARGIN as f32),
                column_gap: Val::Px(MARGIN as f32),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            build_children(parent);
        });
}

pub fn build_panel_grid<T: Component + Default>(
    parent: &mut ChildBuilder,
    x: u16,
    y: u16,
    visibility: Visibility,
    build_children: impl FnOnce(&mut ChildBuilder),
) {
    build_panel::<T>(parent, x, y, visibility, |parent| {
        build_grid(parent, x, y, build_children);
    });
}

pub fn build_progress_bar<T: Component + Default>(parent: &mut ChildBuilder, color: Color) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(400.0),
                height: Val::Px(30.0),
                ..default()
            },
            background_color: BackgroundColor(BACKGROUND_COLOR),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: BackgroundColor(color),
                    ..default()
                },
                T::default(),
            ));
        });
}

pub fn build_flex(
    parent: &mut ChildBuilder,
    justify_content: JustifyContent,
    align_items: AlignItems,
    build_children: impl FnOnce(&mut ChildBuilder),
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content,
                align_items,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            build_children(parent);
        });
}

pub fn sync_progress_bar<T: Component + Stats, U: Component + Stats, V: Component, W: Component>(
    player_query: Query<(&T, &U), (With<V>, Changed<T>)>,
    mut query: Query<&mut Style, With<W>>,
) {
    for (health, max_health) in &player_query {
        for mut style in &mut query {
            style.width = Val::Percent(health.get() / max_health.get() * 100.0);
        }
    }
}
