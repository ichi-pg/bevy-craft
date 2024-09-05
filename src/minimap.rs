use crate::input::*;
use crate::player::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use crate::z_sort::*;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy::render::view::*;
use bevy::window::*;

#[derive(Component)]
struct MinimapCamera;

pub const MINIMAP_LAYER: RenderLayers = RenderLayers::layer(MINIMAP_ORDER);
pub const MINIMAP_ALPHA: f32 = 0.5;

const MINIMAP_ORDER: usize = 1;
const WORLD_WIDTH: f32 = 1000000.0;
const WORLD_HEIGHT: f32 = 1000000.0;
const MAP_WIDTH: u32 = 1600;
const MAP_HEIGHT: u32 = 900;
const MINIMAP_WIDTH: u32 = MAP_WIDTH / 4;
const MINIMAP_HEIGHT: u32 = MAP_HEIGHT / 4;

const INIT_ZOOM: f32 = 10.0;
const ZOOM_RATE: f32 = 1.25;
const MAX_ZOOM_COUNT: f32 = 10.0;
const DRAGGING_RATE: f32 = 1.0;

fn spawn_minimap(query: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) {
    for window in &query {
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    viewport: Some(Viewport {
                        physical_position: UVec2::new(
                            (window.width() as u32 - MAP_WIDTH) / 2,
                            (window.height() as u32 - MAP_HEIGHT) / 2,
                        ),
                        physical_size: UVec2::new(MINIMAP_WIDTH, MINIMAP_HEIGHT),
                        ..default()
                    }),
                    order: MINIMAP_ORDER as isize,
                    ..default()
                },
                ..default()
            },
            MINIMAP_LAYER,
            MinimapCamera,
        ));
    }
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
    // TODO always small map
}

fn init_zoom(mut query: Query<&mut OrthographicProjection, With<MinimapCamera>>) {
    for mut projection in &mut query {
        projection.scale = INIT_ZOOM;
    }
}

fn drag_minimap(
    mut query: Query<(&mut Transform, &OrthographicProjection), With<MinimapCamera>>,
    left_click: Res<LeftClick>,
    window_cursor: Res<WindowCursor>,
) {
    if !left_click.pressed {
        return;
    }
    for (mut transform, projection) in &mut query {
        transform.translation.x -= window_cursor.delta.x * projection.scale * DRAGGING_RATE;
        transform.translation.y += window_cursor.delta.y * projection.scale * DRAGGING_RATE;
    }
}

fn zoom_minimap(
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

fn toggle_minimap(
    width: u32,
    height: u32,
) -> impl FnMut(Query<(&mut Camera, &mut OrthographicProjection), With<MinimapCamera>>) {
    move |mut query| {
        for (mut camera, mut projection) in &mut query {
            let Some(viewport) = camera.viewport.as_mut() else {
                continue;
            };
            viewport.physical_size.x = width;
            viewport.physical_size.y = height;
            projection.scale = INIT_ZOOM;
        }
    }
    // TODO minimap position
}

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_minimap, init_zoom).chain());
        app.add_systems(
            Update,
            (
                (
                    change_ui_state::<KeyM>(UIStates::Map),
                    trace_player::<MinimapCamera>,
                )
                    .run_if(not(in_state(UIStates::Map))),
                (
                    change_ui_state::<KeyM>(UIStates::None),
                    zoom_minimap,
                    drag_minimap,
                )
                    .run_if(in_state(UIStates::Map)),
            ),
        );
        app.add_systems(
            OnEnter(UIStates::Map),
            toggle_minimap(MAP_WIDTH, MAP_HEIGHT),
        );
        app.add_systems(
            OnExit(UIStates::Map),
            toggle_minimap(MINIMAP_WIDTH, MINIMAP_HEIGHT),
        );
    }
    // TODO fast travel
}
