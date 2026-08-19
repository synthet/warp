# PostToolUse hook: notify Warp after a tool call completes, moving the
# session status from Blocked back to Running.

. "$PSScriptRoot\WarpCommon.ps1"

if (-not (Test-ShouldUseStructured)) { exit 0 }

$hook = Read-HookInput
$body = New-WarpPayload -InputObject $hook -EventName 'tool_complete' -Extra @{
    tool_name = [string](Get-Field $hook 'tool_name' '')
}

Send-WarpNotification -Title 'warp://cli-agent' -Body $body
