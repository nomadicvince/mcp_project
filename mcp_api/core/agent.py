class Agent:
    def __init__(self, name):
        self.name = name

    def process(self, context, input_text):
        raise NotImplementedError("Each agent must implement the process method.")