use crate::input::*;
use bevy::prelude::*;
use bevy::state::state::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum UIStates {
    None,
    Inventory,
    Storage,
    Workbench,
    Minimap,
}

pub fn change_ui_state<T: Resource + Pressed>(
    state: UIStates,
) -> impl FnMut(Res<T>, ResMut<NextState<UIStates>>) {
    move |pressed, mut next_state| {
        if !pressed.just_pressed() {
            return;
        }
        next_state.set(state);
    }
}

pub fn change_visibility<T: Component, U: Component, V: Component>(
    visibility: Visibility,
) -> impl FnMut(Query<&mut Visibility, Or<(With<T>, With<U>, With<V>)>>) {
    move |mut query| {
        for mut mut_visibility in &mut query {
            *mut_visibility = visibility;
        }
    }
    // TODO flexible generics
}

pub fn sync_visibility<T: Component, U: FreelyMutableState + Copy>(
    visible: U,
    hidden: U,
) -> impl FnMut(Query<&Visibility, (With<T>, Changed<Visibility>)>, ResMut<NextState<U>>) {
    move |query, mut next_state| {
        for visibility in &query {
            match *visibility {
                Visibility::Inherited => next_state.set(visible),
                Visibility::Hidden => next_state.set(hidden),
                Visibility::Visible => next_state.set(visible),
            }
        }
    }
}

pub fn sync_changed<T: Component, U: Component, V: Event + Default>(
    query: Query<&T, Changed<U>>,
    mut event_writer: EventWriter<V>,
) {
    for _ in &query {
        event_writer.send(V::default());
    }
}

pub struct UIStatusPlugin;

impl Plugin for UIStatusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(UIStates::None);
        app.add_systems(Update, change_ui_state::<Escape>(UIStates::None));
    }
    // TODO escape is game settings
    // TODO shortcut navigation
}
