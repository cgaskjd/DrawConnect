# DrawConnect Core Engine - Setup and Test Script

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  DrawConnect Core Engine Test Setup" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed
$cargoPath = "$env:USERPROFILE\.cargo\bin\cargo.exe"
$rustInstalled = Test-Path $cargoPath

if (-not $rustInstalled) {
    Write-Host "[1/3] Rust not installed, downloading..." -ForegroundColor Yellow

    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupPath = "$env:TEMP\rustup-init.exe"

    try {
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath -UseBasicParsing
        Write-Host "      Download complete!" -ForegroundColor Green

        Write-Host "[2/3] Installing Rust (this may take a few minutes)..." -ForegroundColor Yellow
        Start-Process -FilePath $rustupPath -ArgumentList "-y" -Wait

        $env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path
        Write-Host "      Rust installed!" -ForegroundColor Green
    }
    catch {
        Write-Host "      Installation failed: $_" -ForegroundColor Red
        Write-Host ""
        Write-Host "Please install Rust manually:" -ForegroundColor Yellow
        Write-Host "1. Visit https://rustup.rs/" -ForegroundColor White
        Write-Host "2. Download and run rustup-init.exe" -ForegroundColor White
        Write-Host "3. Run this script again" -ForegroundColor White
        Read-Host "Press Enter to exit"
        exit 1
    }
}
else {
    Write-Host "[1/3] Rust is installed" -ForegroundColor Green
    $env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path
}

Write-Host ""
Write-Host "Rust version:" -ForegroundColor Cyan
& "$env:USERPROFILE\.cargo\bin\rustc.exe" --version
& "$env:USERPROFILE\.cargo\bin\cargo.exe" --version
Write-Host ""

$projectDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $projectDir

Write-Host "[2/3] Building project..." -ForegroundColor Yellow
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build 2>&1

if ($LASTEXITCODE -ne 0) {
    Write-Host "      Build failed, check errors above" -ForegroundColor Red
}
else {
    Write-Host "      Build complete!" -ForegroundColor Green
}

Write-Host ""
Write-Host "[3/3] Running tests..." -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test 2>&1

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
if ($LASTEXITCODE -eq 0) {
    Write-Host "  All tests passed!" -ForegroundColor Green
}
else {
    Write-Host "  Some tests failed, check output above" -ForegroundColor Yellow
}
Write-Host "========================================" -ForegroundColor Cyan

Write-Host ""
Write-Host "Other commands:" -ForegroundColor Cyan
Write-Host "  cargo test                          - Run tests" -ForegroundColor White
Write-Host "  cargo bench                         - Run benchmarks" -ForegroundColor White
Write-Host "  cargo run --example basic_drawing   - Run example" -ForegroundColor White
Write-Host ""

Read-Host "Press Enter to exit"
