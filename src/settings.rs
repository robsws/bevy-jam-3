use bevy::prelude::Resource;
use config::Config;
use std::collections::HashMap;

#[derive(Resource)]
pub struct Settings {
    pub window: WindowSettings,
    pub game: GameSettings,
}

pub struct WindowSettings {
    pub width: f32,
    pub height: f32,
}

pub struct GameSettings {
    pub inner: InnerSettings,
    pub outer: OuterSettings,
}

pub struct InnerSettings {
    pub starting_resolve: u32,
    pub starting_demon_power: u32,
    pub starting_demon_stun_time: u32,
}

pub struct OuterSettings {
    pub starting_health: u32,
}

impl Settings {
    pub fn from_config() -> Settings {
        let config = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .build()
            .unwrap()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();
        Settings {
            window: WindowSettings {
                width: config.get("window__width").unwrap().parse().unwrap(),
                height: config.get("window__height").unwrap().parse().unwrap(),
            },
            game: GameSettings {
                inner: InnerSettings {
                    starting_resolve: config
                        .get("game__inner__starting_resolve")
                        .unwrap()
                        .parse()
                        .unwrap(),
                    starting_demon_power: config
                        .get("game__inner__starting_demon_power")
                        .unwrap()
                        .parse()
                        .unwrap(),
                    starting_demon_stun_time: config
                        .get("game__inner__starting_demon_stun_time")
                        .unwrap()
                        .parse()
                        .unwrap(),
                },
                outer: OuterSettings {
                    starting_health: config
                        .get("game__outer__starting_health")
                        .unwrap()
                        .parse()
                        .unwrap(),
                },
            },
        }
    }
}
