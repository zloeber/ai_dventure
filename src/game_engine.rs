use crate::gpt_model::GptModel;
use crate::{ai_request::AiRequest, game_state::GameState, config::Config};
use crate::game_prompt::GamePrompt;
use std::io::{self, Write};
use colored::*;

pub struct GameEngine {
    ai_client: AiRequest,
    game_state: Option<GameState>,
    config: Config,
}

impl GameEngine {
    pub fn new(api_key: String, model: GptModel, base_url: String, config: Config) -> Self {
        GameEngine {
            ai_client: AiRequest::new(api_key, model, base_url),
            game_state: None,
            config,
        }
    }

    pub async fn start_game(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", GamePrompt::get_welcome_message(&self.config.prompts));
        
        // Get player's name
        let player_name = self.get_player_name();
        
        // Select theme
        let theme = self.select_theme()?;
        
        self.game_state = Some(GameState::new(player_name));
        
        // Get first AI response
        if let Some(ref mut state) = self.game_state {
            state.set_current_prompt(GamePrompt::get_initial_story_prompt(&state.name, &theme));
            
            println!("\n{}", GamePrompt::get_ai_thinking_message(&self.config.prompts));
            let response = self.ai_client.json_chat(state, &self.config.prompts).await?;
            
            // Show the initial story
            println!("\n{}", GamePrompt::get_adventure_start_header(&self.config.prompts));
            println!("{}", GamePrompt::format_story(&response.story).white());
            
            if !response.choices.is_empty() {
                println!("\n{}", GamePrompt::get_choices_header(&self.config.prompts));
                for (i, choice) in response.choices.iter().enumerate() {
                    println!("{}. {}", i + 1, choice.green());
                }
            }
            
            // Update game state
            state.update_from_response(&response);
        }
        
        // Main loop
        self.game_loop().await?;
        
        Ok(())
    }

    fn get_player_name(&self) -> String {
        print!("{}", GamePrompt::get_player_name_prompt(&self.config.prompts));
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn select_theme(&self) -> Result<String, Box<dyn std::error::Error>> {
        println!("\n{}", GamePrompt::get_theme_selection_prompt(&self.config.prompts));
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut custom_theme = String::new();
        
        let themes = &self.config.prompts.themes;
        let theme = match input.trim() {
            "1" if themes.len() > 0 => themes[0].as_str(),
            "2" if themes.len() > 1 => themes[1].as_str(),
            "3" if themes.len() > 2 => themes[2].as_str(),
            "4" if themes.len() > 3 => themes[3].as_str(),
            "5" if themes.len() > 4 => themes[4].as_str(),
            "6" => {
                print!("Enter your custom theme: ");
                io::stdout().flush().unwrap();
                
                io::stdin().read_line(&mut custom_theme)?;
                custom_theme.trim()
            },
            _ if !themes.is_empty() => themes[0].as_str(), // Default to first theme
            _ => "Medieval fantasy with dragons, magic, elves and enchanted kingdoms",
        };
        
        println!("\n{}", GamePrompt::get_theme_selected_message(theme, &self.config.prompts));
        Ok(theme.to_string())
    }

    async fn game_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if let Some(ref game_state) = self.game_state {
                // Display current game status
                game_state.display_status(&self.config.prompts);
            }
            
            // Player input
            let player_choice = self.get_player_input();
            
            // Special commands
            match player_choice.to_lowercase().as_str() {
                "quit" | "exit" => {
                    println!("{}", GamePrompt::get_quit_message(&self.config.prompts));
                    break;
                },
                "status" => {
                    continue; // Status is already shown above
                },
                _ => {}
            }
            
            // Process choice with AI
            self.process_player_action(player_choice).await?;
        }
        
        Ok(())
    }

    fn get_player_input(&self) -> String {
        print!("\n{}", GamePrompt::get_player_input_prompt(&self.config.prompts));
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    async fn process_player_action(&mut self, action: String) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut game_state) = self.game_state {
            
            game_state.set_current_prompt(action);
            
            // Show AI thinking message
            println!("\n{}", GamePrompt::get_ai_thinking_message(&self.config.prompts));
            
            // Get AI response
            let response = self.ai_client.json_chat(game_state, &self.config.prompts).await?;
            
            // Show the new story
            println!("\n{}", GamePrompt::get_adventure_continues_header(&self.config.prompts));
            println!("{}", GamePrompt::format_story(&response.story).white());
            
            if !response.choices.is_empty() {
                println!("\n{}", GamePrompt::get_choices_header(&self.config.prompts));
                for (i, choice) in response.choices.iter().enumerate() {
                    println!("{}. {}", i + 1, choice.green());
                }
            }
            
            // Update game state
            game_state.update_from_response(&response);
        }
        
        Ok(())
    }
}