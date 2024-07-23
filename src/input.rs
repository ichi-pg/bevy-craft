use bevy::{
    input::mouse::*,
    prelude::*,
};

#[derive(Resource, Default)]
pub struct Input {
    pub wheel: i8,
    pub cursor: Vec2,
    pub stick: Vec2,
    pub left: bool,
    pub right: bool,
    pub escape: bool,
    pub tab: bool,
    pub shift_pressed: bool,
    pub ctrl: bool,
    pub space: bool,
    pub q: bool,
    pub e: bool,
    pub r: bool,
    pub f: bool,
    pub v: bool,
    pub c: bool,
    pub num: [bool; 10],
}

fn update_input(
    mut input: ResMut<Input>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut wheels: EventReader<MouseWheel>,
    mut cursors: EventReader<CursorMoved>,
) {
    input.stick = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        input.stick.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        input.stick.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        input.stick.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        input.stick.x -= 1.0;
    }
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
    for cursor in cursors.read() {
        input.cursor = cursor.position;
    }
    input.left = mouse.just_released(MouseButton::Left);
    input.right = mouse.just_pressed(MouseButton::Right);
    input.escape = keyboard.just_pressed(KeyCode::Escape);
    input.tab = keyboard.just_pressed(KeyCode::Tab);
    input.shift_pressed = keyboard.pressed(KeyCode::ShiftLeft);
    input.ctrl = keyboard.just_pressed(KeyCode::ControlLeft);
    input.space = keyboard.just_pressed(KeyCode::Space);
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

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Input {
            ..default()
        });
        app.add_systems(Update, update_input);
    }
}
