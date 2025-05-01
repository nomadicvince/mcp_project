from datetime import datetime
from tools.base import register_tool

@register_tool("time")
def get_current_time():
    return datetime.now().isoformat()