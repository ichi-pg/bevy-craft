use crate::input::*;
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIStates {
    None,
    Inventory,
    Storage,
    Craft,
}

pub fn open_ui<T: Resource + Pressed>(
    state: UIStates,
) -> impl FnMut(Res<T>, ResMut<NextState<UIStates>>) {
    move |pressed: Res<T>, mut next_state: ResMut<NextState<UIStates>>| {
        if !pressed.pressed() {
            return;
        }
        next_state.set(state.clone());
    }
}

pub fn close_ui<T: Resource + Pressed>(
    pressed: Res<T>,
    mut next_state: ResMut<NextState<UIStates>>,
) {
    if !pressed.pressed() {
        return;
    }
    next_state.set(UIStates::None);
}

pub fn on_open_ui<T: Component, U: Component>(
    mut query: Query<&mut Visibility, Or<(With<T>, With<U>)>>,
) {
    for mut visibility in &mut query {
        *visibility = Visibility::Inherited;
    }
}

pub fn on_close_ui<T: Component, U: Component>(
    mut query: Query<&mut Visibility, Or<(With<T>, With<U>)>>,
) {
    for mut visibility in &mut query {
        *visibility = Visibility::Hidden;
    }
}

pub struct UIStatusPlugin;

impl Plugin for UIStatusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(UIStates::None);
        app.add_systems(Update, close_ui::<Escape>);
    }
}
