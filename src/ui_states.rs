use crate::input::*;
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIStates {
    None,
    Inventory,
    Storage,
    Craft,
}

fn close_ui(escape: Res<Escape>, mut next_state: ResMut<NextState<UIStates>>) {
    if !escape.0 {
        return;
    }
    next_state.set(UIStates::None);
}

pub struct UIStatusPlugin;

impl Plugin for UIStatusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(UIStates::None);
        app.add_systems(Update, close_ui);
    }
    // TODO using generics
}
