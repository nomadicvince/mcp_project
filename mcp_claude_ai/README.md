# Model Context Protocol Server

A Rust-based server implementing the Model Context Protocol with Claude 3.7 Sonnet integration. This project includes a backend using Axum and SurrealDB for context storage and a simple HTML/CSS/JS frontend for interacting with the API.

## Features

- **Model Context Protocol Implementation**: Provides a standardized way to interact with language models with contextual information
- **Claude 3.7 Sonnet Integration**: Uses Anthropic's Claude 3.7 Sonnet model for text generation
- **SurrealDB for Context Storage**: Store and retrieve context items using SurrealDB
- **Simple Frontend**: HTML/CSS/JS frontend for searching contexts and generating text

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [SurrealDB](https://surrealdb.com/install)
- Anthropic API key for Claude 3.7 Sonnet

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/model_context_server.git
   cd model_context_server
   ```

2. Create a `.env` file in the project root with your configuration:
   ```bash
   cp .env.example .env
   # Edit the .env file with your settings, especially your Anthropic API key
   ```

3. Start SurrealDB (in a separate terminal):
   ```bash
   surreal start --log debug --user root --pass root --bind 127.0.0.1:8080 memory
   ```

4. Build and run the server:
   ```bash
   cargo run
   ```

5. Open your browser and navigate to `http://localhost:3030`

## API Endpoints

- `GET /api/contexts` - Get all context items
- `GET /api/contexts/:id` - Get a specific context item by ID
- `POST /api/contexts` - Create a new context item
- `GET /api/search?query=text` - Search for context items
- `POST /api/generate` - Generate text with Claude 3.7 Sonnet

## Using the Frontend

### Search for Contexts
1. Enter a search term in the search box
2. Click "Search"
3. Results will be displayed below
4. Click "Use in Generation" to add a context to your generation prompt

### Generate Text with Claude
1. Enter your prompt in the text area
2. Adjust temperature and max tokens settings as needed
3. Add context items from search results
4. Click "Generate"
5. The generated text will be displayed below

### Add New Context
1. Enter an optional ID for the context (one will be generated if not provided)
2. Enter the context content
3. Optionally add metadata in JSON format
4. Click "Add Context"

## Project Structure

```
model_context_server/
├── .env                       # Environment variables (gitignored)
├── .env.example               # Example environment variables file
├── Cargo.toml                 # Rust dependencies and project metadata
├── src/
│   └── main.rs                # Rust server application code
└── static/                    # Frontend static files
    ├── index.html             # HTML frontend
    ├── styles.css             # CSS styles
    └── app.js                 # JavaScript application logic
```

## Claude 3.7 Sonnet Model

The integration uses [Claude 3.7 Sonnet](https://www.anthropic.com/claude) by Anthropic, a powerful AI assistant model that's capable of understanding and generating nuanced text across a wide range of topics and formats.

Features of Claude 3.7 Sonnet:
- Excellent at understanding complex instructions
- High-quality, nuanced text generation
- Good reasoning capabilities
- Context-aware responses
- Helpful, harmless, and honest output

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a pull request.
