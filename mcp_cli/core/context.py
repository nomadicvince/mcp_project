class ContextObject:
    def __init__(self, user_id, memory=None, preferences=None, goals=None):
        self.user_id = user_id
        self.memory = memory or {}
        self.preferences = preferences or {}
        self.goals = goals or {}

    def add_to_memory(self, entry):
        self.memory.setdefault("conversation", []).append(entry)