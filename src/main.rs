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
    api_key: String,
    #[arg(long, default_value = "gpt-4o-mini")]
    model: String,
}

async fn check_api_key(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.openai.com/v1/models")
        .bearer_auth(api_key)
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
    let api_key = args.api_key;
    let model: GptModel = args.model.into();

    // Check if the API key is valid
    if let Err(e) = check_api_key(&api_key).await {
        return Err(e);
    }

    let mut game = GameEngine::new(api_key, model);
    game.start_game().await?;
    
    Ok(())
}