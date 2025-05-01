from core.agent import Agent
from core.model_router import query_model

class SupervisorAgent(Agent):
    def __init__(self):
        super().__init__("SupervisorAgent")

    def process(self, context, input_text):
        context.add_to_memory({"user": input_text})
        response = query_model(input_text)
        context.add_to_memory({"supervisor": response})
        return response