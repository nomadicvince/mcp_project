from core.context import ContextObject
from storage.sqlite_backend import save_context as db_save, load_context as db_load

class Orchestrator:
    def __init__(self, agents, policy_engine):
        self.agents = agents
        self.policy_engine = policy_engine

    def load_context(self, user_id):
        memory_data = db_load(user_id)
        return ContextObject(user_id, memory=memory_data)

    def save_context(self, context):
        db_save(context.user_id, context.memory)

    def select_agent(self, context, input_text):
        return self.agents["supervisor"]

    def handle_request(self, user_id, input_text):
        context = self.load_context(user_id)
        agent = self.select_agent(context, input_text)
        self.policy_engine.validate(context, agent)
        response = agent.process(context, input_text)
        self.save_context(context)
        return response