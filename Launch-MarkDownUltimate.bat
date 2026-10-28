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
  popd
)
if exist "dist\index.html" (
  echo Opening local copy. For API mode run: cargo run -p markdown_ultimate_engine
  start "" "%~dp0dist\index.html"
) else (
  echo Build failed. Open this window's log and fix Trunk/Rust toolchain.
)
echo.
echo Done. Press any key to close.
pause >nul
