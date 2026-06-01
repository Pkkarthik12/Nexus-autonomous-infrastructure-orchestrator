# Start all Nexus agents (requires built binaries and docker compose stack)
$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$config = Join-Path $root "config\example.yaml"

$agents = @("agent-scout", "agent-sentinel", "agent-planner", "agent-executor", "agent-chaos")
foreach ($a in $agents) {
    Start-Process -FilePath "cargo" -ArgumentList "run", "-p", $a, "--", "--config", $config -WorkingDirectory $root
    Write-Host "Started $a"
}
