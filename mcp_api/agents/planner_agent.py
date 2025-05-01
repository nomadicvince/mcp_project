from core.agent import Agent
from core.model_router import query_model

class PlannerAgent(Agent):
    def __init__(self):
        super().__init__("PlannerAgent")

    def process(self, context, input_text):
        context.add_to_memory({"planning_request": input_text})
        response = query_model(f"Make a step-by-step plan for: {input_text}")
        context.add_to_memory({"plan": response})
        return response