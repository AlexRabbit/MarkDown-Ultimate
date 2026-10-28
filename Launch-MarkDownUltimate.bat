@echo off
setlocal
cd /d "%~dp0"
title MarkDown Ultimate Launcher
echo MarkDown Ultimate - preparing...

if not exist "dist\index.html" (
  echo Building web app with Trunk...
  call "%ProgramFiles(x86)%\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1
  pushd frontend
  trunk build --release
  if errorlevel 1 (
    echo Trunk build failed.
    pause
    exit /b 1
  )
  popd
)

if not exist "dist\index.html" (
  echo Build failed — dist\index.html not found.
  pause
  exit /b 1
)

echo Starting local web server (required — do not open index.html as a file).
powershell -ExecutionPolicy Bypass -File "%~dp0tools\Open-LocalWebApp.ps1"
if errorlevel 1 (
  echo Could not start local server.
  pause
  exit /b 1
)

echo.
echo App opened in your browser at http://127.0.0.1:8765/MarkDown-Ultimate/
echo Leave this window open while you use the app, or close to stop the server.
echo.
pause
