use crate::ui_hovered::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct GridNode;

pub fn screen_node(padding: f32, align_items: AlignItems) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::End,
            align_items,
            row_gap: Val::Px(10.0),
            padding: UiRect::new(
                Val::Px(10.0),
                Val::Px(10.0),
                Val::Px(10.0),
                Val::Px(padding),
            ),
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
                width: Val::Px((x * 110 + 10) as f32),
                height: Val::Px((y * 110 + 10) as f32),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(x, 1.0),
                row_gap: Val::Px(10.0),
                column_gap: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
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
