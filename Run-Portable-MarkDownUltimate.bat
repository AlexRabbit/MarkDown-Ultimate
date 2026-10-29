@echo off
setlocal
set "PORTABLE=%~dp0..\MarkDown-Ultimate\MarkDownUltimate.exe"
if exist "%PORTABLE%" (
  start "" "%PORTABLE%"
  exit /b 0
)
echo Portable app not built yet.
echo From the dev repo run: tools\portable-bundle.ps1
echo Then run: %PORTABLE%
pause
exit /b 1
