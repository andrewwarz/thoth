use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::env;
use std::process::Command;
use thoth::config::Config;
use thoth::ollama::OllamaClient;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    if args[1] == "config" {
        run_config().await;
        return;
    }

    let config = match Config::load() {
        Some(c) => c,
        None => {
            eprintln!("No config found. Run 'thoth config' to set up.");
            std::process::exit(1);
        }
    };

    let query = args[1..].join(" ");
    let client = OllamaClient::new(&config.ollama_url, &config.model);

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

fn print_usage() {
    eprintln!("Thoth - Natural language to shell commands");
    eprintln!();
    eprintln!("Usage: thoth <natural language query>");
    eprintln!("       thoth config");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  thoth find all files larger than 10mb");
    eprintln!("  thoth what is using port 3000");
    eprintln!("  thoth show disk usage");
}

async fn run_config() {
    let theme = ColorfulTheme::default();

    // Get current config or defaults
    let current = Config::load().unwrap_or_default();

    println!("Fetching models from Ollama...\n");

    let models = match OllamaClient::list_models(&current.ollama_url).await {
        Ok(m) if !m.is_empty() => m,
        Ok(_) => {
            eprintln!("No models found. Install one with: ollama pull gemma3");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Could not connect to Ollama at {}", current.ollama_url);
            eprintln!("Error: {}", e);
            eprintln!();
            eprintln!("Make sure Ollama is running: ollama serve");
            std::process::exit(1);
        }
    };

    // Build selection items
    let items: Vec<String> = models
        .iter()
        .map(|m| format!("{} ({})", m.name, m.size_human()))
        .collect();

    // Find current model index for default selection
    let default_idx = models
        .iter()
        .position(|m| m.name == current.model)
        .unwrap_or(0);

    let selection = Select::with_theme(&theme)
        .with_prompt("Select your model")
        .items(&items)
        .default(default_idx)
        .interact()
        .expect("Failed to get selection");

    let selected_model = &models[selection];

    let ollama_url: String = Input::with_theme(&theme)
        .with_prompt("Ollama URL")
        .default(current.ollama_url)
        .interact_text()
        .expect("Failed to get input");

    let config = Config {
        model: selected_model.name.clone(),
        ollama_url,
    };

    match config.save() {
        Ok(_) => {
            let path = Config::path().unwrap();
            println!("\n✓ Saved to {}", path.display());
        }
        Err(e) => {
            eprintln!("Failed to save config: {}", e);
            std::process::exit(1);
        }
    }
}
