from core.orchestrator import Orchestrator
from core.policy import PolicyEngine
from agents.supervisor_agent import SupervisorAgent
from agents.research_agent import ResearchAgent
from agents.planner_agent import PlannerAgent
from storage.sqlite_backend import init_db

def main():
    init_db()
    print("=== Model Context Protocol (MCP) ===")
    print("Type 'exit' to quit.\n")

    user_id = "local_user"
    policy_engine = PolicyEngine()
    agents = {
        "supervisor": SupervisorAgent(),
        "research": ResearchAgent(),
        "planner": PlannerAgent()
    }
    orchestrator = Orchestrator(agents, policy_engine)

    while True:
        user_input = input("You> ")
        if user_input.strip().lower() == "exit":
            print("Exiting MCP...")
            break

        response = orchestrator.handle_request(user_id, user_input)
        print(f"MCP> {response}\n")

if __name__ == "__main__":
    main()