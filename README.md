# 🐱 catpack

**Pack your codebase into a single file for LLM** — fast, native, zero dependencies.

`catpack` is a lightweight CLI tool written in Rust that concatenates all project files into a single document, perfect for feeding to AI models (ChatGPT, Claude, Cursor, Copilot).

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)
![License](https://img.shields.io/badge/license-Unlicense-blue)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)

## ✨ Features

- 🚀 **Instant startup** — native binary ~9.5 MB, no Node.js/Python required
- 🌳 **File tree** — AI models see project structure immediately
- 📋 **Clipboard support** — instant paste via OSC 52 (Zellij/tmux) or system clipboard
- 🧮 **Token counting** — accurate estimation for GPT-4, GPT-4o, Claude
- 🎨 **4 output formats** — Simple, Markdown, XML, Separator
- 🙈 **Smart exclusions** — reads `.catpackignore` (`.gitignore` syntax)
- 🔤 **Auto language detection** — for beautiful syntax highlighting

## 📦 Installation

### Via Cargo (Recommended)
```bash
cargo install catpack
```

### Pre-built Binaries
Download the latest release for your platform from the [GitHub Releases](https://github.com/bang-pxoyen/catpack/releases) page.

## 🚀 Quick Start

```bash
# Pack current project into catpack.txt
catpack

# For LLM (XML is the best format for AI models)
catpack -f xml -o prompt.xml

# With file tree and clipboard copy (perfect for Zellij/tmux)
catpack --tree --clip -f xml

# Count tokens without creating a file
catpack --tokens-only --model gpt-4o
```

## 📖 Usage

```text
Options:
  -o, --output <FILE>        Output file [default: catpack.txt]
  -i, --ignore <PATTERN>     Additional ignore patterns (can be used multiple times)
  -d, --dir <DIR>            Target directory [default: .]
  -f, --format <FORMAT>      Format: simple, markdown, xml, separator [default: simple]
      --tree                 Add file tree at the beginning
      --clip                 Copy result to clipboard
      --osc52                Force OSC 52 clipboard (for Zellij, tmux, SSH)
      --model <MODEL>        Model for token counting: gpt-4, gpt-4o, claude [default: gpt-4]
      --tokens-only          Only count tokens, don't create file
      --no-lang              Disable language detection
  -h, --help                 Show help
```

### Examples

```bash
# Basic run
catpack

# Specify output file name
catpack -o my_context.txt

# Add exclusions on the fly
catpack -i "*.log" -i "tests/" -i "docs/archive/"

# For different directory
catpack -d ../other-project -o dump.txt

# XML format (recommended for LLM)
catpack -f xml -o context.xml

# Markdown with file tree
catpack -f markdown --tree -o readme_dump.md

# Copy directly to clipboard
catpack --clip -f xml

# Force OSC 52 (for Zellij, tmux, SSH sessions)
catpack --clip --osc52 -f xml

# Count tokens for GPT-4o
catpack --tokens-only --model gpt-4o

# Combine all features
catpack --tree --clip --model gpt-4o -f xml -o prompt.xml
```

## 🙈 Exclusion Configuration

Create a `.catpackignore` file in your project root. Syntax is fully compatible with `.gitignore`:

```gitignore
# Dependencies and builds
node_modules/
dist/
build/
.venv/
target/

# Logs and temp files
*.log
tmp/
.env
*.min.js

# Specific files
docs/archive/
tests/fixtures/
```

*Note: `.git/`, `.gitignore`, `.catpackignore`, and the output file are always excluded automatically.*

## 🎨 Output Formats

### `xml` ⭐ (Recommended for LLM)
```xml
<file path="src/main.py" lang="python">
def hello():
    print("Hello!")
</file>
```

### `markdown`
````markdown
## 📄 `src/main.py`

```python
def hello():
    print("Hello!")
```
````

### `simple` (default)
```
===== src/main.py =====

def hello():
    print("Hello!")
```

### `separator`
```
============================================================
 src/main.py [python]
============================================================

def hello():
    print("Hello!")

============================================================
```

### Example with file tree (`--tree`)

```
# 📁 Project Structure
#
# ├── src/
# │   ├── main.rs
# │   └── utils.rs
# └─ Cargo.toml

<file path="Cargo.toml" lang="toml">
[package]
name = "myapp"
</file>

<file path="src/main.rs" lang="rust">
fn main() {
    println!("Hello!");
}
</file>
```

## 🧮 Token Counting

`catpack` supports accurate token counting to help you stay within context limits:

| Model | Encoding | Accuracy |
|-------|----------|----------|
| `gpt-4` | cl100k_base | 100% |
| `gpt-4o` | o200k_base | 100% |
| `claude` | cl100k_base (approx.) | ~90-95% |

```bash
$ catpack --tokens-only --model gpt-4o
📊 42 files, 15.3K tokens (gpt-4o), 0.45 MB
```

>  **Tip:** Use `--tokens-only` before sending to LLM to ensure you don't exceed context limits.

## 💡 Design Philosophy

`catpack` was built with a few core principles in mind:

- **Zero Runtime Dependencies:** Written in Rust to provide a single, static binary.
- **Offline First:** Token counting is performed locally using embedded BPE tables. No API keys or network requests are required to estimate your context size.
- **Terminal Native:** First-class support for terminal multiplexers. The `--clip` flag intelligently uses OSC 52 escape sequences, making it work seamlessly in Zellij, tmux, or SSH sessions without relying on system clipboard tools like `xclip`.
- **Honest Transparency:** We prioritize reliability and features (like offline token counting and cross-platform clipboard support) over minimal binary size, resulting in a ~9.5 MB executable that "just works" out of the box.

## 🤝 Contributing

Pull requests are welcome! For detailed build instructions, cross-compilation, and release guidelines, please see [CONTRIBUTING.md](CONTRIBUTING.md).

## 📄 License

This project is released into the public domain under the [Unlicense](https://unlicense.org/).

## 🙏 Acknowledgments

- [ripgrep](https://github.com/BurntSushi/ripgrep) — for the `ignore` crate
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) — for token counting
- [arboard](https://github.com/ArturKovacs/arboard) — for cross-platform clipboard support
