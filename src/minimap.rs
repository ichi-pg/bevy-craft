use crate::block::*;
use crate::input::*;
use crate::player::*;
use crate::ui_parts::*;
use crate::ui_states::*;
use crate::window::*;
use crate::z_sort::*;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy::render::view::*;
use bevy::window::*;

#[derive(Component)]
struct MinimapCamera;

#[derive(Component)]
struct MinimapParent;

#[derive(Component)]
struct PlayerMarker;

enum MapMode {
    Minimap,
    Fullmap,
}

pub const MINIMAP_LAYER: RenderLayers = RenderLayers::layer(MINIMAP_ORDER);
pub const MINIMAP_ALPHA: f32 = 0.5;

const MINIMAP_ORDER: usize = 1;
const FULLMAP_WIDTH: f32 = 1440.0;
const FULLMAP_HEIGHT: f32 = 810.0;
const MINIMAP_WIDTH: f32 = 400.0;
const MINIMAP_HEIGHT: f32 = 225.0;

const INIT_ZOOM: f32 = 20.0 * INVERTED_BLOCK_SIZE;
const ZOOM_RATE: f32 = 1.25;
const MAX_ZOOM_COUNT: f32 = 10.0;
const DRAGGING_RATE: f32 = 1.0;

fn spawn_minimap(mut commands: Commands) {
    let scale = INIT_ZOOM * ZOOM_RATE.powf(MAX_ZOOM_COUNT);
    commands
        .spawn((SpatialBundle::default(), MinimapParent))
        .with_children(|parent| {
            parent
                .spawn((
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
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: BACKGROUND_COLOR.with_alpha(MINIMAP_ALPHA),
                                custom_size: Some(Vec2::new(
                                    FULLMAP_WIDTH * scale,
                                    FULLMAP_HEIGHT * scale,
                                )),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
                            ..default()
                        },
                        MINIMAP_LAYER,
                    ));
                });
        });
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 0.0, 0.0, MINIMAP_ALPHA),
                custom_size: Some(Vec2::splat(1.0)),
                ..default()
            },
            ..default()
        },
        MINIMAP_LAYER,
        PlayerMarker,
    ));
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

fn switch_mode(
    mode: MapMode,
) -> impl FnMut(
    Query<&mut Transform, With<MinimapParent>>,
    Query<(&mut Camera, &mut OrthographicProjection), With<MinimapCamera>>,
    Query<&Window, With<PrimaryWindow>>,
) {
    move |mut query, mut camera_query, window_query| {
        for mut transform in &mut query {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        }
        for (mut camera, mut projection) in &mut camera_query {
            for window in &window_query {
                update_mode(&mode, window, &mut camera);
            }
            projection.scale = INIT_ZOOM;
        }
    }
}

fn update_mode(mode: &MapMode, window: &Window, camera: &mut Camera) {
    let Some(viewport) = camera.viewport.as_mut() else {
        return;
    };
    match mode {
        MapMode::Minimap => {
            let scale = window.physical_height() as f32 / WINDOWED_HEIGHT;
            let width = (MINIMAP_WIDTH * scale).min(window.physical_width() as f32);
            let height = (MINIMAP_HEIGHT * scale).min(window.physical_height() as f32);
            let margin = UI_MARGIN * scale;
            viewport.physical_position.x = (window.physical_width() as f32 - width - margin) as u32;
            viewport.physical_position.y = margin as u32;
            viewport.physical_size.x = width as u32;
            viewport.physical_size.y = height as u32;
        }
        MapMode::Fullmap => {
            let scale = window.physical_height() as f32 / WINDOWED_HEIGHT;
            let width = (FULLMAP_WIDTH * scale).min(window.physical_width() as f32);
            let height = (FULLMAP_HEIGHT * scale).min(window.physical_height() as f32);
            viewport.physical_position.x = ((window.physical_width() as f32 - width) * 0.5) as u32;
            viewport.physical_position.y =
                ((window.physical_height() as f32 - height) * 0.5) as u32;
            viewport.physical_size.x = width as u32;
            viewport.physical_size.y = height as u32;
        }
    }
}

fn window_resized(
    mut camera_query: Query<&mut Camera, With<MinimapCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    event_reader: EventReader<WindowResized>,
    state: Res<State<UIStates>>,
) {
    if event_reader.is_empty() {
        return;
    }
    for mut camera in &mut camera_query {
        for window in &window_query {
            match state.get() {
                UIStates::Map => update_mode(&MapMode::Fullmap, window, &mut camera),
                _ => update_mode(&MapMode::Minimap, window, &mut camera),
            }
        }
    }
}

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_minimap,
                (switch_mode(MapMode::Minimap), init_zoom).after(spawn_minimap),
            ),
        );
        app.add_systems(
            Update,
            (
                window_resized,
                trace_player::<MinimapCamera>(INVERTED_BLOCK_SIZE),
                trace_player::<PlayerMarker>(INVERTED_BLOCK_SIZE),
                change_ui_state::<KeyM>(UIStates::Map).run_if(not(in_state(UIStates::Map))),
                (
                    change_ui_state::<KeyM>(UIStates::None),
                    zoom_fullmap,
                    drag_fullmap,
                )
                    .run_if(in_state(UIStates::Map)),
            ),
        );
        app.add_systems(OnEnter(UIStates::Map), switch_mode(MapMode::Fullmap));
        app.add_systems(OnExit(UIStates::Map), switch_mode(MapMode::Minimap));
    }
    // TODO fast travel
}
