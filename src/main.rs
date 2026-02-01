use thoth::ollama::OllamaClient;
use std::env;
use std::process::Command;

const DEFAULT_MODEL: &str = "gemma3:latest";
const DEFAULT_OLLAMA_URL: &str = "http://localhost:11434";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Thoth - Natural language to shell commands");
        eprintln!();
        eprintln!("Usage: thoth <natural language query>");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  thoth find all files larger than 10mb");
        eprintln!("  thoth what is using port 3000");
        eprintln!("  thoth show disk usage");
        eprintln!("  thoth install htop");
        std::process::exit(1);
    }

    let model = env::var("THOTH_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());
    let ollama_url = env::var("THOTH_OLLAMA_URL").unwrap_or_else(|_| DEFAULT_OLLAMA_URL.to_string());

    let query = args[1..].join(" ");
    let client = OllamaClient::new(&ollama_url, &model);

    match client.natural_to_command(&query).await {
        Ok(command) => {
            if command.is_empty() {
                eprintln!("Could not generate a command");
                std::process::exit(1);
            }

            println!("\x1b[90m$ {}\x1b[0m", command);

            let status = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .status()
                .expect("Failed to execute command");

            std::process::exit(status.code().unwrap_or(1));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
