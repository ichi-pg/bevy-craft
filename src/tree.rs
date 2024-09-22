use bevy::prelude::*;

#[derive(Component)]
pub struct Tree;

fn update_tree(query: Query<&Transform, With<Tree>>, mut commands: Commands) {
    for transform in &query {}
}

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_tree);
    }
}
