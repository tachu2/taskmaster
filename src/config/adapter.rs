pub struct Adapter;

use crate::{config::config::Config, errors::ConfigParseError};
use ini::Ini;

impl Adapter {
    pub fn parse_config(
        config: &mut Config,
        file_path: Option<&String>,
    ) -> Result<(), ConfigParseError> {
        let file_path = match file_path {
            Some(path) => path.to_string(),
            None => Config::find_config()?,
        };
        let ini = Ini::load_from_file(file_path)?;

        for (sec, prop) in ini.iter() {
            println!("Section: {:?}", sec);
            for (key, value) in prop.iter() {
                match key {
                    _ => todo!(),
                }
                println!("{}:{}", key, value);
            }
        }
        Ok(())
    }
}
