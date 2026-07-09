@echo off
chcp 65001 >nul 2>&1
cd /d "%~dp0"

echo [1/2] Zapusk mdBook...
echo.
echo Otkroy v brauzere: http://localhost:3000
echo.
mdbook serve docs --open
if %errorlevel% neq 0 (
    echo [ERROR] mdBook ne zapustilsya!
    echo Ustanovi: cargo install mdbook
    pause
    exit /b 1
)

pause
