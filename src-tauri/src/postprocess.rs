use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostProcessingRule {
    pub id: String,
    pub name: String,
    pub rule_type: RuleType,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleType {
    CapitalizeFirstWord,
    CapitalizeSentences,
    RemoveFillerWords,
    AddPunctuation,
    TrimWhitespace,
    CollapseSpaces,
    RemoveLeadingPunctuation,
    CodeFormatting,
}

impl PostProcessingRule {
    pub fn defaults() -> Vec<Self> {
        vec![
            Self {
                id: "capitalize_first".to_string(),
                name: "Capitalize first word".to_string(),
                rule_type: RuleType::CapitalizeFirstWord,
                enabled: true,
            },
            Self {
                id: "trim_whitespace".to_string(),
                name: "Trim whitespace".to_string(),
                rule_type: RuleType::TrimWhitespace,
                enabled: true,
            },
            Self {
                id: "collapse_spaces".to_string(),
                name: "Collapse multiple spaces".to_string(),
                rule_type: RuleType::CollapseSpaces,
                enabled: true,
            },
            Self {
                id: "remove_filler".to_string(),
                name: "Remove filler words (um, uh, like)".to_string(),
                rule_type: RuleType::RemoveFillerWords,
                enabled: false,
            },
            Self {
                id: "capitalize_sentences".to_string(),
                name: "Capitalize after periods".to_string(),
                rule_type: RuleType::CapitalizeSentences,
                enabled: true,
            },
            Self {
                id: "remove_leading_punct".to_string(),
                name: "Remove leading punctuation".to_string(),
                rule_type: RuleType::RemoveLeadingPunctuation,
                enabled: true,
            },
        ]
    }
}

pub fn apply_rules(text: &str, rules: &[PostProcessingRule]) -> String {
    let mut result = text.to_string();

    for rule in rules.iter().filter(|r| r.enabled) {
        result = apply_rule(&result, &rule.rule_type);
    }

    result
}

fn apply_rule(text: &str, rule_type: &RuleType) -> String {
    match rule_type {
        RuleType::TrimWhitespace => text.trim().to_string(),
        RuleType::CollapseSpaces => {
            let mut result = String::with_capacity(text.len());
            let mut prev_space = false;
            for ch in text.chars() {
                if ch == ' ' {
                    if !prev_space {
                        result.push(ch);
                    }
                    prev_space = true;
                } else {
                    result.push(ch);
                    prev_space = false;
                }
            }
            result
        }
        RuleType::CapitalizeFirstWord => {
            let mut chars = text.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        }
        RuleType::CapitalizeSentences => {
            let mut result = String::with_capacity(text.len());
            let mut capitalize_next = true;
            for ch in text.chars() {
                if capitalize_next && ch.is_alphabetic() {
                    result.extend(ch.to_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(ch);
                    if ch == '.' || ch == '!' || ch == '?' {
                        capitalize_next = true;
                    }
                }
            }
            result
        }
        RuleType::RemoveFillerWords => {
            let fillers = ["um", "uh", "like", "you know", "basically", "actually", "literally"];
            let mut result = text.to_string();
            for filler in &fillers {
                let pattern = format!(r"(?i)\b{}\b,?\s*", regex_lite::escape(filler));
                if let Ok(re) = regex_lite::Regex::new(&pattern) {
                    result = re.replace_all(&result, "").to_string();
                }
            }
            result
        }
        RuleType::RemoveLeadingPunctuation => {
            let trimmed = text.trim_start_matches(|c: char| c.is_ascii_punctuation() || c == ' ');
            trimmed.to_string()
        }
        RuleType::AddPunctuation => {
            let trimmed = text.trim_end();
            if !trimmed.is_empty()
                && !trimmed.ends_with('.')
                && !trimmed.ends_with('!')
                && !trimmed.ends_with('?')
                && !trimmed.ends_with(',')
            {
                format!("{}.", trimmed)
            } else {
                text.to_string()
            }
        }
        RuleType::CodeFormatting => text.to_string(),
    }
}
