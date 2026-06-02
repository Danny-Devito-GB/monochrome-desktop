@echo off
title Monochrome — Dev
chcp 65001 >nul
 
powershell -Command "Write-Host ''; Write-Host '  ========================================' -ForegroundColor Cyan; Write-Host '     Monochrome  |  Development Setup' -ForegroundColor White; Write-Host '  ========================================' -ForegroundColor Cyan; Write-Host ''"

REM ── Node.js ──────────────────────────────────────────────────────────────────
where node >nul 2>nul
if %errorlevel% neq 0 (
    powershell -Command "Write-Host '  [FAIL] Node.js not found.' -ForegroundColor Red; Write-Host '         Install from https://nodejs.org' -ForegroundColor Gray"
    pause & exit /b 1
)
for /f "tokens=*" %%v in ('node --version') do set NODE_VER=%%v
powershell -Command "Write-Host '  [ OK ] Node.js %NODE_VER%' -ForegroundColor Green"

REM ── Rust / Cargo ─────────────────────────────────────────────────────────────
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    powershell -Command "Write-Host '  [FAIL] Rust not found.' -ForegroundColor Red; Write-Host '         Install from https://rustup.rs' -ForegroundColor Gray"
    pause & exit /b 1
)
for /f "tokens=1,2" %%a in ('cargo --version') do set CARGO_VER=%%a %%b
powershell -Command "Write-Host '  [ OK ] %CARGO_VER%' -ForegroundColor Green"

REM ── WebView2 (required by Tauri) ─────────────────────────────────────────────
set WEBVIEW2=0
reg query "HKLM\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" >nul 2>nul && set WEBVIEW2=1
reg query "HKCU\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" >nul 2>nul && set WEBVIEW2=1
if %WEBVIEW2% equ 1 (
    powershell -Command "Write-Host '  [ OK ] WebView2 runtime' -ForegroundColor Green"
) else (
    powershell -Command "Write-Host '  [WARN] WebView2 not detected. Tauri requires it.' -ForegroundColor Yellow; Write-Host '         https://developer.microsoft.com/microsoft-edge/webview2' -ForegroundColor Gray"
)
 
echo.

REM ── Dependencies ─────────────────────────────────────────────────────────────
if exist "node_modules\" (
    powershell -Command "Write-Host '  Skipping npm install (node_modules\ exists).' -ForegroundColor Gray; Write-Host '  Delete node_modules\ to force a reinstall.' -ForegroundColor DarkGray"
) else (
    powershell -Command "Write-Host '  Installing npm dependencies...' -ForegroundColor Cyan"
    call npm install
    if %errorlevel% neq 0 (
        powershell -Command "Write-Host '  [FAIL] npm install failed.' -ForegroundColor Red"
        pause & exit /b 1
    )
    powershell -Command "Write-Host '  [ OK ] Dependencies installed.' -ForegroundColor Green"
)

echo.
powershell -Command "Write-Host '  ========================================' -ForegroundColor Cyan; Write-Host '     Launching development build...' -ForegroundColor White; Write-Host '  ========================================' -ForegroundColor Cyan"
echo.

call npm run dev
 
echo.
powershell -Command "Write-Host '  Application closed.' -ForegroundColor Yellow"
pause