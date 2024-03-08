$currentDir = Split-Path $MyInvocation.MyCommand.Path

$frontendDir = Join-Path -Path $currentDir -ChildPath "frontend"

$backendDir = Join-Path -Path $currentDir -ChildPath "backend"

Start-Process powershell -WorkingDirectory $frontendDir -ArgumentList "-Command", "vite build --watch"

Start-Process powershell -WorkingDirectory $backendDir -ArgumentList "-Command", "cargo watch -x run"