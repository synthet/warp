# Stop hook: notify Warp that Claude finished its turn.

. "$PSScriptRoot\WarpCommon.ps1"

if (-not (Test-ShouldUseStructured)) { exit 0 }

$hook = Read-HookInput

# Skip when a stop hook is already active, to avoid double-notifying.
if ([string](Get-Field $hook 'stop_hook_active' 'false') -eq 'true') { exit 0 }

$transcriptPath = [string](Get-Field $hook 'transcript_path' '')

# The Stop hook fires before Claude Code has flushed the current turn to the
# transcript, so give it a moment before reading.
Start-Sleep -Milliseconds 300

$query = Limit-Text (Get-LastUserText $transcriptPath)
$response = Limit-Text (Get-LastAssistantText $transcriptPath)

$body = New-WarpPayload -InputObject $hook -EventName 'stop' -Extra @{
    query           = $query
    response        = $response
    transcript_path = $transcriptPath
}

Send-WarpNotification -Title 'warp://cli-agent' -Body $body
