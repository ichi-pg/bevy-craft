use crate::input::*;
use crate::player::*;
use crate::ui_states::*;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy::render::view::*;
use bevy::window::*;

#[derive(Component)]
struct MinimapCamera;

const MINIMAP_ORDER: usize = 1;
pub const MINIMAP_LAYER: RenderLayers = RenderLayers::layer(MINIMAP_ORDER);

pub const MINIMAP_ALPHA: f32 = 0.5;
const MINIMAP_Z: f32 = -1.0;
const WORLD_WIDTH: f32 = 1000000.0;
const WORLD_HEIGHT: f32 = 1000000.0;

const MINIMAP_WIDTH: u32 = 1600;
const MINIMAP_HEIGHT: u32 = 900;

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
                            (window.width() as u32 - MINIMAP_WIDTH) / 2,
                            (window.height() as u32 - MINIMAP_HEIGHT) / 2,
                        ),
                        physical_size: UVec2::new(MINIMAP_WIDTH, MINIMAP_HEIGHT),
                        ..default()
                    }),
                    order: MINIMAP_ORDER as isize,
                    is_active: false,
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
                color: Color::srgba(0.2, 0.2, 0.2, MINIMAP_ALPHA),
                custom_size: Some(Vec2::new(WORLD_WIDTH, WORLD_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, MINIMAP_Z),
            ..default()
        },
        MINIMAP_LAYER,
    ));
}

fn init_zoom(mut query: Query<&mut OrthographicProjection, With<MinimapCamera>>) {
    for mut projection in &mut query {
        projection.scale = INIT_ZOOM;
    }
}

fn dragging_minimap(
    mut query: Query<(&mut Transform, &OrthographicProjection), With<MinimapCamera>>,
    left_click: Res<LeftClick>,
    window_cursor: Res<WindowCursor>,
) {
    if !left_click.pressed {
        return;
    }
    for (mut transform, projection) in &mut query {
        transform.translation.x += window_cursor.delta.x * projection.scale * DRAGGING_RATE;
        transform.translation.y -= window_cursor.delta.y * projection.scale * DRAGGING_RATE;
    }
}

fn on_wheel(mut query: Query<&mut OrthographicProjection, With<MinimapCamera>>, wheel: Res<Wheel>) {
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

fn activate_minimap(
    is_active: bool,
) -> impl FnMut(
    Query<(&mut Camera, &mut Transform), With<MinimapCamera>>,
    Query<&Transform, (With<PlayerController>, Without<MinimapCamera>)>,
) {
    move |mut query, player_query| {
        for player_transform in &player_query {
            for (mut camera, mut transform) in &mut query {
                camera.is_active = is_active;
                transform.translation.x = player_transform.translation.x;
                transform.translation.y = player_transform.translation.y;
            }
        }
    }
}

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_minimap);
        app.add_systems(PostStartup, init_zoom);
        app.add_systems(
            Update,
            (
                change_ui_state::<KeyM>(UIStates::Minimap).run_if(not(in_state(UIStates::Minimap))),
                change_ui_state::<KeyM>(UIStates::None).run_if(in_state(UIStates::Minimap)),
                on_wheel,
                dragging_minimap,
            ),
        );
        app.add_systems(OnEnter(UIStates::Minimap), activate_minimap(true));
        app.add_systems(OnExit(UIStates::Minimap), activate_minimap(false));
    }
    // TODO fast travel
}
