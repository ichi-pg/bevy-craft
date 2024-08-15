use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIStates {
    None,
    Inventory,
    Storage,
    Craft,
}

pub struct UIStatusPlugin;

impl Plugin for UIStatusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(UIStates::None);
    }
    // TODO can open when other opened
}
