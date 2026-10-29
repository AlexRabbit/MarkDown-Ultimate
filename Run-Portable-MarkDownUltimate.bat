@echo off
setlocal
cd /d "%~dp0"
if exist "%~dp0MarkDownUltimate.exe" (
  start "" "%~dp0MarkDownUltimate.exe"
  exit /b 0
)
echo MarkDownUltimate.exe is not built yet.
echo Run: powershell -File tools\portable-bundle.ps1
pause
exit /b 1
