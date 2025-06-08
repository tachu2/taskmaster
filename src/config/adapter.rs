pub struct Adapter;

use crate::{config::config::Config, errors::ConfigParseError};
use ini::Ini;

impl Adapter {
    pub fn parse_config(
        config: &mut Config,
        file_path: Option<&str>,
    ) -> Result<(), ConfigParseError> {
        let file_path = match file_path {
            Some(path) => path.to_string(),
            None => Config::find_config()?,
        };
        Ok(())
    }
}
