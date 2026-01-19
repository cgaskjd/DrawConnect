# DrawConnect Desktop Application - Build and Run Script

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  DrawConnect Desktop App Builder" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$projectDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $projectDir

# Check Node.js
$nodeInstalled = Get-Command node -ErrorAction SilentlyContinue
if (-not $nodeInstalled) {
    Write-Host "[ERROR] Node.js is not installed!" -ForegroundColor Red
    Write-Host "Please install Node.js from https://nodejs.org/" -ForegroundColor Yellow
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host "Node.js version:" -ForegroundColor Cyan
node --version
Write-Host ""

# Check Rust/Cargo
$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path
$cargoInstalled = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargoInstalled) {
    Write-Host "[ERROR] Rust/Cargo is not installed!" -ForegroundColor Red
    Write-Host "Please run the core-engine setup script first" -ForegroundColor Yellow
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host "Rust version:" -ForegroundColor Cyan
& "$env:USERPROFILE\.cargo\bin\rustc.exe" --version
Write-Host ""

# Install npm dependencies
Write-Host "[1/3] Installing npm dependencies..." -ForegroundColor Yellow
npm install

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Failed to install npm dependencies" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}
Write-Host "      Dependencies installed!" -ForegroundColor Green
Write-Host ""

# Build Tauri app in development mode
Write-Host "[2/3] Building Tauri application..." -ForegroundColor Yellow
Write-Host "      This may take a few minutes on first build" -ForegroundColor Gray
Write-Host ""

npm run tauri dev

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Build Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan

Read-Host "Press Enter to exit"
