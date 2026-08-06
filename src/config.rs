use std::fs;

use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_addr: String
}

pub fn load() -> Config {
    let config_string = fs::read_to_string("config.toml")
        .expect("config.toml not found");
    toml::from_str::<Config>(&config_string)
        .expect("failed to parse config")
}
