use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

mod tree;
mod clipboard;
mod tokens;
mod osc52;

use tokens::Model;

const TOOL_NAME: &str = "catpack";

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    Simple,
    Markdown,
    Xml,
    Separator,
}

#[derive(Parser, Debug)]
#[command(
    name = TOOL_NAME,
    about = "Pack your codebase into a single file for LLM context",
    version
)]
struct Args {
    #[arg(short, long, default_value = "catpack.txt")]
    output: String,

    #[arg(short, long, action = clap::ArgAction::Append)]
    ignore: Vec<String>,

    #[arg(short, long, default_value = ".")]
    dir: String,

    #[arg(short, long, value_enum, default_value = "simple")]
    format: Format,

    #[arg(long)]
    no_lang: bool,

    #[arg(long)]
    tree: bool,

    #[arg(long)]
    clip: bool,

    #[arg(long)]
    osc52: bool,

    #[arg(long, default_value = "gpt-4")]
    model: String,

    #[arg(long)]
    tokens_only: bool,
}

/// Detects programming language by file extension
fn detect_language(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("py") => "python",
        Some("js") => "javascript",
        Some("ts") => "typescript",
        Some("jsx") => "jsx",
        Some("tsx") => "tsx",
        Some("html") | Some("htm") => "html",
        Some("css") => "css",
        Some("scss") | Some("sass") => "scss",
        Some("json") => "json",
        Some("yaml") | Some("yml") => "yaml",
        Some("md") => "markdown",
        Some("sh") | Some("bash") => "bash",
        Some("zsh") => "zsh",
        Some("sql") => "sql",
        Some("rs") => "rust",
        Some("go") => "go",
        Some("java") => "java",
        Some("c") | Some("h") => "c",
        Some("cpp") | Some("cc") | Some("cxx") | Some("hpp") => "cpp",
        Some("rb") => "ruby",
        Some("php") => "php",
        Some("swift") => "swift",
        Some("kt") | Some("kts") => "kotlin",
        Some("xml") => "xml",
        Some("toml") => "toml",
        Some("ini") | Some("cfg") => "ini",
        Some("txt") => "text",
        Some("lua") => "lua",
        Some("r") => "r",
        _ => "",
    }
}

/// Checks if a file is binary by looking for null bytes in the first 1024 bytes
fn is_binary(path: &Path) -> bool {
    let Ok(bytes) = fs::read(path) else {
        return true;
    };
    bytes.iter().take(1024).any(|&b| b == 0)
}

/// Formats file content based on the chosen output format
fn format_file(path: &str, content: &str, lang: &str, format: &Format) -> String {
    match format {
        Format::Simple => format!("\n\n===== {} =====\n\n{}\n", path, content),

        Format::Markdown => {
            let lang_tag = if lang.is_empty() { "" } else { lang };
            format!("\n\n## 📄 `{}`\n\n```{}\n{}\n```\n", path, lang_tag, content)
        }

        Format::Xml => {
            let lang_attr = if lang.is_empty() {
                String::new()
            } else {
                format!(" lang=\"{}\"", lang)
            };
            let safe_content = content.replace("</file>", "<\\/file>");
            format!(
                "\n\n<file path=\"{}\"{}>\n{}\n</file>\n",
                path, lang_attr, safe_content
            )
        }

        Format::Separator => {
            let sep = "=".repeat(60);
            let lang_info = if lang.is_empty() {
                String::new()
            } else {
                format!(" [{}]", lang)
            };
            format!(
                "\n\n{}\n📄 {}{}\n{}\n\n{}\n\n{}\n",
                sep, path, lang_info, sep, content, sep
            )
        }
    }
}

/// Builds ignore rules from file and CLI arguments
fn build_ignore(dir: &Path, extra_ignores: &[String], output_file: &str) -> Result<Gitignore> {
    let mut builder = GitignoreBuilder::new(dir);

    let ignore_file = dir.join(".catpackignore");
    if ignore_file.exists() {
        builder.add(ignore_file);
    }

    builder.add_line(None, output_file)?;
    builder.add_line(None, ".git/")?;
    builder.add_line(None, ".gitignore")?;
    builder.add_line(None, ".catpackignore")?;

    for pattern in extra_ignores {
        builder.add_line(None, pattern)?;
    }

    Ok(builder.build()?)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let target_dir = PathBuf::from(&args.dir)
        .canonicalize()
        .with_context(|| format!("Directory not found: {}", args.dir))?;

    if !target_dir.is_dir() {
        anyhow::bail!("{} is not a directory", target_dir.display());
    }

    let ignore = build_ignore(&target_dir, &args.ignore, &args.output)?;
    let model = Model::from_str(&args.model);

    println!(
        "Packing files from {} (format: {:?}, model: {})...",
        target_dir.display(),
        args.format,
        args.model
    );

    let mut output_content = String::new();
    let mut file_paths = Vec::new();
    let mut file_count = 0usize;
    let mut total_size = 0u64;
    let mut files_data = Vec::new();

    for entry in walkdir::WalkDir::new(&target_dir)
        .into_iter()
        .filter_entry(|e| {
            let rel = e.path().strip_prefix(&target_dir).unwrap_or(e.path());
            !ignore.matched(rel, e.file_type().is_dir()).is_ignore()
        })
    {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let rel_path = path.strip_prefix(&target_dir)?;

        if ignore.matched(rel_path, false).is_ignore() {
            continue;
        }

        if is_binary(path) {
            continue;
        }

        match fs::read_to_string(path) {
            Ok(content) => {
                let path_str = rel_path.to_string_lossy().replace('\\', "/");
                let lang = if args.no_lang { "" } else { detect_language(path) };
                let formatted = format_file(&path_str, &content, lang, &args.format);

                files_data.push((path_str.clone(), formatted, content.len()));
                file_paths.push(path_str);
                file_count += 1;
                total_size += content.len() as u64;
            }
            Err(e) => {
                eprintln!("Warning: cannot read {}: {}", path.display(), e);
            }
        }
    }

    if args.tree {
        output_content.push_str(&tree::generate_tree(&target_dir, &file_paths));
    }

    for (_, formatted, _) in files_data {
        output_content.push_str(&formatted);
    }

    let token_count = tokens::count_tokens(&output_content, &model)?;

    if args.tokens_only {
        println!(
            "📊 {} files, {} tokens ({}), {:.2} MB",
            file_count,
            tokens::format_tokens(token_count),
            args.model,
            total_size as f64 / (1024.0 * 1024.0)
        );
        return Ok(());
    }

    let mut out_file = fs::File::create(&args.output)
        .with_context(|| format!("Cannot create output file: {}", args.output))?;
    out_file.write_all(output_content.as_bytes())?;

    if args.clip {
        let copy_result = if args.osc52 {
            clipboard::copy_to_clipboard_osc52_only(&output_content)
        } else {
            clipboard::copy_to_clipboard(&output_content)
        };

        if let Err(e) = copy_result {
            eprintln!("⚠️ Warning: could not copy to clipboard: {}", e);
        }
    }

    let size_mb = total_size as f64 / (1024.0 * 1024.0);
    println!(
        "✅ Packed {} files ({:.2} MB, {} tokens) into {}",
        file_count,
        size_mb,
        tokens::format_tokens(token_count),
        args.output
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_detect_language() {
        assert_eq!(detect_language(Path::new("test.rs")), "rust");
        assert_eq!(detect_language(Path::new("test.py")), "python");
        assert_eq!(detect_language(Path::new("test.js")), "javascript");
        assert_eq!(detect_language(Path::new("test.unknown")), "");
        assert_eq!(detect_language(Path::new("no_extension")), "");
    }

    #[test]
    fn test_is_binary() {
        let temp_dir = std::env::temp_dir();
        let text_file = temp_dir.join("catpack_test_text.txt");
        let bin_file = temp_dir.join("catpack_test_bin.bin");

        fs::write(&text_file, "hello world").unwrap();
        fs::write(&bin_file, [0x00, 0x01, 0x02, 0x03]).unwrap();

        assert!(!is_binary(&text_file));
        assert!(is_binary(&bin_file));

        let _ = fs::remove_file(&text_file);
        let _ = fs::remove_file(&bin_file);
    }

    #[test]
    fn test_format_file_xml_escapes() {
        let content = "console.log('</file>');";
        let formatted = format_file("test.js", content, "javascript", &Format::Xml);
        
        assert!(formatted.contains("<file path=\"test.js\" lang=\"javascript\">"));
        assert!(formatted.contains("console.log('<\\/file>');"));
        assert!(formatted.ends_with("</file>\n"));
    }

    #[test]
    fn test_format_tokens() {
        assert_eq!(tokens::format_tokens(500), "500");
        assert_eq!(tokens::format_tokens(1500), "1.5K");
        assert_eq!(tokens::format_tokens(1_500_000), "1.5M");
    }
}
