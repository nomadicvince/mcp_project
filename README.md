# Model Context Protocol (MCP)

> A modular AI protocol designed for agents that remember, reason, and evolve.  
> Built from the ground up to prioritize **context**, **sovereignty**, and **developer control**.

---

## About This Project

This is my personal implementation of the **Model Context Protocol (MCP)** — a local-first, model-agnostic architecture for intelligent agent systems.

MCP isn’t a framework. It’s a protocol.  
A lightweight orchestration layer where every agent operates on shared memory, acts independently, and communicates through structured context.

Whether you're using local models with **Ollama**, or scaling up with **OpenAI** or **Claude**, this system gives you full control over how agents think, remember, and respond.

---

## Key Features

- **Context-first architecture** – agents access and write to a persistent `ContextObject`
- **Model-agnostic interface** – plug in `Ollama`, `OpenAI`, or `Claude` instantly
- **Modular orchestration** – swap agents, inject tools, define custom flows
- **No dependencies on LangChain, CrewAI, etc.** – everything is explicit, transparent, and yours

---

## Getting Started

```bash
git clone https://github.com/YOUR_USERNAME/mcp.git
cd mcp
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

Update `core/config.py` to select your backend:

```python
MODEL_BACKEND = "ollama"  # or "openai", "claude"
```

Then run it:

```bash
python run_server.py
```

---

## Why This Matters

This protocol exists because I wanted something that most frameworks don’t offer:

- **Full memory control**  
- **Transparent agent behavior**  
- **Freedom to run locally, scale globally, and own the system logic**

If you care about building systems that **last** — not just ones that reply — this project is a place to start.

---

## Roadmap

- [ ] Add long-term memory backend (Redis or SQLite)
- [ ] Support tool-calling agents
- [ ] WebSocket / FastAPI interface
- [ ] Agent-to-agent messaging
- [ ] RAG + Embedding tools

---

## License

MIT

---

## Author

**Vincent Moore**  
[https://vincentmoore.ai](https://vincentmoore.ai)  
GitHub: [@nomadicvince](https://github.com/nomadicvince)