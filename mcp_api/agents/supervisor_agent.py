from core.agent import Agent
from core.model_router import query_model
from tools.base import call_tool
import shlex

class SupervisorAgent(Agent):
    def __init__(self):
        super().__init__("SupervisorAgent")

    def process(self, context, input_text):
        context.add_to_memory({"user": input_text})

        if input_text.startswith("!"):
            try:
                parts = shlex.split(input_text[1:])
                tool_name = parts[0]
                args = parts[1:]
                response = call_tool(tool_name, *args)
            except Exception as e:
                response = f"[Tool Error] {e}"
        else:
            response = query_model(input_text)

        context.add_to_memory({"supervisor": response})
        return response