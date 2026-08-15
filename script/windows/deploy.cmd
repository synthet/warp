@echo off
REM Double-clickable wrapper for deploy.ps1. UAC elevation happens inside the script
REM when C:\Program Files\Warp is not writable.
setlocal
set "SCRIPT=%~dp0deploy.ps1"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%SCRIPT%" %*
exit /b %ERRORLEVEL%
