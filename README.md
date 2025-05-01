# 🧠 Model Context Protocol (MCP)

## Overview

The **Model Context Protocol (MCP)** is an architectural concept for coordinating interactions between large language models, memory systems, and toolchains. It allows agents (like planners, researchers, or supervisors) to share context and delegate tasks intelligently.

> This repository contains an implementation of an MCP system — built in both Python and Rust — with the goal of exploring practical applications of multi-agent AI orchestration, persistent memory, and real-world tool integration.

---

## 🎯 Project Goals

- ✅ Build a modular, multi-agent architecture that can evolve over time
- ✅ Implement persistent memory for context-aware conversations
- ✅ Add tool-calling (math, file ops, mock search, time, echo)
- ✅ Support local model APIs (e.g. Ollama) and cloud APIs (e.g. OpenAI)
- ✅ Provide both CLI and API-based interfaces
- ✅ Explore both Python and Rust backends

---

## 🔀 Versions in This Repository

| Version         | Language | Interface      | Memory | Agents         | Tool Support | API / WS        | Use Case                        |
|-----------------|----------|----------------|--------|----------------|---------------|------------------|----------------------------------|
| `mcp_cli`       | Python   | CLI            | ✅ SQLite | Supervisor      | ❌             | ❌                | Lightweight, test-focused        |
| `mcp_api`       | Python   | HTTP + WS      | ✅ SQLite | Supervisor + Planner + Research | ✅         | ✅ REST + WebSocket | Full API MVP                     |
| `mcp_rust_v1.3` | Rust     | CLI + HTTP API | ✅ SQLite | Same as above   | ❌ (planned)   | ✅ via Axum       | High-performance experimental    |

---

## 🧪 Setup Instructions

### 📁 `mcp_cli` — Minimal Local Testing (Python)

```bash
cd mcp_cli
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python run_server.py
```

> ✅ Test via terminal  
> ❌ No API or tools in this version

---

### 📁 `mcp_api` — Full API MVP (Python)

```bash
cd mcp_api
cp .env.example .env
docker build -t mcp-api .
docker run -p 3000:3000 --env-file .env mcp-api
```

Access:

- `http://localhost:3000/docs` → Swagger UI  
- `GET /healthz` → Health check  
- `POST /query` → Main endpoint  
- `ws://localhost:3000/ws` → Real-time agent access

✅ Includes:
- REST + WebSocket interface  
- Multi-agent routing  
- Tool-calling  
- Persistent memory  
- Docker-ready

---

### 📁 `mcp_rust_v1.3` — Rust Version with Axum

```bash
cd mcp_rust_v1_3
cargo build
cargo run
```

API available at `http://localhost:3000`

✅ Features:
- Multi-agent support  
- Persistent memory (SQLite)  
- Axum-based high-performance API  
- CLI + HTTP input

---

## 🧭 Roadmap:
- [ ] Agent-to-Agent Messaging  
- [ ] Document Retrieval (RAG)  
- [ ] Front-End Interface (React or Svelte)  
- [ ] Production Deployment (Fly.io, Render, Linode)  
- [ ] Enhanced Memory Scope & TTL  
- [ ] Shared memory and scoped goals

---

## 💡 Choosing the Right Version

| Goal                                   | Use Version     |
|----------------------------------------|-----------------|
| Quick testing in terminal              | `mcp_cli`       |
| Full API-ready multi-agent architecture| `mcp_api`       |
| High-performance compiled system       | `mcp_rust_v1.3` |

---

## 📜 License

MIT License — Free to use, fork, and adapt.

---

