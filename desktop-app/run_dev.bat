@echo off
echo ========================================
echo   DrawConnect Desktop App
echo ========================================
echo.
echo Starting development server...
echo.

powershell.exe -ExecutionPolicy Bypass -File "%~dp0run_dev.ps1"

pause
