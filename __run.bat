@echo off
chcp 65001 >nul 2>&1
cd /d "%~dp0"

echo [1/2] Sborka...
cargo build
if %errorlevel% neq 0 (
    echo [ERROR] Sborka ne udalas!
    pause
    exit /b 1
)

echo.
echo Zapusk...
target\debug\rust4ui.exe
pause
