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
    #[arg(long)]
    api_key: String,
    #[arg(long, default_value = "gpt-4o-mini")]
    model: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let api_key = args.api_key;
    let model: GptModel = args.model.into();
    let mut game = GameEngine::new(api_key, model);
    game.start_game().await?;
    
    Ok(())
}