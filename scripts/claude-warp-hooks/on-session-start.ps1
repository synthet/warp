# SessionStart hook (startup|resume): tell Warp a Claude session came up, and
# report the integration version so Warp can track it.

. "$PSScriptRoot\WarpCommon.ps1"

if (-not (Test-ShouldUseStructured)) { exit 0 }

$hook = Read-HookInput

# Best-effort Claude Code version detection, cached in $CLAUDE_ENV_FILE so the
# other hooks can skip the lookup. The file is sourced as shell, so keep the
# `export KEY="value"` format.
if ($env:CLAUDE_ENV_FILE -and -not $env:CLAUDE_CODE_VERSION) {
    try {
        $detected = (& claude --version 2>$null | Select-Object -First 1)
        if ($detected) {
            Add-Content -LiteralPath $env:CLAUDE_ENV_FILE -Value "export CLAUDE_CODE_VERSION=`"$detected`""
            $env:CLAUDE_CODE_VERSION = $detected
        }
    } catch {
        # Version detection is optional - never fail the hook over it.
    }
}

# Wire-compatible with the 2.2.0 plugin this replaces.
$body = New-WarpPayload -InputObject $hook -EventName 'session_start' -Extra @{
    plugin_version = '2.2.0'
}

Send-WarpNotification -Title 'warp://cli-agent' -Body $body
