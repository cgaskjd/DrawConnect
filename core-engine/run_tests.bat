@echo off
echo ========================================
echo   DrawConnect Core Engine Test Runner
echo ========================================
echo.
echo Starting test environment...
echo.

powershell.exe -ExecutionPolicy Bypass -File "%~dp0setup_and_test.ps1"

pause
