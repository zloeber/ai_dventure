# 🧙‍♂️ AI-Dventure: Interactive AI-Powered Text Adventure Game

Welcome to **AI-Dventure**! 🚀

Embark on a thrilling journey through worlds of fantasy, horror, sci-fi, or history, all narrated by a powerful AI. Your choices shape the story, your inventory grows, and your adventure is unique every time!

![Screenshot showing AI-Dventure's interactive gameplay with story text, player choices, and inventory system](screenshot.png)

## Features ✨
- **🤖 AI Storyteller:** Powered by OpenAI's GPT models to create immersive, dynamic narratives that adapt to your choices
- **🎯 Dynamic Choices:** Choose from suggested options OR type your own custom actions - the AI will seamlessly incorporate your creativity into the story
- **🎒 Persistent Game State:** Your inventory, current location, and story progress are continuously tracked and influence future events
- **🌍 Rich Themes:** Select from fantasy, horror, sci-fi, historical settings, or let the AI surprise you with a unique theme
- **💬 Natural Language Input:** Communicate with the game using natural language - no need to memorize specific commands

## How to Play 🎮
1. **Start the game** and enter your adventurer's name
2. **Choose your adventure theme** from fantasy, horror, sci-fi, historical, or let the AI surprise you
3. **Read the generated story** and decide your next move:
   - Select one of the numbered choices provided by the AI **OR**
   - Type your own creative action to take the story in an unexpected direction
4. **Continue the adventure** as the AI responds to your choices, updating your inventory and story
5. **Use special commands** like `status` to check your current state, or `quit` to exit gracefully

## Getting Started 🚀

### Prerequisites
- **Rust** (latest stable version recommended)
- **OpenAI API Key** - Get yours from [OpenAI's website](https://openai.com/api/)

### Installation & Running

#### Using OpenAI
```bash
# Clone the repository
git clone https://github.com/your-username/ai_dventure.git
cd ai_dventure

# Run with your API key
cargo run -- --api-key <YOUR_API_KEY>

# Or set it as an environment variable
export OPENAI_API_KEY=<YOUR_API_KEY>
cargo run
```

#### Using Local OLLAMA
```bash
# Make sure OLLAMA is running locally
# Install from: https://ollama.ai

# Pull a model (if not already done)
ollama pull llama3

# Run the game with OLLAMA
cargo run -- --base-url http://localhost:11434/v1 --model llama3

# Or with environment variable
export OPENAI_BASE_URL=http://localhost:11434/v1
cargo run -- --model llama3

# Try other models
cargo run -- --base-url http://localhost:11434/v1 --model mistral
cargo run -- --base-url http://localhost:11434/v1 --model codellama
```





## Example Gameplay
```
=== ADVENTURE BEGINS ===
You stand at the gates of a mysterious ancient castle. The wind howls through 
the twisted iron bars, and the pale moon casts eerie shadows across the 
crumbling stone walls. A faint light flickers in one of the tower windows.

Available choices:
1. Push open the heavy castle gates and enter
2. Circle around to find another entrance
3. Call out to whoever might be in the tower
4. Search the surrounding area for clues

What do you do? (type your choice or 'quit' to exit): I examine the gates for traps
```

## Why AI-Dventure is Engaging 🎭
- **🔄 Infinite Replayability:** Every playthrough is unique - the AI generates fresh content based on your choices
- **🎨 Creative Freedom:** Don't feel constrained by the given options - describe any action and watch the AI weave it into the narrative
- **📚 Rich Storytelling:** Experience detailed, atmospheric descriptions that bring each scene to life
- **🎲 Unpredictable Adventures:** The story can take unexpected turns based on your creativity and the AI's imagination

## Requirements
- Rust
- OpenAI API key (ENV variable `OPENAI_API_KEY`) - or local OLLAMA installation

## Command line arguments
The application accepts the following parameters:
- `--api-key`: specify custom key if not defined in env (optional for local endpoints like OLLAMA)
- `--model`: choose model name (e.g. `gpt-3.5-turbo`, `gpt-4o-mini`, `llama3`, `mistral`, etc.). If not specified `gpt-4o-mini` is used
- `--base-url`: specify custom API base URL (ENV variable `OPENAI_BASE_URL`). Default: `https://api.openai.com/v1`

Example:
```bash
# Run in development mode with OpenAI
cargo run

# With OpenAI parameters
cargo run -- --api-key <YOUR_API_KEY> --model gpt-4o-mini

# With local OLLAMA endpoint
cargo run -- --base-url http://localhost:11434/v1 --model llama3

# Using environment variables for OLLAMA
export OPENAI_BASE_URL=http://localhost:11434/v1
cargo run -- --model llama3

# Build a release binary
cargo build --release

# Run the release binary with OLLAMA
./target/release/ai_dventure --base-url http://localhost:11434/v1 --model llama3
```

## License
[*MIT*](LICENSE)

---
Ready to shape your own legend? 🏰🦄👽
