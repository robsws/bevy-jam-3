use bevy::prelude::*;
use inner::CardGamePlugin;

mod inner;
mod settings;

use settings::Settings;

fn main() {
    let settings = Settings::from_config();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Inner Demons".into(),
                resolution: (settings.window.width, settings.window.height).into(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(settings)
        .add_plugin(CardGamePlugin)
        .run();
}
