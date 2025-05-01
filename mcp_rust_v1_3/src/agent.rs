use crate::context::Context;
use async_trait::async_trait;
use crate::model::query_model;

#[async_trait]
pub trait Agent {
    fn name(&self) -> &str;
    async fn process(&self, context: &mut Context, input: String) -> String;
    fn matches(&self, input: &str) -> bool;
}

pub struct SupervisorAgent;
pub struct PlannerAgent;
pub struct ResearchAgent;

impl SupervisorAgent {
    pub fn new() -> Self { SupervisorAgent }
}
impl PlannerAgent {
    pub fn new() -> Self { PlannerAgent }
}
impl ResearchAgent {
    pub fn new() -> Self { ResearchAgent }
}

#[async_trait]
impl Agent for SupervisorAgent {
    fn name(&self) -> &str { "SupervisorAgent" }
    fn matches(&self, _input: &str) -> bool { true }

    async fn process(&self, context: &mut Context, input: String) -> String {
        context.add_to_memory("user", input.clone());
        let response = query_model(&input).await;
        context.add_to_memory("supervisor", response.clone());
        response
    }
}

#[async_trait]
impl Agent for PlannerAgent {
    fn name(&self) -> &str { "PlannerAgent" }
    fn matches(&self, input: &str) -> bool {
        input.contains("plan") || input.contains("steps")
    }

    async fn process(&self, context: &mut Context, input: String) -> String {
        let response = format!("Step-by-step plan for '{}'", input);
        context.add_to_memory("planner", response.clone());
        response
    }
}

#[async_trait]
impl Agent for ResearchAgent {
    fn name(&self) -> &str { "ResearchAgent" }
    fn matches(&self, input: &str) -> bool {
        input.contains("research") || input.contains("find out")
    }

    async fn process(&self, context: &mut Context, input: String) -> String {
        let response = format!("Research result for '{}'", input);
        context.add_to_memory("research", response.clone());
        response
    }
}