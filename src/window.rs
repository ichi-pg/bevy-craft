use crate::input::*;
use bevy::prelude::*;
use bevy::window::*;

pub const WINDOWED_WIDTH: f32 = 1920.0;
pub const WINDOWED_HEIGHT: f32 = 1080.0;

fn toggle_fullscreen(
    mut query: Query<&mut Window, With<PrimaryWindow>>,
    alt: Res<AltRight>,
    enter: Res<Enter>,
) {
    if !alt.pressed {
        return;
    }
    if !enter.just_pressed {
        return;
    }
    for mut window in &mut query {
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen,
            _ => WindowMode::Windowed,
        }
    }
}

fn window_resized(mut query: Query<&mut Window>, mut event_reader: EventReader<WindowResized>) {
    for event in event_reader.read() {
        let Ok(mut window) = query.get_mut(event.window) else {
            continue;
        };
        let height = window.physical_height();
        window
            .resolution
            .set_scale_factor(height as f32 / WINDOWED_HEIGHT);
    }
}

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (toggle_fullscreen, window_resized.after(toggle_fullscreen)),
        );
    }
}
