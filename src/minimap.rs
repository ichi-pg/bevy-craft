use crate::camera::*;
use crate::input::*;
use crate::ui_states::*;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy::render::view::*;
use bevy::window::*;

#[derive(Component)]
struct MinimapCamera;

const MINIMAP_ORDER: usize = 1;
pub const MINIMAP_LAYER: RenderLayers = RenderLayers::layer(MINIMAP_ORDER);
const UI_ORDER: usize = 2;
const UI_LAYER: RenderLayers = RenderLayers::layer(UI_ORDER);
const MINIMAP_WIDTH: u32 = 1600;
const MINIMAP_HEIGHT: u32 = 900;

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
            PlayerCamera,
        ));
    }
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: UI_ORDER as isize,
                ..default()
            },
            ..default()
        },
        UI_LAYER,
    ));
    // TODO background
}

fn zoom_minimap(mut query: Query<&mut OrthographicProjection, With<MinimapCamera>>) {
    for mut projection in &mut query {
        projection.scale = 10.0;
    }
}

fn activate_minimap(is_active: bool) -> impl FnMut(Query<&mut Camera, With<MinimapCamera>>) {
    move |mut query| {
        for mut camera in &mut query {
            camera.is_active = is_active;
        }
    }
}

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_minimap);
        app.add_systems(PostStartup, zoom_minimap);
        app.add_systems(
            Update,
            (
                change_ui_state::<KeyM>(UIStates::Minimap).run_if(not(in_state(UIStates::Minimap))),
                change_ui_state::<KeyM>(UIStates::None).run_if(in_state(UIStates::Minimap)),
            ),
        );
        app.add_systems(OnEnter(UIStates::Minimap), activate_minimap(true));
        app.add_systems(OnExit(UIStates::Minimap), activate_minimap(false));
    }
    // TODO zoom in out
    // TODO fast travel
}
