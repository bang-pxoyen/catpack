use anyhow::Result;
use tiktoken_rs::CoreBPE;

pub enum Model {
    GPT4,
    GPT4o,
    Claude,
}

impl Model {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "gpt-4o" | "gpt4o" => Model::GPT4o,
            "claude" | "anthropic" => Model::Claude,
            _ => Model::GPT4,
        }
    }
    
    pub fn encoding(&self) -> Result<CoreBPE> {
        match self {
            Model::GPT4 | Model::Claude => {
                Ok(tiktoken_rs::get_bpe_from_model("gpt-4")?)
            }
            Model::GPT4o => {
                Ok(tiktoken_rs::get_bpe_from_model("gpt-4o")?)
            }
        }
    }
}

/// Counts the number of tokens in the text
pub fn count_tokens(text: &str, model: &Model) -> Result<usize> {
    let bpe = model.encoding()?;
    Ok(bpe.encode_with_special_tokens(text).len())
}

/// Formats token count for output
pub fn format_tokens(count: usize) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_from_str() {
        assert!(matches!(Model::from_str("gpt-4o"), Model::GPT4o));
        assert!(matches!(Model::from_str("GPT4O"), Model::GPT4o));
        assert!(matches!(Model::from_str("claude"), Model::Claude));
        assert!(matches!(Model::from_str("anthropic"), Model::Claude));
        assert!(matches!(Model::from_str("unknown-model"), Model::GPT4));
    }

    #[test]
    fn test_count_tokens_basic() {
        let model = Model::GPT4;
        // "hello world" is typically 2 tokens in cl100k_base
        let count = count_tokens("hello world", &model).unwrap();
        assert!(count >= 2); 
    }
}
