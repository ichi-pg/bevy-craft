use crate::input::*;
use crate::player::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use crate::window::ScreenMode;
use crate::z_sort::*;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy::render::view::*;
use bevy::window::*;

#[derive(Component)]
struct MinimapCamera;

#[derive(Component)]
struct MinimapParent;

pub const MINIMAP_LAYER: RenderLayers = RenderLayers::layer(MINIMAP_ORDER);
pub const MINIMAP_ALPHA: f32 = 0.5;

const MINIMAP_ORDER: usize = 1;
const WORLD_WIDTH: f32 = 1000000.0;
const WORLD_HEIGHT: f32 = 1000000.0;
const FULLMAP_WIDTH: f32 = 1440.0;
const FULLMAP_HEIGHT: f32 = 810.0;
const MINIMAP_WIDTH: f32 = 400.0;
const MINIMAP_HEIGHT: f32 = 225.0;

const INIT_ZOOM: f32 = 10.0;
const ZOOM_RATE: f32 = 1.25;
const MAX_ZOOM_COUNT: f32 = 10.0;
const DRAGGING_RATE: f32 = 1.0;

fn spawn_minimap(mut commands: Commands) {
    commands
        .spawn((SpatialBundle::default(), MinimapParent))
        .with_children(|parent| {
            parent.spawn((
                Camera2dBundle {
                    camera: Camera {
                        viewport: Some(Viewport::default()),
                        order: MINIMAP_ORDER as isize,
                        ..default()
                    },
                    ..default()
                },
                MINIMAP_LAYER,
                MinimapCamera,
            ));
        });
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BACKGROUND_COLOR.with_alpha(MINIMAP_ALPHA),
                custom_size: Some(Vec2::new(WORLD_WIDTH, WORLD_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
            ..default()
        },
        MINIMAP_LAYER,
    ));
    // TODO background trace camera or clear color
}

fn init_zoom(mut query: Query<&mut OrthographicProjection, With<MinimapCamera>>) {
    for mut projection in &mut query {
        projection.scale = INIT_ZOOM;
    }
}

fn drag_fullmap(
    mut query: Query<&mut Transform, With<MinimapParent>>,
    camera_query: Query<&OrthographicProjection, With<MinimapCamera>>,
    left_click: Res<LeftClick>,
    window_cursor: Res<WindowCursor>,
) {
    if !left_click.pressed {
        return;
    }
    for projection in &camera_query {
        for mut transform in &mut query {
            transform.translation.x -= window_cursor.delta.x * projection.scale * DRAGGING_RATE;
            transform.translation.y += window_cursor.delta.y * projection.scale * DRAGGING_RATE;
        }
    }
}

fn zoom_fullmap(
    mut query: Query<&mut OrthographicProjection, With<MinimapCamera>>,
    wheel: Res<Wheel>,
) {
    if wheel.0 == 0 {
        return;
    }
    for mut projection in &mut query {
        projection.scale = if wheel.0 > 0 {
            projection.scale / ZOOM_RATE
        } else {
            projection.scale * ZOOM_RATE
        }
        .clamp(
            INIT_ZOOM / ZOOM_RATE.powf(MAX_ZOOM_COUNT),
            INIT_ZOOM * ZOOM_RATE.powf(MAX_ZOOM_COUNT),
        );
    }
}

fn switch_minimap(
    mut query: Query<&mut Transform, With<MinimapParent>>,
    mut camera_query: Query<(&mut Camera, &mut OrthographicProjection), With<MinimapCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for mut transform in &mut query {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
    }
    for (mut camera, mut projection) in &mut camera_query {
        let Some(viewport) = camera.viewport.as_mut() else {
            continue;
        };
        for window in &window_query {
            let scale = window.scale_factor();
            viewport.physical_position.x =
                ((window.width() - MINIMAP_WIDTH - UI_MARGIN) * scale) as u32;
            viewport.physical_position.y = (UI_MARGIN * scale) as u32;
            viewport.physical_size.x = (MINIMAP_WIDTH * scale) as u32;
            viewport.physical_size.y = (MINIMAP_HEIGHT * scale) as u32;
        }
        projection.scale = INIT_ZOOM;
    }
}

fn switch_fullmap(
    mut query: Query<&mut Transform, With<MinimapParent>>,
    mut camera_query: Query<(&mut Camera, &mut OrthographicProjection), With<MinimapCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for mut transform in &mut query {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
    }
    for (mut camera, mut projection) in &mut camera_query {
        let Some(viewport) = camera.viewport.as_mut() else {
            continue;
        };
        for window in &window_query {
            let scale = window.scale_factor();
            viewport.physical_position.x = ((window.width() - FULLMAP_WIDTH) * 0.5 * scale) as u32;
            viewport.physical_position.y =
                ((window.height() - FULLMAP_HEIGHT) * 0.5 * scale) as u32;
            viewport.physical_size.x = (FULLMAP_WIDTH * scale) as u32;
            viewport.physical_size.y = (FULLMAP_HEIGHT * scale) as u32;
        }
        projection.scale = INIT_ZOOM;
    }
}

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_minimap,
                (switch_minimap, init_zoom).after(spawn_minimap),
            ),
        );
        app.add_systems(
            Update,
            (
                trace_player::<MinimapCamera>,
                change_ui_state::<KeyM>(UIStates::Map).run_if(not(in_state(UIStates::Map))),
                (
                    change_ui_state::<KeyM>(UIStates::None),
                    zoom_fullmap,
                    drag_fullmap,
                )
                    .run_if(in_state(UIStates::Map)),
            ),
        );
        app.add_systems(OnEnter(UIStates::Map), switch_fullmap);
        app.add_systems(OnExit(UIStates::Map), switch_minimap);
        app.add_systems(
            OnEnter(ScreenMode::Windowed),
            (
                switch_fullmap.run_if(in_state(UIStates::Map)),
                switch_minimap.run_if(not(in_state(UIStates::Map))),
            ),
        );
        app.add_systems(
            OnEnter(ScreenMode::Fullscreen),
            (
                switch_fullmap.run_if(in_state(UIStates::Map)),
                switch_minimap.run_if(not(in_state(UIStates::Map))),
            ),
        );
    }
    // TODO fast travel
}
