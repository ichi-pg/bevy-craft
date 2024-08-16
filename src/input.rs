use bevy::input::mouse::*;
use bevy::prelude::*;
use bevy::window::*;

#[derive(Resource)]
pub struct Wheel(pub i8);

#[derive(Resource, Deref, DerefMut)]
pub struct WorldCursor(pub Vec2);

#[derive(Resource, Deref, DerefMut)]
pub struct WindowCursor(pub Vec2);

#[derive(Resource, Deref, DerefMut)]
pub struct LeftStick(pub Vec2);

#[derive(Resource)]
pub struct LeftClick(pub bool);

#[derive(Resource)]
pub struct LeftClickPressed(pub bool);

#[derive(Resource)]
pub struct RightClick(pub bool);

#[derive(Resource)]
pub struct Escape(pub bool);

#[derive(Resource)]
pub struct Tab(pub bool);

#[derive(Resource)]
pub struct Enter(pub bool);

#[derive(Resource)]
pub struct AltPressed(pub bool);

#[derive(Resource)]
pub struct ShiftPressed(pub bool);

#[derive(Resource)]
pub struct CtrlPressed(pub bool);

#[derive(Resource)]
pub struct SpacePressed(pub bool);

#[derive(Resource)]
pub struct KeyQ(pub bool);

#[derive(Resource)]
pub struct KeyE(pub bool);

#[derive(Resource)]
pub struct KeyR(pub bool);

#[derive(Resource)]
pub struct KeyF(pub bool);

#[derive(Resource)]
pub struct KeyC(pub bool);

#[derive(Resource)]
pub struct KeyV(pub bool);

#[derive(Resource)]
pub struct KeyB(pub bool);

#[derive(Resource)]
pub struct KeyM(pub bool);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct KeyNum(pub [bool; 10]);

pub trait Pressed {
    fn pressed(&self) -> bool;
}

impl Pressed for Escape {
    fn pressed(&self) -> bool {
        self.0
    }
}

impl Pressed for Tab {
    fn pressed(&self) -> bool {
        self.0
    }
}

impl Pressed for KeyC {
    fn pressed(&self) -> bool {
        self.0
    }
}

fn read_keyboard1(
    mut left_stick: ResMut<LeftStick>,
    mut escape: ResMut<Escape>,
    mut tab: ResMut<Tab>,
    mut enter: ResMut<Enter>,
    mut alt_pressed: ResMut<AltPressed>,
    mut shift_pressed: ResMut<ShiftPressed>,
    mut ctrl_pressed: ResMut<CtrlPressed>,
    mut space_pressed: ResMut<SpacePressed>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
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
    escape.0 = keyboard.just_pressed(KeyCode::Escape) && !escape.0;
    tab.0 = keyboard.just_pressed(KeyCode::Tab) && !tab.0;
    enter.0 = keyboard.just_pressed(KeyCode::Enter) && !enter.0;
    alt_pressed.0 = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);
    shift_pressed.0 = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    ctrl_pressed.0 =
        keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
    space_pressed.0 = keyboard.pressed(KeyCode::Space);
}

fn read_keyboard2(
    mut key_q: ResMut<KeyQ>,
    mut key_e: ResMut<KeyE>,
    mut key_r: ResMut<KeyR>,
    mut key_f: ResMut<KeyF>,
    mut key_c: ResMut<KeyC>,
    mut key_v: ResMut<KeyV>,
    mut key_b: ResMut<KeyB>,
    mut key_m: ResMut<KeyM>,
    mut key_num: ResMut<KeyNum>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    key_q.0 = keyboard.just_pressed(KeyCode::KeyQ) && !key_q.0;
    key_e.0 = keyboard.just_pressed(KeyCode::KeyE) && !key_e.0;
    key_r.0 = keyboard.just_pressed(KeyCode::KeyR) && !key_r.0;
    key_f.0 = keyboard.just_pressed(KeyCode::KeyF) && !key_f.0;
    key_c.0 = keyboard.just_pressed(KeyCode::KeyC) && !key_c.0;
    key_v.0 = keyboard.just_pressed(KeyCode::KeyV) && !key_v.0;
    key_b.0 = keyboard.just_pressed(KeyCode::KeyB) && !key_b.0;
    key_m.0 = keyboard.just_pressed(KeyCode::KeyM) && !key_m.0;
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
                read_keyboard1,
                read_keyboard2,
                read_mouse,
                read_wheel,
                read_cursor,
            ),
        );
    }
    // FIXME not update frame
    // TODO optimize to changed
    // TODO other devices
    // TODO abstract names
}
