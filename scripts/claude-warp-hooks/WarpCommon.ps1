# Shared helpers for the Warp CLI-agent notification hooks.
#
# PowerShell port of the claude-code-warp plugin's bash scripts. Same wire
# protocol, no bash and no jq dependency.
#
# Dot-source this from each hook script:
#   . "$PSScriptRoot\WarpCommon.ps1"

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# The protocol version this port knows how to produce.
$script:PluginProtocolVersion = 1

# Last Warp release per channel that advertised WARP_CLI_AGENT_PROTOCOL_VERSION
# without gating it behind the HOANotifications feature flag. Those builds claim
# protocol support but cannot actually render structured notifications.
$script:LastBrokenDev     = ''
$script:LastBrokenStable  = 'v0.2026.03.25.08.24.stable_05'
$script:LastBrokenPreview = 'v0.2026.03.25.08.24.preview_05'

# First Claude Code version supporting the `terminalSequence` hook output field.
$script:TerminalSequenceMinVersion = [version]'2.1.141'

# Returns $true when structured notifications are safe to send.
function Test-ShouldUseStructured {
    if ([string]::IsNullOrEmpty($env:WARP_CLI_AGENT_PROTOCOL_VERSION)) { return $false }
    if ([string]::IsNullOrEmpty($env:WARP_CLIENT_VERSION)) { return $false }

    $version = $env:WARP_CLIENT_VERSION
    $threshold = ''
    if     ($version -like '*dev*')     { $threshold = $script:LastBrokenDev }
    elseif ($version -like '*stable*')  { $threshold = $script:LastBrokenStable }
    elseif ($version -like '*preview*') { $threshold = $script:LastBrokenPreview }

    # At or before the last broken release for this channel -> not safe.
    if ($threshold -and [string]::CompareOrdinal($version, $threshold) -le 0) { return $false }

    return $true
}

# Reads the hook's JSON payload from stdin. Returns $null when absent/unparseable.
function Read-HookInput {
    $raw = [Console]::In.ReadToEnd()
    if ([string]::IsNullOrWhiteSpace($raw)) { return $null }
    try { return $raw | ConvertFrom-Json } catch { return $null }
}

# Safe property read - the hook payload shape varies by event.
function Get-Field {
    param($Object, [string]$Name, $Default = '')
    if ($null -eq $Object) { return $Default }
    $prop = $Object.PSObject.Properties[$Name]
    if (-not $prop -or $null -eq $prop.Value) { return $Default }
    return $prop.Value
}

function Limit-Text {
    param([string]$Text, [int]$Max = 200)
    if ($Text -and $Text.Length -gt $Max) { return $Text.Substring(0, $Max - 3) + '...' }
    return $Text
}

# Negotiated protocol version: min(this port, what Warp advertises), default 1.
function Get-ProtocolVersion {
    $negotiated = $script:PluginProtocolVersion
    $declared = 0
    if ([int]::TryParse($env:WARP_CLI_AGENT_PROTOCOL_VERSION, [ref]$declared)) {
        if ($declared -lt $negotiated) { $negotiated = $declared }
    }
    return $negotiated
}

# Builds the warp://cli-agent JSON body: common fields plus event-specific ones.
function New-WarpPayload {
    param(
        $InputObject,
        [Parameter(Mandatory)][string]$EventName,
        [hashtable]$Extra = @{}
    )

    $cwd = [string](Get-Field $InputObject 'cwd' '')
    $project = ''
    if ($cwd) { $project = Split-Path -Leaf $cwd }

    $payload = [ordered]@{
        v          = Get-ProtocolVersion
        agent      = 'claude'
        event      = $EventName
        session_id = [string](Get-Field $InputObject 'session_id' '')
        cwd        = $cwd
        project    = $project
    }
    foreach ($key in $Extra.Keys) { $payload[$key] = $Extra[$key] }

    return ($payload | ConvertTo-Json -Compress -Depth 12)
}

# Emits the OSC 777 notify sequence for Warp.
#
# Claude Code >= 2.1.141 accepts a `terminalSequence` field on hook stdout and
# writes the escape sequence to the terminal itself. The bash original fell back
# to writing /dev/tty on older versions; Windows has no equivalent side channel,
# so we emit the field as a best effort and let Claude Code ignore it.
function Send-WarpNotification {
    param([string]$Title = 'Notification', [string]$Body = '')

    $esc = [char]27
    $bel = [char]7
    $sequence = "$esc]777;notify;$Title;$Body$bel"

    $version = $null
    if ($env:CLAUDE_CODE_VERSION -and $env:CLAUDE_CODE_VERSION -match '\d+\.\d+\.\d+') {
        $version = [version]$Matches[0]
    }

    # Known-old Claude Code would reject the unknown field on Stop hooks.
    if ($version -and $version -lt $script:TerminalSequenceMinVersion) { return }

    [Console]::Out.Write((@{ terminalSequence = $sequence } | ConvertTo-Json -Compress))
}

# Reads a transcript .jsonl and returns the last human prompt text.
# "user" entries include tool results, so keep only those with actual text.
function Get-LastUserText {
    param([string]$Path)
    if (-not $Path -or -not (Test-Path -LiteralPath $Path)) { return '' }

    $lines = @(Get-Content -LiteralPath $Path -ErrorAction SilentlyContinue)
    for ($i = $lines.Count - 1; $i -ge 0; $i--) {
        if ([string]::IsNullOrWhiteSpace($lines[$i])) { continue }
        $entry = try { $lines[$i] | ConvertFrom-Json } catch { $null }
        if ($null -eq $entry -or (Get-Field $entry 'type' '') -ne 'user') { continue }

        $content = Get-Field (Get-Field $entry 'message' $null) 'content' $null
        if ($content -is [string]) { return $content }
        if ($null -ne $content) {
            $texts = @($content | Where-Object { (Get-Field $_ 'type' '') -eq 'text' } |
                       ForEach-Object { [string](Get-Field $_ 'text' '') })
            if ($texts.Count -gt 0) { return ($texts -join ' ') }
        }
    }
    return ''
}

# Reads a transcript .jsonl and returns the last assistant response text.
function Get-LastAssistantText {
    param([string]$Path)
    if (-not $Path -or -not (Test-Path -LiteralPath $Path)) { return '' }

    $lines = @(Get-Content -LiteralPath $Path -ErrorAction SilentlyContinue)
    for ($i = $lines.Count - 1; $i -ge 0; $i--) {
        if ([string]::IsNullOrWhiteSpace($lines[$i])) { continue }
        $entry = try { $lines[$i] | ConvertFrom-Json } catch { $null }
        if ($null -eq $entry -or (Get-Field $entry 'type' '') -ne 'assistant') { continue }

        $content = Get-Field (Get-Field $entry 'message' $null) 'content' $null
        if ($null -eq $content) { continue }
        $texts = @($content | Where-Object { (Get-Field $_ 'type' '') -eq 'text' } |
                   ForEach-Object { [string](Get-Field $_ 'text' '') })
        if ($texts.Count -gt 0) { return ($texts -join ' ') }
    }
    return ''
}
