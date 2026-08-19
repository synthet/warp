# StopFailure hook: notify Warp that Claude's turn ended due to an API error.

. "$PSScriptRoot\WarpCommon.ps1"

if (-not (Test-ShouldUseStructured)) { exit 0 }

$hook = Read-HookInput

$errorType = [string](Get-Field $hook 'error' '')
$errorMessage = [string](Get-Field $hook 'last_assistant_message' '')
$transcriptPath = [string](Get-Field $hook 'transcript_path' '')

# Warp shows the last user query as the notification title, matching Stop.
$query = Limit-Text (Get-LastUserText $transcriptPath)

$body = New-WarpPayload -InputObject $hook -EventName 'stop_failure' -Extra @{
    query           = $query
    response        = $errorMessage
    error_type      = $errorType
    transcript_path = $transcriptPath
}

Send-WarpNotification -Title 'warp://cli-agent' -Body $body
