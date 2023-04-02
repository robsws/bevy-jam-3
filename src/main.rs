use bevy::prelude::*;
use config::Config;

#[derive(Resource)]
struct Settings(Config);

fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()
        .unwrap();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Inner Demons".into(),
                resolution: (1000., 800.).into(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Settings(config))
        .run();
}
