from tools.base import register_tool
import os

@register_tool("readfile")
def read_file(filepath):
    if not os.path.exists(filepath):
        return f"File '{filepath}' not found."
    with open(filepath, "r") as f:
        return f.read()

@register_tool("writefile")
def write_file(filepath, *text):
    with open(filepath, "w") as f:
        f.write(" ".join(text))
    return f"Written to '{filepath}'."