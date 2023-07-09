use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::{Read, Write};
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub database: String,
    pub ws_port: i32,
    pub ws_key: String,
}
pub fn generate_random_key(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let characters: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let key: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..characters.len());
            characters[idx]
        })
        .collect();
    key
}
pub fn write_config_to_yml(
    config: &Config,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let yaml_string = serde_yaml::to_string(config)?;
    let mut file = File::create(file_path)?;
    file.write_all(yaml_string.as_bytes())?;
    Ok(())
}
pub fn read_yml(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}
