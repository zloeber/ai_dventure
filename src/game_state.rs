use crate::ai_request::GameResponse;

pub struct GameState {
    pub current_location: String,
    pub inventory: Vec<String>,
    pub visited_locations: Vec<String>,
    pub name: String,
    pub complete_story: String,
    pub summary: String,
    pub last_response: Option<String>,
    pub current_prompt: String,
}

impl GameState {
    pub fn new(name: String) -> Self {
        GameState {
            current_location: "Unknown".to_string(),
            inventory: Vec::new(),
            visited_locations: Vec::new(),
            name,
            complete_story: String::new(),
            summary: String::new(),
            last_response: None,
            current_prompt: String::new(),
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.inventory.push(item);
    }

    pub fn visit_location(&mut self, location: String) {
        if !self.visited_locations.contains(&location) {
            self.visited_locations.push(location);
        }
    }

    pub fn update_story(&mut self, new_story: String) {
        self.complete_story.push_str(&new_story);
    }

    pub fn update_from_response(&mut self, response: &GameResponse) {
        self.current_location = response.location.clone();
        self.visit_location(response.location.clone());
        
        // Aggiunge nuovi oggetti all'inventario
        for item in &response.new_items {
            self.add_item(item.clone());
        }
        
        self.summary = response.summary.clone();
        
        self.last_response = Some(response.story.clone());
        
        self.update_story(format!("\n{}", response.story));
    }

    pub fn set_current_prompt(&mut self, prompt: String) {
        self.current_prompt = prompt;
    }

    pub fn display_status(&self, prompts_config: &crate::config::PromptsConfig) {
        use colored::*;
        use crate::game_prompt::GamePrompt;
        
        println!("\n{}", GamePrompt::display_status_header(prompts_config));
        let labels = GamePrompt::get_status_labels();
        
        println!("{} {}", labels.0, self.name.green());
        println!("{} {}", labels.1, self.current_location.blue());
        
        println!("\n{}", labels.2);
        if self.inventory.is_empty() {
            println!("  {}", labels.3);
        } else {
            for (i, item) in self.inventory.iter().enumerate() {
                println!("  {}. {}", i + 1, item);
            }
        }

        
        println!("{}", GamePrompt::get_status_separator());
    }

}