use crate::context::Context;
use crate::agent::Agent;
use crate::db::{load_memory, save_memory};

pub struct Orchestrator {
    agents: Vec<Box<dyn Agent + Send + Sync>>,
}

impl Orchestrator {
    pub fn new(agents: Vec<Box<dyn Agent + Send + Sync>>) -> Self {
        Orchestrator { agents }
    }

    pub async fn handle_request(&mut self, input: String) -> String {
        let user_id = "local_user";
        let mem = load_memory(user_id);
        let mut context = Context::new(user_id);
        for (k, v) in mem {
            for val in v {
                context.add_to_memory(&k, val);
            }
        }

        let agent = self.agents.iter()
            .find(|a| a.matches(&input))
            .unwrap_or(&self.agents[0]);

        let response = agent.process(&mut context, input).await;
        save_memory(user_id, &context.memory);
        response
    }
}