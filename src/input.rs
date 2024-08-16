use bevy::input::mouse::*;
use bevy::prelude::*;
use bevy::window::*;
use bevy_craft::*;

#[derive(Resource)]
pub struct Wheel(pub i8);

#[derive(Resource, Deref, DerefMut)]
pub struct WorldCursor(pub Vec2);

#[derive(Resource, Deref, DerefMut)]
pub struct WindowCursor(pub Vec2);

#[derive(Resource, Deref, DerefMut)]
pub struct LeftStick(pub Vec2);

#[derive(Resource, Pressed)]
pub struct LeftClick(pub bool);

#[derive(Resource, Pressed)]
pub struct LeftClickPressed(pub bool);

#[derive(Resource, Pressed)]
pub struct RightClick(pub bool);

#[derive(Resource, Pressed)]
pub struct Escape(pub bool);

#[derive(Resource, Pressed)]
pub struct Tab(pub bool);

#[derive(Resource, Pressed)]
pub struct Enter(pub bool);

#[derive(Resource, Pressed)]
pub struct AltPressed(pub bool);

#[derive(Resource, Pressed)]
pub struct ShiftPressed(pub bool);

#[derive(Resource, Pressed)]
pub struct CtrlPressed(pub bool);

#[derive(Resource, Pressed)]
pub struct SpacePressed(pub bool);

#[derive(Resource, Pressed)]
pub struct KeyQ(pub bool);

#[derive(Resource, Pressed)]
pub struct KeyE(pub bool);

#[derive(Resource, Pressed)]
pub struct KeyR(pub bool);

#[derive(Resource, Pressed)]
pub struct KeyF(pub bool);

#[derive(Resource, Pressed)]
pub struct KeyC(pub bool);

#[derive(Resource, Pressed)]
pub struct KeyV(pub bool);

#[derive(Resource, Pressed)]
pub struct KeyB(pub bool);

#[derive(Resource, Pressed)]
pub struct KeyM(pub bool);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct KeyNum(pub [bool; 10]);

pub trait Pressed {
    fn pressed(&self) -> bool;
    fn set_pressed(&mut self, pressed: bool);
}

fn read_wasd(mut left_stick: ResMut<LeftStick>, keyboard: Res<ButtonInput<KeyCode>>) {
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
    codes: Vec<KeyCode>,
) -> impl FnMut(ResMut<T>, Res<ButtonInput<KeyCode>>) {
    move |mut res, keyboard| {
        for code in &codes {
            if keyboard.pressed(*code) {
                res.set_pressed(true);
                return;
            }
        }
        res.set_pressed(false);
    }
}

fn read_just_pressed<T: Resource + Pressed>(
    code: KeyCode,
) -> impl FnMut(ResMut<T>, Res<ButtonInput<KeyCode>>) {
    move |mut res, keyboard| {
        let pressed = res.pressed();
        res.set_pressed(keyboard.just_pressed(code) && !pressed);
    }
}

fn read_mouse(
    mut left_click: ResMut<LeftClick>,
    mut left_click_pressed: ResMut<LeftClickPressed>,
    mut right_click: ResMut<RightClick>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    left_click.0 = mouse.just_pressed(MouseButton::Left) && !left_click.0;
    left_click_pressed.0 = mouse.pressed(MouseButton::Left);
    right_click.0 = mouse.just_pressed(MouseButton::Right) && !right_click.0;
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
        app.insert_resource(Wheel(0));
        app.insert_resource(WindowCursor(Vec2::ZERO));
        app.insert_resource(WorldCursor(Vec2::ZERO));
        app.insert_resource(LeftStick(Vec2::ZERO));
        app.insert_resource(LeftClick(false));
        app.insert_resource(LeftClickPressed(false));
        app.insert_resource(RightClick(false));
        app.insert_resource(Escape(false));
        app.insert_resource(Tab(false));
        app.insert_resource(Enter(false));
        app.insert_resource(AltPressed(false));
        app.insert_resource(ShiftPressed(false));
        app.insert_resource(CtrlPressed(false));
        app.insert_resource(SpacePressed(false));
        app.insert_resource(KeyQ(false));
        app.insert_resource(KeyE(false));
        app.insert_resource(KeyR(false));
        app.insert_resource(KeyF(false));
        app.insert_resource(KeyC(false));
        app.insert_resource(KeyV(false));
        app.insert_resource(KeyB(false));
        app.insert_resource(KeyM(false));
        app.insert_resource(KeyNum::default());
        app.add_systems(
            PreUpdate,
            (
                read_wasd,
                read_mouse,
                read_wheel,
                read_cursor,
                read_numbers,
                read_pressed::<AltPressed>(vec![KeyCode::AltLeft, KeyCode::AltRight]),
                read_pressed::<ShiftPressed>(vec![KeyCode::ShiftLeft, KeyCode::ShiftRight]),
                read_pressed::<CtrlPressed>(vec![KeyCode::ControlLeft, KeyCode::ControlRight]),
                read_pressed::<SpacePressed>(vec![KeyCode::Space]),
                read_just_pressed::<Escape>(KeyCode::Escape),
                read_just_pressed::<Tab>(KeyCode::Tab),
                read_just_pressed::<Enter>(KeyCode::Enter),
                read_just_pressed::<KeyQ>(KeyCode::KeyQ),
                read_just_pressed::<KeyE>(KeyCode::KeyE),
                read_just_pressed::<KeyR>(KeyCode::KeyR),
                read_just_pressed::<KeyF>(KeyCode::KeyF),
                read_just_pressed::<KeyC>(KeyCode::KeyC),
                read_just_pressed::<KeyV>(KeyCode::KeyV),
                read_just_pressed::<KeyB>(KeyCode::KeyB),
                read_just_pressed::<KeyM>(KeyCode::KeyM),
            ),
        );
    }
    // FIXME not update frame
    // TODO optimize to changed
    // TODO other devices
    // TODO abstract names
}
