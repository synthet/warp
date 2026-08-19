# PermissionRequest hook: notify Warp that Claude needs permission to run a tool.

. "$PSScriptRoot\WarpCommon.ps1"

if (-not (Test-ShouldUseStructured)) { exit 0 }

$hook = Read-HookInput

$toolName = [string](Get-Field $hook 'tool_name' 'unknown')
if (-not $toolName) { $toolName = 'unknown' }

$toolInput = Get-Field $hook 'tool_input' $null
if ($null -eq $toolInput) { $toolInput = [pscustomobject]@{} }

# Human-readable preview: prefer the command, then the file path, else the
# serialized input clipped to 80 chars.
$preview = ''
$command = Get-Field $toolInput 'command' ''
$filePath = Get-Field $toolInput 'file_path' ''
if ($command) {
    $preview = [string]$command
} elseif ($filePath) {
    $preview = [string]$filePath
} else {
    $serialized = try { $toolInput | ConvertTo-Json -Compress -Depth 12 } catch { '' }
    if ($serialized.Length -gt 80) { $serialized = $serialized.Substring(0, 80) }
    $preview = $serialized
}

$summary = "Wants to run $toolName"
if ($preview) { $summary = "$summary`: $(Limit-Text $preview 120)" }

$body = New-WarpPayload -InputObject $hook -EventName 'permission_request' -Extra @{
    summary    = $summary
    tool_name  = $toolName
    tool_input = $toolInput
}

Send-WarpNotification -Title 'warp://cli-agent' -Body $body
