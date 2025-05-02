from core.orchestrator import Orchestrator
from core.policy import PolicyEngine
from agents.supervisor_agent import SupervisorAgent
from storage.sqlite_backend import init_db

def main():
    init_db()
    print("=== Model Context Protocol (MCP) ===")
    print("Type 'exit' to quit.\n")

    user_id = "local_user"
    policy_engine = PolicyEngine()
    agents = {
        "supervisor": SupervisorAgent()
    }
    orchestrator = Orchestrator(agents, policy_engine)

    while True:
        user_input = input("You> ")
        if user_input.strip().lower() == "exit":
            print("Exiting MCP...")
            break

        print("[DEBUG] Sending to orchestrator:", user_input)
        response = orchestrator.handle_request(user_id, user_input)
        print("[DEBUG] MCP Response:", response)
        print(f"MCP> {response}\n")

if __name__ == "__main__":
    main()