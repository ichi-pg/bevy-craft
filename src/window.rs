use crate::input::*;
use bevy::prelude::*;
use bevy::window::*;

fn toggle_fullscreen(mut query: Query<&mut Window, With<PrimaryWindow>>, input: Res<Input>) {
    if !input.alt_pressed {
        return;
    }
    if !input.enter {
        return;
    }
    for mut window in &mut query {
        match window.mode {
            WindowMode::Windowed => {
                window.mode = WindowMode::BorderlessFullscreen;
            }
            _ => {
                window.mode = WindowMode::Windowed;
            }
        }
    }
    // TODO scaling
}

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_fullscreen);
    }
}
