from fastapi import FastAPI
from pydantic import BaseModel
from core.orchestrator import Orchestrator
from core.policy import PolicyEngine
from agents.supervisor_agent import SupervisorAgent
from agents.research_agent import ResearchAgent
from agents.planner_agent import PlannerAgent
from storage.sqlite_backend import init_db

app = FastAPI()

init_db()
policy_engine = PolicyEngine()
agents = {
    "supervisor": SupervisorAgent(),
    "research": ResearchAgent(),
    "planner": PlannerAgent()
}
orchestrator = Orchestrator(agents, policy_engine)

class Query(BaseModel):
    user_id: str
    message: str

@app.post("/query")
def query_mcp(query: Query):
    response = orchestrator.handle_request(query.user_id, query.message)
    return {"response": response}