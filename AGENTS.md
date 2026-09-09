# Agent Registry

OB ships with thirty scoped agents declared in `agents/registry.json`. The JSON declaration is the runtime source of truth and is seeded into SQLite at startup.

Each agent defines:

- stable identifier and name
- narrow purpose and system instruction
- allowed tools and risk classes
- model preferences
- JSON input and output schemas
- timeout and finite retry policy
- verification strategy

## Execution rules

1. OB Prime receives the user request.
2. Task Planner emits an acyclic dependency graph.
3. Security Guardian evaluates every proposed tool action.
4. Agents run only with declared tools.
5. Independent steps may run concurrently up to the configured limit of ten.
6. Failure is isolated; independent steps may continue.
7. Verification Agent checks declared postconditions.
8. OB Prime reports completed, partial, failed, or cancelled state without concealing errors.

## Included agents

1. OB Prime
2. Task Planner
3. Researcher
4. Web Navigator
5. Browser Automation
6. File Intelligence
7. Document Analyst
8. Data Analyst
9. Coding Engineer
10. Code Reviewer
11. Debugger
12. GitHub Engineer
13. Windows Control
14. Screen Vision
15. OCR Specialist
16. Voice Assistant
17. Memory Manager
18. Knowledge Manager
19. Automation Engineer
20. System Monitor
21. Security Guardian
22. Notification Manager
23. Communication Assistant
24. Calendar/Productivity
25. Media Controller
26. Download Manager
27. Presentation/Document Creator
28. API Explorer
29. Device/Hardware Monitor
30. Verification Agent

Agent Town renders persisted runtime states. No activity animation is shown when the registry is idle.

## Custom agents

A custom agent must use the same schema, reference registered tools, stay within autonomy policy, and pass registry validation before activation. New critical capabilities require a dedicated verification contract and cannot be granted by model-generated prose.
