import requests
import openai
import json
import os

MODEL_BACKEND = os.getenv("MODEL_BACKEND", "ollama")
OLLAMA_MODEL = "mistral"
OPENAI_MODEL = "gpt-4"
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY")
CLAUDE_MODEL = "claude-3-opus-20240229"
CLAUDE_API_KEY = os.getenv("CLAUDE_API_KEY")

def query_model(prompt):
    try:
        if MODEL_BACKEND == "ollama":
            response = requests.post("http://localhost:11434/api/generate", json={
                "model": OLLAMA_MODEL,
                "prompt": prompt,
                "stream": False
            }, timeout=10)
            return response.json().get("response", "").strip()

        elif MODEL_BACKEND == "openai":
            openai.api_key = OPENAI_API_KEY
            response = openai.ChatCompletion.create(
                model=OPENAI_MODEL,
                messages=[{"role": "user", "content": prompt}]
            )
            return response["choices"][0]["message"]["content"].strip()

        elif MODEL_BACKEND == "claude":
            headers = {
                "x-api-key": CLAUDE_API_KEY,
                "content-type": "application/json"
            }
            data = {
                "model": CLAUDE_MODEL,
                "messages": [{"role": "user", "content": prompt}],
                "max_tokens": 1024,
                "temperature": 0.7
            }
            response = requests.post("https://api.anthropic.com/v1/messages", headers=headers, data=json.dumps(data))
            return response.json()["content"][0]["text"].strip()

        else:
            return f"[ERROR] Unsupported MODEL_BACKEND: {MODEL_BACKEND}"

    except Exception as e:
        return f"[ERROR calling model backend] {e}"