
class ContextObject:
    def __init__(self, user_id, memory=None):
        self.user_id = user_id
        self.memory = memory or {}

    def add_to_memory(self, entry):
        self.memory.setdefault("conversation", []).append(entry)
