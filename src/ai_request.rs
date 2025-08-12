use serde::{Deserialize, Serialize};

use crate::{game_prompt::GamePrompt, game_state::GameState, gpt_model::GptModel};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String, // "system", "user", "assistant"
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub format_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
}

// Struct for the game response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResponse {
    pub story: String,
    pub new_items: Vec<String>,
    pub location: String,
    pub choices: Vec<String>,
    pub summary: String,
}

impl Message {
    pub fn system(content: &str) -> Self {
        Message {
            role: "system".to_string(),
            content: content.to_string(),
        }
    }
    
    pub fn user(content: &str) -> Self {
        Message {
            role: "user".to_string(),
            content: content.to_string(),
        }
    }
}

pub struct AiRequest {
    client: reqwest::Client,
    api_key: String,
    model: GptModel,
}

impl AiRequest {
    pub fn new(api_key: String, model: GptModel) -> Self {
        AiRequest {
            client: reqwest::Client::new(),
            api_key,
            model,
        }
    }

    
    pub async fn json_chat(&self, game_state: &GameState) -> Result<GameResponse, Box<dyn std::error::Error>> {
        let system_prompt = GamePrompt::get_system_prompt();
        
        let user_prompt = GamePrompt::build_game_context_prompt(game_state);
        
        let messages = vec![
            Message::system(system_prompt),
            Message::user(&user_prompt),
        ];
        
        let request = ChatRequest {
            model: self.model.into(),
            messages,
            max_tokens: Some(3000),
            temperature: Some(0.9),
            response_format: Some(ResponseFormat {
                format_type: "json_object".to_string(),
            }),
        };
        
        let response_text = self.send_request(&request).await?;
        let game_response: GameResponse = serde_json::from_str(&response_text)?;
        Ok(game_response)
    }
        
    
    async fn send_request(&self, request: &ChatRequest) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://api.openai.com/v1/chat/completions";
        
        let response = self.client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API Error: {}", error_text).into());
        }
        
        let api_response: ApiResponse = response.json().await?;
        
        if let Some(choice) = api_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("No response from AI".into())
        }
    }
}