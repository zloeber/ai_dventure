use crate::{game_engine::GameEngine, gpt_model::GptModel};
use clap::Parser;

mod game_prompt;
mod ai_request;
mod game_state;
mod game_engine;
mod gpt_model;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, env = "OPENAI_API_KEY")]
    api_key: Option<String>,
    #[arg(long, default_value = "gpt-4o-mini")]
    model: String,
    #[arg(long, env = "OPENAI_BASE_URL", default_value = "https://api.openai.com/v1")]
    base_url: String,
}

async fn check_api_key(api_key: Option<&String>, base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Skip validation if API key is not provided (for local endpoints that don't require auth)
    let Some(key) = api_key else {
        return Ok(());
    };
    
    let client = reqwest::Client::new();
    let models_url = format!("{}/models", base_url);
    let response = client
        .get(&models_url)
        .bearer_auth(key)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(format!("Invalid API key: {}", response.status()).into()),
        
    }

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let api_key = args.api_key.unwrap_or_default();
    let base_url = args.base_url;
    let model: GptModel = args.model.into();

    // Check if the API key is valid
    if let Err(e) = check_api_key(if api_key.is_empty() { None } else { Some(&api_key) }, &base_url).await {
        return Err(e);
    }

    let mut game = GameEngine::new(api_key, model, base_url);
    game.start_game().await?;
    
    Ok(())
}