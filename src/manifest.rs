use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use toml::de;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Manifest {
    dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Dependency {
    pub organization: String,
    pub version: String,
}

impl Manifest {
    pub fn load(path: &Path) -> Result<Manifest, ManifestLoadError> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        Ok(toml::from_slice(&buffer)?)
    }
}

#[derive(Debug)]
pub enum ManifestLoadError {
    IoError { io_error: io::Error },
    DecodeError { decode_error: de::Error },
}

impl From<io::Error> for ManifestLoadError {
    fn from(io_error: io::Error) -> Self {
        ManifestLoadError::IoError { io_error }
    }
}

impl From<de::Error> for ManifestLoadError {
    fn from(decode_error: de::Error) -> Self {
        ManifestLoadError::DecodeError { decode_error }
    }
}

#[cfg(test)]
mod tests {
    use crate::manifest::{Dependency, Manifest};
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn test_load() {
        let path = Path::new("test/Onus.toml");
        let config = Manifest::load(path).unwrap();

        let mut dependencies = HashMap::new();
        dependencies.insert(
            "tofu_logging".to_string(),
            Dependency {
                organization: "tf.tofu".to_string(),
                version: "0.1.0.4".to_string(),
            },
        );

        dependencies.insert(
            "tofu".to_string(),
            Dependency {
                organization: "tf.tofu".to_string(),
                version: "0.1.0.4".to_string(),
            },
        );

        dependencies.insert(
            "cats_effect".to_string(),
            Dependency {
                organization: "org.typelevel".to_string(),
                version: "1.6.0".to_string(),
            },
        );

        assert_eq!(config, Manifest { dependencies });
    }
}
