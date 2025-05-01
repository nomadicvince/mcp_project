tool_registry = {}

def register_tool(name):
    def decorator(func):
        tool_registry[name] = func
        return func
    return decorator

def call_tool(name, *args, **kwargs):
    if name in tool_registry:
        return tool_registry[name](*args, **kwargs)
    raise ValueError(f"Tool '{name}' is not registered.")