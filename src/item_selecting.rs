use crate::input::*;
use crate::item_node::*;
use crate::math::*;
use crate::ui_states::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct SelectedItem(pub u8);

fn on_wheel(mut selected: ResMut<SelectedItem>, wheel: Res<Wheel>) {
    if wheel.0 == 0 {
        return;
    }
    selected.0 = (selected.0 as i8 - wheel.0.signum()).repeat(0, 9) as u8;
}

fn on_pressed_digit(mut selected: ResMut<SelectedItem>, digits: Res<Digits>) {
    for (index, digit) in digits.iter().enumerate() {
        if digit.just_pressed {
            selected.0 = index as u8;
        }
    }
}

fn sync_selected(
    mut query: Query<(&ItemIndex, &mut Visibility), With<Text>>,
    selected: Res<SelectedItem>,
) {
    if !selected.is_changed() {
        return;
    }
    for (index, mut visibility) in &mut query {
        *visibility = if index.0 == selected.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        }
    }
}

pub struct ItemSelectingPlugin;

impl Plugin for ItemSelectingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedItem(0));
        app.add_systems(
            Update,
            (
                on_wheel.run_if(not(in_state(UIStates::Minimap))),
                on_pressed_digit,
                sync_selected,
            ),
        );
    }
}
