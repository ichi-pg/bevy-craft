use bevy::input::mouse::*;
use bevy::prelude::*;
use bevy::window::*;
use bevy_craft::*;

macro_rules! define_pressed {
    ( $( $x:tt ),* ) => {
        $(
            #[derive(Resource, Pressed, Default)]
            pub struct $x {
                pub pressed: bool,
                pub just_pressed: bool,
            }
        )*
    };
}

define_pressed!(
    LeftClick, RightClick, Escape, Tab, Enter, Space, Alt, Shift, Control, KeyQ, KeyE, KeyR, KeyF,
    KeyC, KeyV, KeyB, KeyM
);

#[derive(Resource, Deref, DerefMut)]
pub struct WorldCursor(pub Vec2);

#[derive(Resource, Deref, DerefMut)]
pub struct WindowCursor(pub Vec2);

#[derive(Resource, Deref, DerefMut)]
pub struct LeftStick(pub Vec2);

#[derive(Resource)]
pub struct Wheel(pub i8);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct KeyNum(pub [bool; 10]);

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

fn read_numbers(mut key_num: ResMut<KeyNum>, keyboard: Res<ButtonInput<KeyCode>>) {
    key_num[0] = keyboard.just_pressed(KeyCode::Digit1) && !key_num[0];
    key_num[1] = keyboard.just_pressed(KeyCode::Digit2) && !key_num[1];
    key_num[2] = keyboard.just_pressed(KeyCode::Digit3) && !key_num[2];
    key_num[3] = keyboard.just_pressed(KeyCode::Digit4) && !key_num[3];
    key_num[4] = keyboard.just_pressed(KeyCode::Digit5) && !key_num[4];
    key_num[5] = keyboard.just_pressed(KeyCode::Digit6) && !key_num[5];
    key_num[6] = keyboard.just_pressed(KeyCode::Digit7) && !key_num[6];
    key_num[7] = keyboard.just_pressed(KeyCode::Digit8) && !key_num[7];
    key_num[8] = keyboard.just_pressed(KeyCode::Digit9) && !key_num[8];
    key_num[9] = keyboard.just_pressed(KeyCode::Digit0) && !key_num[9];
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
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    for (camera, transform) in &camera_query {
        for window in &window_query {
            if let Some(window_cursor) = window.cursor_position() {
                if let Some(world_cursor) = camera
                    .viewport_to_world(transform, window_cursor)
                    .map(|ray| ray.origin.truncate())
                {
                    mut_world_cursor.0 = world_cursor;
                }
                mut_window_cursor.0 = window_cursor;
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
        // TODO flexible key codes
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
            (KeyF, KeyR),
            (KeyC, KeyC),
            (KeyV, KeyV),
            (KeyB, KeyB),
            (KeyM, KeyM)
        );
        app.insert_resource(Wheel(0));
        app.insert_resource(WindowCursor(Vec2::ZERO));
        app.insert_resource(WorldCursor(Vec2::ZERO));
        app.insert_resource(LeftStick(Vec2::ZERO));
        app.insert_resource(LeftClick::default());
        app.insert_resource(RightClick::default());
        app.insert_resource(KeyNum::default());
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
}
