from tools.base import register_tool

@register_tool("add")
def add(x, y):
    return float(x) + float(y)

@register_tool("multiply")
def multiply(x, y):
    return float(x) * float(y)