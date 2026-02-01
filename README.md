# Thoth

> *Named after the Egyptian god of wisdom, writing, and magic*

**Natural language to shell commands, powered by local LLMs.**

Describe what you want in plain English, and Thoth translates it to the right shell command for your OS.

```bash
$ thoth find all files larger than 100mb
$ find . -size +100M -type f
./Downloads/movie.mp4
./Documents/backup.zip
```

## Features

- **Natural language input** - No need to remember command syntax
- **OS-aware** - Generates macOS, Ubuntu, Fedora, Arch, or Windows commands automatically
- **Local & private** - Uses Ollama, everything runs on your machine
- **Zero config** - Just install and use
- **Fast** - Direct execution, no confirmation prompts

## Quick Start

### 1. Install Ollama

```bash
# macOS
brew install ollama

# Linux
curl -fsSL https://ollama.com/install.sh | sh
```

### 2. Pull a model

```bash
ollama pull gemma3
```

### 3. Install Thoth

**From releases (recommended):**

```bash
# macOS (Apple Silicon)
curl -L https://github.com/andrewwarz/thoth/releases/latest/download/thoth-macos-arm64 -o /usr/local/bin/thoth
chmod +x /usr/local/bin/thoth

# macOS (Intel)
curl -L https://github.com/andrewwarz/thoth/releases/latest/download/thoth-macos-x64 -o /usr/local/bin/thoth
chmod +x /usr/local/bin/thoth

# Linux (x64)
curl -L https://github.com/andrewwarz/thoth/releases/latest/download/thoth-linux-x64 -o /usr/local/bin/thoth
chmod +x /usr/local/bin/thoth
```

**From source:**

```bash
git clone https://github.com/andrewwarz/thoth
cd thoth
cargo build --release
cp target/release/thoth /usr/local/bin/
```

### 4. Use it

```bash
thoth show disk usage
thoth find what is using port 3000
thoth list all running docker containers
thoth compress this folder into a tar.gz
```

## Examples

| You type | Thoth runs |
|----------|------------|
| `thoth show my ip address` | `ifconfig \| grep inet` (macOS) |
| `thoth install htop` | `brew install htop` (macOS) / `apt install htop` (Ubuntu) |
| `thoth find large files` | `find . -size +100M -type f` |
| `thoth what is using port 8080` | `lsof -i :8080` |
| `thoth show git branches` | `git branch -a` |
| `thoth kill process named node` | `pkill node` |
| `thoth count lines of code` | `find . -name "*.py" \| xargs wc -l` |

## Configuration

Thoth works out of the box with sensible defaults.

**Environment variables:**

| Variable | Default | Description |
|----------|---------|-------------|
| `THOTH_MODEL` | `gemma3:latest` | Ollama model to use |
| `THOTH_OLLAMA_URL` | `http://localhost:11434` | Ollama API endpoint |

```bash
# Use a different model
THOTH_MODEL=llama3 thoth find hidden files
```

## How it works

1. Thoth detects your OS (macOS, Ubuntu, Fedora, Arch, etc.)
2. Sends your query + OS context to a local Ollama model
3. Parses the generated command
4. Executes it directly in your shell

All processing happens locally. Nothing is sent to external servers.

## Supported Operating Systems

| OS | Detection | Package Manager |
|----|-----------|-----------------|
| macOS | Automatic | brew |
| Ubuntu/Debian | `/etc/os-release` | apt |
| Fedora | `/etc/os-release` | dnf |
| RHEL/CentOS | `/etc/os-release` | yum/dnf |
| Arch | `/etc/os-release` | pacman |
| Alpine | `/etc/os-release` | apk |
| openSUSE | `/etc/os-release` | zypper |

## Requirements

- [Ollama](https://ollama.com) running locally
- A pulled model (gemma3, llama3, mistral, etc.)

## License

MIT
