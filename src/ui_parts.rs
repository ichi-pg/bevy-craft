use crate::ui_hovered::*;
use bevy::prelude::*;

pub fn screen_node(padding: f32) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::End,
            align_items: AlignItems::Center,
            row_gap: Val::Px(10.0),
            padding: UiRect::bottom(Val::Px(padding)),
            ..default()
        },
        ..default()
    }
}

pub fn colored_grid<T: Component + Default>(
    x: u16,
    y: u16,
    visibility: Visibility,
) -> (NodeBundle, Interaction, UI, T) {
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
        T::default(),
    )
}
