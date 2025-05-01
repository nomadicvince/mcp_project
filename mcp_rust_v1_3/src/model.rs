use reqwest::Client;
use std::env;

pub async fn query_model(prompt: &str) -> String {
    let backend = env::var("MODEL_BACKEND").unwrap_or_else(|_| "ollama".to_string());

    match backend.as_str() {
        "ollama" => query_ollama(prompt).await,
        "openai" => query_openai(prompt).await,
        _ => "(Unsupported model backend)".to_string(),
    }
}

async fn query_ollama(prompt: &str) -> String {
    let client = Client::new();
    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "mistral",
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await;

    match res {
        Ok(r) => match r.json::<serde_json::Value>().await {
            Ok(json) => json["response"].as_str().unwrap_or("(no response)").to_string(),
            Err(_) => "(failed to parse Ollama)".to_string(),
        },
        Err(_) => "(Ollama error)".to_string(),
    }
}

async fn query_openai(prompt: &str) -> String {
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
    let client = Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&serde_json::json!({
            "model": "gpt-4",
            "messages": [{ "role": "user", "content": prompt }]
        }))
        .send()
        .await;

    match res {
        Ok(r) => match r.json::<serde_json::Value>().await {
            Ok(json) => json["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("(no content)")
                .to_string(),
            Err(_) => "(failed to parse OpenAI)".to_string(),
        },
        Err(_) => "(OpenAI error)".to_string(),
    }
}