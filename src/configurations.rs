use serde::Deserialize;
use std::collections::HashMap;
// use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct DebuggerConfig {
    pub name: String,

    #[serde(rename = "type")]
    pub program_type: String,

    #[serde(rename = "type-path")]
    pub type_path: String,
    pub request: RequestType,
    pub program: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestType {
    Launch,
    Attach,
}

impl DebuggerConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::File::new("launch.yml", config::FileFormat::Yaml))
            .build()?;

        config.try_deserialize::<DebuggerConfig>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_configuration() {
        let config = DebuggerConfig::new().expect("Should've been able to parse config file");

        dbg!(config);
    }
}
