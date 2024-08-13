use bevy::prelude::*;
use bevy_framepace::*;

fn limit_framerate(mut settings: ResMut<FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(144.0);
}

pub struct FrameratePlugin;

impl Plugin for FrameratePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FramepacePlugin);
        app.add_systems(Startup, limit_framerate);
    }
}
