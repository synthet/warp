#Requires -Version 5.1
<#
.SYNOPSIS
    Installs PowerShell Claude Code hooks for Warp terminal notifications on Windows.

.DESCRIPTION
    Copies scripts/claude-warp-hooks into %USERPROFILE%\.claude\hooks\warp\ and merges
    a hooks block into %USERPROFILE%\.claude\settings.json. Does not install the
    bash-based warp@claude-code-warp plugin (broken on Windows).

    Run from the Synth Warp repo root:
      .\scripts\install_claude_warp_hooks_windows.ps1
#>
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$RepoRoot = Split-Path -Parent $PSScriptRoot
$SourceDir = Join-Path $RepoRoot 'scripts\claude-warp-hooks'
$ClaudeDir = Join-Path $env:USERPROFILE '.claude'
$DestDir = Join-Path $ClaudeDir 'hooks\warp'
$SettingsPath = Join-Path $ClaudeDir 'settings.json'
$InstalledPluginsPath = Join-Path $ClaudeDir 'plugins\installed_plugins.json'

$HookScripts = @(
    'WarpCommon.ps1',
    'on-session-start.ps1',
    'on-stop.ps1',
    'on-stop-failure.ps1',
    'on-notification.ps1',
    'on-permission-request.ps1',
    'on-prompt-submit.ps1',
    'on-post-tool-use.ps1'
)

function Get-HookCommand {
    param([string]$ScriptName)
    $scriptPath = ($DestDir -replace '\\', '/') + "/$ScriptName"
    return "pwsh -NoProfile -File `"$scriptPath`""
}

function New-HooksBlock {
    $cmd = @{
        SessionStart      = Get-HookCommand 'on-session-start.ps1'
        Stop              = Get-HookCommand 'on-stop.ps1'
        StopFailure       = Get-HookCommand 'on-stop-failure.ps1'
        Notification      = Get-HookCommand 'on-notification.ps1'
        PermissionRequest = Get-HookCommand 'on-permission-request.ps1'
        UserPromptSubmit  = Get-HookCommand 'on-prompt-submit.ps1'
        PostToolUse       = Get-HookCommand 'on-post-tool-use.ps1'
    }

    return [ordered]@{
        SessionStart = @(
            [ordered]@{
                matcher = 'startup|resume'
                hooks   = @(
                    [ordered]@{
                        type    = 'command'
                        command = $cmd.SessionStart
                    }
                )
            }
        )
        Stop = @(
            [ordered]@{
                hooks = @(
                    [ordered]@{
                        type    = 'command'
                        command = $cmd.Stop
                    }
                )
            }
        )
        StopFailure = @(
            [ordered]@{
                hooks = @(
                    [ordered]@{
                        type    = 'command'
                        command = $cmd.StopFailure
                    }
                )
            }
        )
        Notification = @(
            [ordered]@{
                matcher = 'idle_prompt'
                hooks   = @(
                    [ordered]@{
                        type    = 'command'
                        command = $cmd.Notification
                    }
                )
            }
        )
        PermissionRequest = @(
            [ordered]@{
                hooks = @(
                    [ordered]@{
                        type    = 'command'
                        command = $cmd.PermissionRequest
                    }
                )
            }
        )
        UserPromptSubmit = @(
            [ordered]@{
                hooks = @(
                    [ordered]@{
                        type    = 'command'
                        command = $cmd.UserPromptSubmit
                    }
                )
            }
        )
        PostToolUse = @(
            [ordered]@{
                hooks = @(
                    [ordered]@{
                        type    = 'command'
                        command = $cmd.PostToolUse
                    }
                )
            }
        )
    }
}

if (-not (Test-Path -LiteralPath $SourceDir)) {
    Write-Error "Hook source not found: $SourceDir (run from Synth Warp repo root)"
}

foreach ($name in $HookScripts) {
    $src = Join-Path $SourceDir $name
    if (-not (Test-Path -LiteralPath $src)) {
        Write-Error "Missing hook script in repo: $src"
    }
}

New-Item -ItemType Directory -Force -Path $DestDir | Out-Null
foreach ($name in $HookScripts) {
    Copy-Item -LiteralPath (Join-Path $SourceDir $name) -Destination (Join-Path $DestDir $name) -Force
}
Write-Host "Copied $($HookScripts.Count) hook scripts to $DestDir"

if (Test-Path -LiteralPath $InstalledPluginsPath) {
    $raw = Get-Content -LiteralPath $InstalledPluginsPath -Raw
    if ($raw -match 'warp@claude-code-warp') {
        Write-Warning @"
warp@claude-code-warp is still listed in installed_plugins.json.
The bash plugin hooks ShellExecute .sh files on Windows and will conflict.
Run: claude plugin uninstall warp@claude-code-warp
"@
    }
}

$settings = [PSCustomObject]@{}
if (Test-Path -LiteralPath $SettingsPath) {
    $settings = Get-Content -LiteralPath $SettingsPath -Raw | ConvertFrom-Json
}

$hooksObject = New-HooksBlock | ConvertTo-Json -Depth 12 | ConvertFrom-Json
$settings | Add-Member -NotePropertyName hooks -NotePropertyValue $hooksObject -Force

if ($settings.PSObject.Properties.Name -contains 'enabledPlugins' -and $null -ne $settings.enabledPlugins) {
    $warpKey = 'warp@claude-code-warp'
    $prop = $settings.enabledPlugins.PSObject.Properties[$warpKey]
    if ($null -ne $prop) {
        $prop.Value = $false
    }
}

($settings | ConvertTo-Json -Depth 12) | Set-Content -LiteralPath $SettingsPath -Encoding utf8
Write-Host "Updated $SettingsPath hooks block"

Write-Host @"

Done. Next steps:
  1. Restart Claude Code (exit and start claude again).
  2. Run a tool call; hook logs should show .../hooks/warp/on-post-tool-use.ps1
  3. Do NOT install warp@claude-code-warp on Windows.

See docs/guides/claude-code-warp-windows-hooks.md
"@
