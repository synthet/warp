# UserPromptSubmit hook: notify Warp when the user submits a prompt, moving the
# session status from idle/blocked back to running.

. "$PSScriptRoot\WarpCommon.ps1"

if (-not (Test-ShouldUseStructured)) { exit 0 }

$hook = Read-HookInput
$query = Limit-Text ([string](Get-Field $hook 'prompt' ''))

$body = New-WarpPayload -InputObject $hook -EventName 'prompt_submit' -Extra @{
    query = $query
}

Send-WarpNotification -Title 'warp://cli-agent' -Body $body
