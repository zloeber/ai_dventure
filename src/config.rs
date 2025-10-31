use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub api: ApiConfig,
    #[serde(default)]
    pub game: GameConfig,
    #[serde(default)]
    pub prompts: PromptsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_game_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_name_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_selection_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_selected_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adventure_start_header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adventure_continues_header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices_header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_input_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quit_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_thinking_message: Option<String>,
    #[serde(default = "default_themes")]
    pub themes: Vec<String>,
}

fn default_model() -> String {
    "gpt-4o-mini".to_string()
}

fn default_base_url() -> String {
    "https://api.openai.com/v1".to_string()
}

fn default_themes() -> Vec<String> {
    vec![
        "Medieval fantasy with dragons, magic, elves and enchanted kingdoms".to_string(),
        "Dark horror with mysterious creatures, grim atmospheres and suspense".to_string(),
        "Science fiction with spaceships, aliens, advanced technology and alien planets".to_string(),
        "Historical with knights, duels, court intrigue and realistic settings".to_string(),
        "Surprise theme chosen by AI".to_string(),
    ]
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            api_key: None,
            model: default_model(),
            base_url: default_base_url(),
        }
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            save_game_path: None,
        }
    }
}

impl Default for PromptsConfig {
    fn default() -> Self {
        PromptsConfig {
            system_prompt: None,
            welcome_message: None,
            player_name_prompt: None,
            theme_selection_prompt: None,
            theme_selected_message: None,
            adventure_start_header: None,
            adventure_continues_header: None,
            choices_header: None,
            player_input_prompt: None,
            quit_message: None,
            status_header: None,
            ai_thinking_message: None,
            themes: default_themes(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api: ApiConfig::default(),
            game: GameConfig::default(),
            prompts: PromptsConfig::default(),
        }
    }
}

impl Config {
    /// Load configuration from a YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    /// Load configuration from config.yml if it exists, otherwise return default
    pub fn load() -> Self {
        let config_path = "config.yml";
        if Path::new(config_path).exists() {
            match Self::from_file(config_path) {
                Ok(config) => {
                    println!("✓ Loaded configuration from config.yml");
                    config
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load config.yml: {}. Using defaults.", e);
                    Config::default()
                }
            }
        } else {
            Config::default()
        }
    }

    /// Merge with CLI arguments (CLI args take precedence)
    pub fn merge_with_cli(&mut self, api_key: Option<String>, model: Option<String>, base_url: Option<String>) {
        if let Some(key) = api_key {
            self.api.api_key = Some(key);
        }
        if let Some(m) = model {
            self.api.model = m;
        }
        if let Some(url) = base_url {
            self.api.base_url = url;
        }
    }
}
