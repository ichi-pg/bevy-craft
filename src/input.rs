use crate::camera::*;
use bevy::input::mouse::*;
use bevy::prelude::*;
use bevy::window::*;
use bevy_craft::*;

macro_rules! define_pressed {
    ( $( $x:tt ),* ) => {
        $(
            #[derive(Resource, Default, Pressed)]
            pub struct $x {
                pub pressed: bool,
                pub just_pressed: bool,
            }
        )*
    };
}

define_pressed!(
    Digit, LeftClick, RightClick, Escape, Tab, Enter, Space, Alt, Shift, Control, KeyQ, KeyE, KeyR,
    KeyT, KeyF, KeyG, KeyC, KeyV, KeyB, KeyM
);

macro_rules! define_cursor {
    ( $( $x:tt ),* ) => {
        $(
            #[derive(Resource, Default)]
            pub struct $x {
                pub position: Vec2,
                pub delta: Vec2,
            }
        )*
    };
}

define_cursor!(WorldCursor, WindowCursor);

#[derive(Resource, Deref, DerefMut)]
pub struct LeftStick(pub Vec2);

#[derive(Resource)]
pub struct Wheel(pub i8);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct Digits(pub [Digit; 10]);

pub trait Pressed {
    fn pressed(&self) -> bool;
    fn just_pressed(&self) -> bool;
    fn set_just_pressed(&mut self, pressed: bool);
    fn set_pressed(&mut self, pressed: bool);
}

fn read_stick(mut left_stick: ResMut<LeftStick>, keyboard: Res<ButtonInput<KeyCode>>) {
    left_stick.0 = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        left_stick.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        left_stick.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        left_stick.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        left_stick.x -= 1.0;
    }
}

fn read_numbers(mut digits: ResMut<Digits>, keyboard: Res<ButtonInput<KeyCode>>) {
    macro_rules! number_pressed {
        ( $( ( $x:tt, $y:tt) ),* ) => {
            $(
                digits[$x].just_pressed = keyboard.just_pressed(KeyCode::$y) && !digits[$x].just_pressed;
                digits[$x].pressed = keyboard.pressed(KeyCode::$y);
            )*
        };
    }
    number_pressed!(
        (0, Digit1),
        (1, Digit2),
        (2, Digit3),
        (3, Digit4),
        (4, Digit5),
        (5, Digit6),
        (6, Digit7),
        (7, Digit8),
        (8, Digit9),
        (9, Digit0)
    );
}

fn read_pressed<T: Resource + Pressed>(
    code: KeyCode,
) -> impl FnMut(ResMut<T>, Res<ButtonInput<KeyCode>>) {
    move |mut res, keyboard| {
        let pressed = res.pressed();
        res.set_just_pressed(keyboard.just_pressed(code) && !pressed);
        res.set_pressed(keyboard.pressed(code));
    }
}

fn read_click(
    mut left_click: ResMut<LeftClick>,
    mut right_click: ResMut<RightClick>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    left_click.just_pressed = mouse.just_pressed(MouseButton::Left) && !left_click.just_pressed;
    left_click.pressed = mouse.pressed(MouseButton::Left);
    right_click.just_pressed = mouse.just_pressed(MouseButton::Right) && !right_click.just_pressed;
}

fn read_wheel(mut mut_wheel: ResMut<Wheel>, mut wheels: EventReader<MouseWheel>) {
    mut_wheel.0 = 0;
    for wheel in wheels.read() {
        mut_wheel.0 += match wheel.unit {
            MouseScrollUnit::Line => wheel.y as i8,
            MouseScrollUnit::Pixel => wheel.y as i8,
        };
    }
}

fn read_cursor(
    mut mut_window_cursor: ResMut<WindowCursor>,
    mut mut_world_cursor: ResMut<WorldCursor>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
) {
    for (camera, transform) in &camera_query {
        for window in &window_query {
            if let Some(window_cursor) = window.cursor_position() {
                if let Some(world_cursor) = camera
                    .viewport_to_world(transform, window_cursor)
                    .map(|ray| ray.origin.truncate())
                {
                    mut_world_cursor.delta = world_cursor - mut_world_cursor.position;
                    mut_world_cursor.position = world_cursor;
                }
                mut_window_cursor.delta = window_cursor - mut_window_cursor.position;
                mut_window_cursor.position = window_cursor;
            }
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        macro_rules! insert_pressed {
            ( $( ( $x:tt, $y:tt) ),* ) => {
                $(
                    app.insert_resource($x::default());
                    app.add_systems(PreUpdate, read_pressed::<$x>(KeyCode::$y));
                )*
            };
        }
        insert_pressed!(
            (Escape, Escape),
            (Tab, Tab),
            (Enter, Enter),
            (Space, Space),
            (Alt, AltLeft),
            (Shift, ShiftLeft),
            (Control, ControlLeft),
            (KeyQ, KeyQ),
            (KeyE, KeyE),
            (KeyR, KeyR),
            (KeyT, KeyT),
            (KeyF, KeyF),
            (KeyG, KeyG),
            (KeyC, KeyC),
            (KeyV, KeyV),
            (KeyB, KeyB),
            (KeyM, KeyM)
        );
        app.insert_resource(Wheel(0));
        app.insert_resource(WindowCursor::default());
        app.insert_resource(WorldCursor::default());
        app.insert_resource(LeftStick(Vec2::ZERO));
        app.insert_resource(LeftClick::default());
        app.insert_resource(RightClick::default());
        app.insert_resource(Digits::default());
        app.add_systems(
            PreUpdate,
            (
                read_stick,
                read_click,
                read_wheel,
                read_cursor,
                read_numbers,
            ),
        );
    }
    // FIXME not update frame
    // TODO optimize to changed
    // TODO other devices
    // TODO abstract names
    // TODO flexible key codes
    // TODO using states?
}
