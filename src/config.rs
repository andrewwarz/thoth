use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub model: String,
    pub ollama_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: "gemma3:latest".to_string(),
            ollama_url: "http://localhost:11434".to_string(),
        }
    }
}

impl Config {
    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("thoth").join("config.toml"))
    }

    pub fn load() -> Option<Self> {
        let path = Self::path()?;
        let content = fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::path().ok_or("Could not determine config directory")?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;

        Ok(())
    }
}
