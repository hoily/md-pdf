use serde_yaml;
use serde_yaml::Value;
use std::path::PathBuf;
use std::collections::HashMap;
use utils::result_prefix;

pub mod read;
pub mod validate;
pub mod consts;
pub mod validate_types;
pub mod csl;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub input: Vec<PathBuf>,
    pub output: HashMap<String, PathBuf>,
    pub title: String,
    pub verbosity: u64,
    pub references: Option<References>
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct References {
    pub bibliography: PathBuf,
    pub csl: PathBuf
}

impl Config {
    fn new(raw: Value) -> Config {
        return Config {
            input: read::get_input_files(raw.clone()),
            output: read::get_output_files(raw.clone()),
            title: read::get_string(&raw, "title"),
            references: read::get_references(raw.clone()),
            ..Default::default()
        };
    }
}


pub fn get_config() -> Result<Config, String> {
    let config_str = try!(read::read());
    let config: Value =
        try!(result_prefix(serde_yaml::from_str(&config_str), "Config Parse Error".into()));
    try!(result_prefix(validate::validate(config.clone()), "Config Validation Error".into()));
    return Ok(Config::new(config));
}
