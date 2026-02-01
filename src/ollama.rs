use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env::consts::OS;
use std::fs;
use std::time::Duration;

fn get_os_name() -> String {
    match OS {
        "macos" => "macOS".to_string(),
        "windows" => "Windows".to_string(),
        "linux" => detect_linux_distro(),
        _ => OS.to_string(),
    }
}

fn detect_linux_distro() -> String {
    // Try to read /etc/os-release
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("ID=") {
                let distro = line.trim_start_matches("ID=").trim_matches('"');
                return match distro {
                    "ubuntu" => "Ubuntu Linux".to_string(),
                    "debian" => "Debian Linux".to_string(),
                    "fedora" => "Fedora Linux".to_string(),
                    "rhel" | "centos" | "rocky" | "almalinux" => "RHEL/CentOS Linux".to_string(),
                    "arch" => "Arch Linux".to_string(),
                    "alpine" => "Alpine Linux".to_string(),
                    "opensuse" | "sles" => "openSUSE Linux".to_string(),
                    _ => format!("{} Linux", distro),
                };
            }
        }
    }
    "Linux".to_string()
}

#[derive(Debug, Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: GenerateOptions,
}

#[derive(Debug, Serialize)]
struct GenerateOptions {
    num_predict: u32,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

pub struct OllamaClient {
    client: Client,
    base_url: String,
    model: String,
}

impl OllamaClient {
    pub fn new(base_url: &str, model: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.to_string(),
            model: model.to_string(),
        }
    }

    pub async fn natural_to_command(&self, query: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let os = get_os_name();
        let prompt = format!(
            "/no_think You are a {} shell command generator. Output ONLY the complete command, nothing else. Use current directory (.) when path not specified.\n\nTask: {}\nCommand:",
            os, query
        );

        let request = GenerateRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
            options: GenerateOptions { num_predict: 200 },
        };

        let response = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?
            .json::<GenerateResponse>()
            .await?;

        // Clean up response - extract command from various formats
        let mut raw = response.response.trim().to_string();

        // Strip <think>...</think> blocks from qwen3
        if let Some(end_idx) = raw.find("</think>") {
            raw = raw[end_idx + 8..].trim().to_string();
        }

        // Find the actual command line (skip empty lines and code fence markers)
        let command = raw
            .lines()
            .map(|line| line.trim())
            .filter(|line| {
                !line.is_empty()
                    && !line.starts_with("```")
                    && !line.starts_with('#')
                    && !line.starts_with("<")
            })
            .next()
            .unwrap_or("")
            .trim_start_matches('`')
            .trim_end_matches('`')
            .trim_start_matches("$ ")
            .trim_start_matches('$')
            .trim()
            .to_string();

        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_initializes_with_config() {
        let client = OllamaClient::new("http://localhost:11434", "qwen2.5-coder:1.5b-base");
        assert_eq!(client.base_url, "http://localhost:11434");
        assert_eq!(client.model, "qwen2.5-coder:1.5b-base");
    }

    #[tokio::test]
    async fn natural_to_command_returns_result() {
        let client = OllamaClient::new("http://localhost:11434", "qwen2.5-coder:1.5b-base");

        // Check if Ollama is running
        let health_check = reqwest::get("http://localhost:11434/api/tags").await;
        if health_check.is_err() {
            eprintln!("Skipping test: Ollama not running");
            return;
        }

        let result = client.natural_to_command("list files").await;
        assert!(result.is_ok(), "Expected command, got: {:?}", result);
    }
}
