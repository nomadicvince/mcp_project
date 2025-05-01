from tools.base import register_tool

@register_tool("search")
def mock_search(query):
    return f"Search result for '{query}': [simulated content]"