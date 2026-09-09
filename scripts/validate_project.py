from pathlib import Path
import datetime as dt
import json
import re
import shutil
import sys

ROOT = Path(__file__).resolve().parents[1]
EXCLUDED = {"node_modules", "dist", "target", "artifacts", ".git"}
SOURCE_EXTENSIONS = {".rs", ".ts", ".tsx", ".js", ".mjs", ".json", ".md", ".css", ".html", ".sql", ".toml", ".ps1", ".yml", ".yaml"}
INCOMPLETE_MARKERS = ["TO" + "DO", "FIX" + "ME", "un" + "implemented!", "to" + "do!", "rest of " + "file", "same as " + "above", "place" + "holder"]
RETIRED_NAMES = ["NO" + "VA", "NE" + "XUS"]
REQUIRED_TABLES = {
    "users", "settings", "providers", "models", "conversations", "messages", "memories", "memory_embeddings",
    "documents", "document_chunks", "agents", "agent_runs", "tools", "tool_runs", "tasks", "task_steps", "automations",
    "automation_runs", "permissions", "audit_logs", "api_registry", "integrations", "notifications", "system_snapshots",
    "research_sources", "knowledge_entities", "knowledge_relations"
}
REQUIRED_FILES = {
    "package.json", "README.md", "ARCHITECTURE.md", "SECURITY.md", "AGENTS.md", "TOOLS.md", "INTEGRATIONS.md",
    "DEVELOPMENT.md", "PRIVACY.md", "TROUBLESHOOTING.md", "BUILD_STATE.json", "agents/registry.json", "tools/registry.json",
    "src/app/App.tsx", "src-tauri/Cargo.toml", "src-tauri/tauri.conf.json", "src-tauri/src/lib.rs", "src-tauri/migrations/0001_init.sql"
}

checks = []
def check(name, condition, detail=""):
    checks.append({"name": name, "status": "pass" if condition else "fail", "detail": detail})

def project_files():
    for path in ROOT.rglob("*"):
        if not path.is_file() or any(part in EXCLUDED for part in path.relative_to(ROOT).parts):
            continue
        yield path

missing = sorted(path for path in REQUIRED_FILES if not (ROOT / path).exists())
check("required_file_set", not missing, ", ".join(missing))

json_errors = []
for path in project_files():
    if path.suffix == ".json":
        try:
            json.loads(path.read_text(encoding="utf-8"))
        except Exception as error:
            json_errors.append(f"{path.relative_to(ROOT)}: {error}")
check("json_parsing", not json_errors, "; ".join(json_errors))

agents = json.loads((ROOT / "agents/registry.json").read_text(encoding="utf-8"))
tools = json.loads((ROOT / "tools/registry.json").read_text(encoding="utf-8"))
agent_ids = [item["id"] for item in agents]
tool_names = [item["name"] for item in tools]
check("thirty_agents", len(agents) == 30 and len(set(agent_ids)) == 30, f"count={len(agents)}")
check("tool_names_unique", len(tool_names) >= 40 and len(set(tool_names)) == len(tool_names), f"count={len(tool_names)}")
missing_tools = sorted({name for agent in agents for name in agent["tools"] if name not in set(tool_names)})
check("agent_tool_references", not missing_tools, ", ".join(missing_tools))
valid_risks = {"READ_ONLY", "SAFE", "SENSITIVE", "DESTRUCTIVE", "CRITICAL"}
check("risk_classes", all(tool["permission"] in valid_risks for tool in tools), "invalid risk class present")
check("agent_contracts", all(agent.get("inputSchema") and agent.get("outputSchema") and agent.get("verification") and agent.get("retry") for agent in agents))
check("tool_contracts", all(tool.get("inputSchema") and tool.get("outputSchema") and tool.get("verificationMethod") and tool.get("implementation") for tool in tools))

migration = (ROOT / "src-tauri/migrations/0001_init.sql").read_text(encoding="utf-8")
tables = set(re.findall(r"CREATE TABLE IF NOT EXISTS\s+([a-zA-Z0-9_]+)", migration, flags=re.I))
check("database_table_contract", REQUIRED_TABLES.issubset(tables), ", ".join(sorted(REQUIRED_TABLES - tables)))
check("database_indexes", migration.upper().count("CREATE INDEX") >= 8, f"count={migration.upper().count('CREATE INDEX')}")

text_issues = []
secret_issues = []
for path in project_files():
    if path.suffix.lower() not in SOURCE_EXTENSIONS:
        continue
    text = path.read_text(encoding="utf-8", errors="replace")
    lower = text.lower()
    for name in RETIRED_NAMES:
        if name.lower() in lower:
            text_issues.append(f"{path.relative_to(ROOT)}: retired identity")
    for marker in INCOMPLETE_MARKERS:
        if marker.lower() in lower:
            text_issues.append(f"{path.relative_to(ROOT)}: incomplete marker {marker}")
    if re.search(r"(?:sk-[A-Za-z0-9]{20,}|AIza[A-Za-z0-9_-]{25,}|ghp_[A-Za-z0-9]{20,})", text):
        secret_issues.append(str(path.relative_to(ROOT)))
check("identity_and_completeness_scan", not text_issues, "; ".join(text_issues[:20]))
check("secret_literal_scan", not secret_issues, ", ".join(secret_issues))

lib_text = (ROOT / "src-tauri/src/lib.rs").read_text(encoding="utf-8")
modules = re.findall(r"^mod\s+([a-zA-Z0-9_]+);", lib_text, flags=re.M)
missing_modules = [module for module in modules if not (ROOT / f"src-tauri/src/{module}.rs").exists() and not (ROOT / f"src-tauri/src/{module}/mod.rs").exists()]
check("rust_module_graph", not missing_modules, ", ".join(missing_modules))

import_errors = []
for path in (ROOT / "src").rglob("*.ts*"):
    text = path.read_text(encoding="utf-8")
    for target in re.findall(r"from\s+['\"](\.[^'\"]+)['\"]", text):
        base = (path.parent / target)
        candidates = [base, base.with_suffix(".ts"), base.with_suffix(".tsx"), base / "index.ts", base / "index.tsx", base.with_suffix(".json")]
        if not any(candidate.exists() for candidate in candidates):
            import_errors.append(f"{path.relative_to(ROOT)} -> {target}")
check("frontend_local_imports", not import_errors, "; ".join(import_errors))

package = json.loads((ROOT / "package.json").read_text(encoding="utf-8"))
check("build_commands", all(name in package.get("scripts", {}) for name in ["build", "test", "tauri:build", "validate"]))
check("product_identity", package.get("name") == "ob-oraborus" and json.loads((ROOT / "src-tauri/tauri.conf.json").read_text())["productName"] == "OB Oraborus")
check("icon_set", all((ROOT / path).exists() for path in ["src-tauri/icons/32x32.png", "src-tauri/icons/128x128.png", "src-tauri/icons/128x128@2x.png", "src-tauri/icons/icon.ico"]))

failed = [item for item in checks if item["status"] == "fail"]
all_files = sorted(str(path.relative_to(ROOT)).replace("\\", "/") for path in project_files())
artifacts = ["artifacts/OB-Oraborus-Setup.exe", "artifacts/OB-Oraborus-Portable.exe", "artifacts/BUILD_REPORT.json"]
pending = [path for path in artifacts if not (ROOT / path).exists()]
environment_notes = []
if shutil.which("rustc") is None or shutil.which("cargo") is None:
    environment_notes.append({"stage": "native_compile", "message": "Rust toolchain is not installed in this execution environment; run the Windows build script on a Windows MSVC host."})
if sys.platform != "win32":
    environment_notes.append({"stage": "windows_bundle", "message": "NSIS and the Windows executable require a Windows build host."})
state_path = ROOT / "BUILD_STATE.json"
state = json.loads(state_path.read_text(encoding="utf-8"))
state.update({
    "currentStatus": "validation_failed" if failed else ("complete" if not pending else "source_validated_native_windows_build_pending"),
    "generatedFiles": all_files,
    "pendingFiles": pending,
    "buildErrors": [{"stage": item["name"], "message": item["detail"] or "check failed"} for item in failed] + environment_notes,
    "fixedErrors": state.get("fixedErrors", []),
    "testsCompleted": checks,
    "nextOperation": "Fix failed validation checks." if failed else ("No pending operation." if not pending else "Run scripts/build-windows.ps1 on Windows x64 with Rust MSVC and WebView2."),
    "updatedAt": dt.datetime.now(dt.timezone.utc).isoformat()
})
state_path.write_text(json.dumps(state, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
for item in checks:
    print(f"[{item['status'].upper():4}] {item['name']} {item['detail']}")
print(f"\n{len(checks) - len(failed)}/{len(checks)} source checks passed")
sys.exit(1 if failed else 0)
