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
  echo Build failed - dist\index.html not found.
  pause
  exit /b 1
)

if not exist "dist\js\mdu-ui.js" (
  if not exist "dist\js" mkdir "dist\js"
  copy /Y "%~dp0frontend\js\mdu-ui.js" "%~dp0dist\js\mdu-ui.js" >nul 2>&1
)

if "%MDU_PORT%"=="" set "MDU_PORT=8765"

for /f "tokens=5" %%a in ('netstat -ano ^| findstr ":%MDU_PORT% " ^| findstr LISTENING') do (
  taskkill /F /PID %%a >nul 2>&1
)

where pythonw >nul 2>&1
if %errorlevel%==0 (
  set "PYEXE=pythonw"
  set "PYARG="
  goto :start_server
)
where python >nul 2>&1
if %errorlevel%==0 (
  set "PYEXE=python"
  set "PYARG="
  goto :start_server
)
if exist "%~dp0python\pythonw.exe" (
  set "PYEXE=%~dp0python\pythonw.exe"
  set "PYARG="
  goto :start_server
)
if exist "%~dp0python\python.exe" (
  set "PYEXE=%~dp0python\python.exe"
  set "PYARG="
  goto :start_server
)
where py >nul 2>&1
if %errorlevel%==0 (
  set "PYEXE=py"
  set "PYARG=-3"
  goto :start_server
)
echo Python is required for the local web server.
echo Install from https://www.python.org/downloads/ then run this launcher again.
pause
exit /b 1

:start_server
echo Starting local web server - do not open index.html as a file.
set "MDU_QUIET=1"
set "MDU_PORT=%MDU_PORT%"
start "" /B %PYEXE% %PYARG% "%~dp0tools\serve_dist.py"
timeout /t 3 /nobreak >nul
start "" "http://127.0.0.1:%MDU_PORT%/MarkDown-Ultimate/"
echo.
echo MarkDown Ultimate: http://127.0.0.1:%MDU_PORT%/MarkDown-Ultimate/
echo Leave this window open while you use the app.
echo.
pause
