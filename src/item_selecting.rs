use crate::input::*;
use crate::item_container::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct SelectedItem(pub u8);

fn change_selected(mut selected: ResMut<SelectedItem>, input: Res<Input>) {
    if input.wheel < 0 {
        if selected.0 == 9 {
            selected.0 = 0;
        } else {
            selected.0 += 1;
        }
    } else if input.wheel > 0 {
        if selected.0 == 0 {
            selected.0 = 9;
        } else {
            selected.0 -= 1;
        }
    } else {
        for (index, just_pressed) in input.num.iter().enumerate() {
            if *just_pressed {
                selected.0 = index as u8;
            }
        }
    }
}

fn sync_selected(
    mut query: Query<(&ItemIndex, &mut Visibility), With<Text>>,
    selected: Res<SelectedItem>,
) {
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
        app.add_systems(Update, (change_selected, sync_selected));
    }
}
