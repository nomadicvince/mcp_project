from core.agent import Agent
from core.model_router import query_model

class ResearchAgent(Agent):
    def __init__(self):
        super().__init__("ResearchAgent")

    def process(self, context, input_text):
        context.add_to_memory({"research_query": input_text})
        response = query_model(f"Research this topic: {input_text}")
        context.add_to_memory({"research_response": response})
        return response