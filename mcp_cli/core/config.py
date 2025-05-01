import os

MODEL_BACKEND = "ollama"
OLLAMA_MODEL = "mistral"
OPENAI_MODEL = "gpt-4"
CLAUDE_MODEL = "claude-3-opus-20240229"
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
CLAUDE_API_KEY = os.getenv("CLAUDE_API_KEY")