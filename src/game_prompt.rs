use colored::*;
use crate::game_state::GameState;
use crate::config::PromptsConfig;
use textwrap;

pub struct GamePrompt {}

impl GamePrompt {
    pub fn get_system_prompt(prompts_config: &PromptsConfig) -> String {
        prompts_config.system_prompt.clone().unwrap_or_else(|| {
            r#"
        You are a creative AI storyteller for an immersive text-based adventure game. Your role is to craft engaging, detailed, and atmospheric narratives that bring the game world to life.

        STORYTELLING GUIDELINES:
        - Create vivid, immersive descriptions (1-3 paragraphs)
        - Develop rich atmospheric details about locations, characters, and situations
        - Include sensory details (sights, sounds, smells, textures)
        - Build tension and intrigue to keep the player engaged
        - Make each story segment feel like a chapter in an exciting novel
        - Respond to player actions with meaningful consequences
        - Introduce interesting NPCs with personalities and dialogue when appropriate
        - Create compelling scenarios that require thoughtful decision-making

        RESPONSE FORMAT:
        Always respond with valid JSON in this exact structure:
        {
            "story": "Rich, detailed narrative text (about 1-3 paragraphs max describing the scene, action, and atmosphere)",
            "new_items": ["item1", "item2"], // Only include if player actually gains items
            "location": "Descriptive location name",
            "choices": ["Choice 1", "Choice 2", "Choice 3"], // 2-4 meaningful choices
            "summary": "Updated summary including the new events"
        }

        The "story" field should be substantial and engaging. Paint a picture with words and make the player feel immersed in the adventure.
        "#.to_string()
        })
    }

    pub fn build_game_context_prompt(game_state: &GameState) -> String {
        format!(
            "GAME CONTEXT:\n\
            Player Name: {}\n\
            Current Location: {}\n\
            Inventory: {:?}\n\
            Visited Locations: {:?}\n\n\
            STORY SUMMARY:\n\
            {}\n\n\
            LAST AI RESPONSE:\n\
            {}\n\n\
            CURRENT PLAYER ACTION:\n\
            {}\n\n\
            Please continue the story based on the player's action and respond with the JSON format specified in the system prompt.",
            game_state.name,
            game_state.current_location,
            game_state.inventory,
            game_state.visited_locations,
            game_state.summary,
            game_state.last_response.as_ref().unwrap_or(&"[Game Start]".to_string()),
            game_state.current_prompt
        )
    }

    pub fn get_welcome_message(prompts_config: &PromptsConfig) -> String {
        prompts_config.welcome_message.clone().unwrap_or_else(|| {
            format!("{}\n{}", 
                "\n=== AI ADVENTURE ===".cyan().bold(),
                "Welcome to the AI-powered adventure game!".green()
            )
        })
    }

    pub fn get_player_name_prompt(prompts_config: &PromptsConfig) -> String {
        prompts_config.player_name_prompt.clone().unwrap_or_else(|| {
            "What's your name, adventurer? ".cyan().to_string()
        })
    }

    pub fn get_theme_selection_prompt(prompts_config: &PromptsConfig) -> String {
        prompts_config.theme_selection_prompt.clone().unwrap_or_else(|| {
            let title = "Select the theme of your adventure:".cyan().bold();
            let options = vec![
                "1. Fantasy - Dragons, magic, and enchanted realms".green(),
                "2. Dark/Horror - Cosmic horror and dark mysteries".red(),
                "3. Sci-Fi - Space, technology, and aliens".blue(),
                "4. Historical - Swords, duels, and intrigue".yellow(),
                "5. Surprise me - Let the AI choose for you".magenta(),
                "6. You choose - Custom theme".cyan()
            ];

            let mut prompt = format!("{}\n", title);
            for option in options {
                prompt.push_str(&format!("{}\n", option));
            }
            prompt.push_str("Please enter the number of your choice: ");
            prompt
        })
    }

    pub fn get_theme_selected_message(theme: &str, prompts_config: &PromptsConfig) -> String {
        prompts_config.theme_selected_message.clone().unwrap_or_else(|| {
            format!("{} {}", "Theme selected:".green(), theme.yellow())
        })
    }

    pub fn get_initial_story_prompt(player_name: &str, theme: &str) -> String {
        format!(
            "Create an engaging opening for a text-based adventure game.\n\
            Player name: {}\n\
            Theme: {}\n\
            Start the adventure with an interesting scenario that requires player choice.\n\
            Provide 3 choices for the player.\n\
            Respond in the specified JSON format.",
            player_name, theme
        )
    }

    pub fn get_adventure_start_header(prompts_config: &PromptsConfig) -> String {
        prompts_config.adventure_start_header.clone().unwrap_or_else(|| {
            "=== ADVENTURE BEGINS ===".cyan().bold().to_string()
        })
    }

    pub fn get_adventure_continues_header(prompts_config: &PromptsConfig) -> String {
        prompts_config.adventure_continues_header.clone().unwrap_or_else(|| {
            "=== ADVENTURE CONTINUES ===".cyan().bold().to_string()
        })
    }

    pub fn get_choices_header(prompts_config: &PromptsConfig) -> String {
        prompts_config.choices_header.clone().unwrap_or_else(|| {
            "Available choices:".yellow().to_string()
        })
    }

    pub fn get_player_input_prompt(prompts_config: &PromptsConfig) -> String {
        prompts_config.player_input_prompt.clone().unwrap_or_else(|| {
            "What do you do? (type your choice or 'quit' to exit): ".cyan().to_string()
        })
    }

    pub fn get_quit_message(prompts_config: &PromptsConfig) -> String {
        prompts_config.quit_message.clone().unwrap_or_else(|| {
            "Thanks for playing! Goodbye!".red().to_string()
        })
    }

    pub fn display_status_header(prompts_config: &PromptsConfig) -> String {
        prompts_config.status_header.clone().unwrap_or_else(|| {
            "=== PLAYER STATUS ===".cyan().bold().to_string()
        })
    }

    pub fn get_status_labels() -> (String, String, String, String, String) {
        (
            "Name:".to_string(),
            "Location:".to_string(), 
            "Inventory:".cyan().to_string(),
            "Empty".dimmed().to_string(),
            "Visited locations:".cyan().to_string()
        )
    }

    pub fn get_status_separator() -> String {
        "=".repeat(30).cyan().to_string()
    }

    pub fn get_ai_thinking_message(prompts_config: &PromptsConfig) -> String {
        prompts_config.ai_thinking_message.clone().unwrap_or_else(|| {
            "🤖  AI is cooking... ⏳".yellow().italic().to_string()
        })
    }

    pub fn format_story(story: &str) -> String {
        let wrapped = textwrap::wrap(story, 80);
        wrapped.join("\n")
    }
}