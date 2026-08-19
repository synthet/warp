# Notification hook (idle_prompt): notify Warp that Claude has gone idle and
# is waiting on the user.

. "$PSScriptRoot\WarpCommon.ps1"

if (-not (Test-ShouldUseStructured)) { exit 0 }

$hook = Read-HookInput

$notificationType = [string](Get-Field $hook 'notification_type' 'unknown')
if (-not $notificationType) { $notificationType = 'unknown' }

$message = [string](Get-Field $hook 'message' 'Input needed')
if (-not $message) { $message = 'Input needed' }

$body = New-WarpPayload -InputObject $hook -EventName $notificationType -Extra @{
    summary = $message
}

Send-WarpNotification -Title 'warp://cli-agent' -Body $body
