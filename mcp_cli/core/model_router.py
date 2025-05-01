import requests
import openai
import json
from core.config import MODEL_BACKEND, OLLAMA_MODEL, OPENAI_MODEL, CLAUDE_MODEL, OPENAI_API_KEY, CLAUDE_API_KEY

def query_model(prompt):
    if MODEL_BACKEND == "ollama":
        response = requests.post("http://localhost:11434/api/generate", json={
            "model": OLLAMA_MODEL,
            "prompt": prompt,
            "stream": False
        })
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
        raise ValueError(f"Unsupported MODEL_BACKEND: {MODEL_BACKEND}")