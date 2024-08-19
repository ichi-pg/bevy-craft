use crate::item_node::*;
use crate::ui_hovered::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct GridNode;

const MARGIN: u16 = 10;

pub fn screen_node(y: u16, containers: u16, align_items: AlignItems) -> NodeBundle {
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
                Val::Px((MARGIN + containers * MARGIN * 2 + y * MARGIN + y * ITEM_SIZE) as f32),
            ),
            ..default()
        },
        ..default()
    }
    // TODO ui root?
}

pub fn grid_space(x: u16, y: u16, justify_content: JustifyContent) -> NodeBundle {
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

pub fn grid_node(
    x: u16,
    y: u16,
    visibility: Visibility,
) -> (NodeBundle, Interaction, UI, GridNode) {
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
            background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            visibility,
            ..default()
        },
        Interaction::None,
        UI,
        GridNode,
    )
    // TODO split background and grid
}
