use std::collections::HashMap;

use bevy::prelude::*;

pub struct Atlas {
    pub path: String,
}

#[derive(Resource, Deref, DerefMut)]
pub struct AtlasMap(pub HashMap<u16, Atlas>);

fn create_atlas() -> AtlasMap {
    let mut atlas = HashMap::new();
    for item in [(1, "")] {
        atlas.insert(
            item.0,
            Atlas {
                path: String::from(item.1),
            },
        );
    }
    AtlasMap(atlas)
}

pub struct AtlasPlugin;

impl Plugin for AtlasPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_atlas());
    }
}
