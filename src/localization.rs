use bevy::prelude::*;
use std::collections::HashMap;

pub struct LocalText {
    pub text: String,
}

#[derive(Resource, Deref, DerefMut)]
pub struct LocalTextMap(pub HashMap<u16, LocalText>);

fn create_local_texts() -> LocalTextMap {
    let mut texts = HashMap::new();
    for item in [(1, "")] {
        texts.insert(
            item.0,
            LocalText {
                text: String::from(item.1),
            },
        );
    }
    LocalTextMap(texts)
    // TODO switch with country code
}

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_local_texts());
    }
}
