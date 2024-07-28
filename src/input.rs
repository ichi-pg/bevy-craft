use bevy::{
    input::mouse::{
        MouseWheel,
        MouseScrollUnit,
    },
    prelude::*,
    window::PrimaryWindow,
};

#[derive(Resource, Default)]
pub struct Input {
    pub wheel: i8,
    pub cursor: Vec2,
    pub left_stick: Vec2,
    pub left_click: bool,
    pub right_click: bool,
    pub escape: bool,
    pub tab: bool,
    pub shift_pressed: bool,
    pub ctrl: bool,
    pub space_pressed: bool,
    pub q: bool,
    pub e: bool,
    pub r: bool,
    pub f: bool,
    pub v: bool,
    pub c: bool,
    pub num: [bool; 10],
}

fn read_keyboard(
    mut input: ResMut<Input>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    input.left_stick = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        input.left_stick.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        input.left_stick.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        input.left_stick.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        input.left_stick.x -= 1.0;
    }
    input.escape = keyboard.just_pressed(KeyCode::Escape);
    input.tab = keyboard.just_pressed(KeyCode::Tab);
    input.shift_pressed = keyboard.pressed(KeyCode::ShiftLeft);
    input.ctrl = keyboard.just_pressed(KeyCode::ControlLeft);
    input.space_pressed = keyboard.pressed(KeyCode::Space);
    input.q = keyboard.just_pressed(KeyCode::KeyQ);
    input.e = keyboard.just_pressed(KeyCode::KeyE);
    input.r = keyboard.just_pressed(KeyCode::KeyR);
    input.f = keyboard.just_pressed(KeyCode::KeyF);
    input.v = keyboard.just_pressed(KeyCode::KeyV);
    input.c = keyboard.just_pressed(KeyCode::KeyC);
    input.num[0] = keyboard.just_pressed(KeyCode::Digit0);
    input.num[1] = keyboard.just_pressed(KeyCode::Digit1);
    input.num[2] = keyboard.just_pressed(KeyCode::Digit2);
    input.num[3] = keyboard.just_pressed(KeyCode::Digit3);
    input.num[4] = keyboard.just_pressed(KeyCode::Digit4);
    input.num[5] = keyboard.just_pressed(KeyCode::Digit5);
    input.num[6] = keyboard.just_pressed(KeyCode::Digit6);
    input.num[7] = keyboard.just_pressed(KeyCode::Digit7);
    input.num[8] = keyboard.just_pressed(KeyCode::Digit8);
    input.num[9] = keyboard.just_pressed(KeyCode::Digit9);
}

fn read_mouse(
    mut input: ResMut<Input>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    input.left_click = mouse.just_released(MouseButton::Left);
    input.right_click = mouse.just_pressed(MouseButton::Right);
}

fn read_wheel(
    mut input: ResMut<Input>,
    mut wheels: EventReader<MouseWheel>,
) {
    input.wheel = 0;
    for wheel in wheels.read() {
        input.wheel += match wheel.unit {
            MouseScrollUnit::Line => {
                wheel.y as i8
            }
            MouseScrollUnit::Pixel => {
                wheel.y as i8
            }
        };
    }
}

fn read_cursor(
    mut input: ResMut<Input>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        input.cursor = world_position;
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Input {
            ..default()
        });
        app.add_systems(Update, (
            read_keyboard,
            read_mouse,
            read_wheel,
            read_cursor,
        ));
    }
}
