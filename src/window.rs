use crate::input::*;
use bevy::prelude::*;
use bevy::window::*;
use bevy::winit::*;

fn toggle_fullscreen(
    mut query: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
    input: Res<Input>,
) {
    if !input.alt_pressed {
        return;
    }
    if !input.enter {
        return;
    }
    for (entity, mut window) in &mut query {
        match window.mode {
            WindowMode::Windowed => {
                window.mode = WindowMode::BorderlessFullscreen;
                match winit_windows.get_window(entity) {
                    Some(winit_window) => match winit_window.current_monitor() {
                        Some(monitor) => {
                            let height = window.height();
                            window
                                .resolution
                                .set_scale_factor(monitor.size().height as f32 / height);
                        }
                        None => todo!(),
                    },
                    None => todo!(),
                }
            }
            _ => {
                window.mode = WindowMode::Windowed;
                window.resolution.set_scale_factor(1.0);
            }
        }
    }
}

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_fullscreen);
    }
}
