
from core.context import ContextObject
from storage.sqlite_backend import save_context, load_context

class Orchestrator:
    def __init__(self, agents, policy_engine):
        self.agents = agents
        self.policy_engine = policy_engine

    def handle_request(self, user_id, input_text):
        memory = load_context(user_id)
        context = ContextObject(user_id, memory)
        agent = self.agents["supervisor"]
        self.policy_engine.validate(context, agent)
        response = agent.process(context, input_text)
        save_context(user_id, context.memory)
        return response
