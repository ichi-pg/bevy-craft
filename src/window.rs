use crate::input::*;
use bevy::prelude::*;
use bevy::window::*;
use bevy::winit::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ScreenMode {
    Windowed,
    Fullscreen,
}

fn toggle_fullscreen(
    mut query: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
    alt: Res<AltRight>,
    enter: Res<Enter>,
    mut next_state: ResMut<NextState<ScreenMode>>,
) {
    if !alt.pressed {
        return;
    }
    if !enter.just_pressed {
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
                next_state.set(ScreenMode::Fullscreen);
            }
            _ => {
                window.mode = WindowMode::Windowed;
                window.resolution.set_scale_factor(1.0);
                next_state.set(ScreenMode::Windowed);
            }
        }
    }
    // TODO web flex
}

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ScreenMode::Windowed);
        app.add_systems(Update, toggle_fullscreen);
    }
}
