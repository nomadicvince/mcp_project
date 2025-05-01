from tools.base import register_tool

@register_tool("echo")
def echo_input(text):
    return f"Echo: {text}"