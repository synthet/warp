@echo off
REM Double-clickable wrapper for deploy.ps1. Installs Synth Term per user into
REM %LOCALAPPDATA%\Programs\SynthTerm; no elevation is needed.
setlocal
set "SCRIPT=%~dp0deploy.ps1"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%SCRIPT%" %*
exit /b %ERRORLEVEL%
