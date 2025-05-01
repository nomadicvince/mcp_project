use std::collections::HashMap;

pub struct Context {
    pub user_id: String,
    pub memory: HashMap<String, Vec<String>>,
}

impl Context {
    pub fn new(user_id: &str) -> Self {
        Context {
            user_id: user_id.to_string(),
            memory: HashMap::new(),
        }
    }

    pub fn add_to_memory(&mut self, key: &str, value: String) {
        self.memory.entry(key.to_string()).or_default().push(value);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.memory).unwrap()
    }
}