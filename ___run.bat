@echo off
chcp 65001 >nul 2>&1
cd /d "%~dp0"

echo [1/3] Proverka...
if exist target\debug\rust4ui.exe (
    for %%F in (target\debug\rust4ui.exe) do echo Binary ot: %%~tF
) else (
    echo Binary: net (pervaya sborka)
)
echo.

echo [2/3] Sborka...
cargo build -v > "%TEMP%\cargo_build.log" 2>&1
set CARGO_EXIT=%errorlevel%
powershell -noprofile -command "$f=Get-Content '%TEMP%\cargo_build.log'; $f | ForEach-Object { if ($_ -match 'the file `([^`]+)`') { Write-Host ('       Changed: '+$Matches[1]) } elseif ($_ -match 'Compiling |Finished|^error\[') { Write-Host $_ } }"
if %CARGO_EXIT% neq 0 (
    echo [ERROR] Sborka ne udalas!
    echo. 1>&2
    powershell -noprofile -command "$f=Get-Content '%TEMP%\cargo_build.log'; $f | ForEach-Object { if ($_ -match 'the file `([^`]+)`' -or $_ -match 'Compiling |Finished|^error\[' -or $_ -match '^error\[|^\s+-->|^\s+\d+\s+|\s+=\s+note') { Write-Host $_ } }" 1>&2
    pause
    exit /b 1
)

echo.
echo [3/3] Zapusk...
target\debug\rust4ui.exe
pause
