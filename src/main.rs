use crate::{game_engine::GameEngine, gpt_model::GptModel, config::Config};
use clap::Parser;

mod config;
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
    #[arg(long)]
    model: Option<String>,
    #[arg(long, env = "OPENAI_BASE_URL")]
    base_url: Option<String>,
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
    // Load configuration from file
    let mut config = Config::load();
    
    // Parse CLI arguments
    let args = Args::parse();
    
    // Merge CLI args with config (CLI takes precedence)
    config.merge_with_cli(args.api_key, args.model, args.base_url);
    
    let api_key = config.api.api_key.clone().unwrap_or_default();
    let base_url = config.api.base_url.clone();
    let model: GptModel = config.api.model.clone().into();

    // Check if the API key is valid
    if let Err(e) = check_api_key(if api_key.is_empty() { None } else { Some(&api_key) }, &base_url).await {
        return Err(e);
    }

    let mut game = GameEngine::new(api_key, model, base_url, config);
    game.start_game().await?;
    
    Ok(())
}