#!/usr/bin/env powershell
# Tests for deploy_compliance.ps1: the source record and the required-notice check.
# Run: powershell -NoProfile -ExecutionPolicy Bypass -File script/windows/test_deploy_compliance.ps1
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'deploy_compliance.ps1')

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$failures = 0
function Assert-True([bool]$Condition, [string]$Message) {
    if ($Condition) { Write-Output "ok   $Message" } else { Write-Output "FAIL $Message"; $script:failures++ }
}

function Build-StagingFixture {
    $dir = Join-Path ([System.IO.Path]::GetTempPath()) ("deploy-compliance-" + [guid]::NewGuid())
    New-Item -ItemType Directory -Path (Join-Path $dir 'resources') -Force | Out-Null
    foreach ($f in $RequiredStagedNotices) {
        $p = Join-Path $dir $f
        New-Item -ItemType Directory -Path (Split-Path -Parent $p) -Force | Out-Null
        Set-Content -LiteralPath $p -Value 'x'
    }
    return $dir
}

# A complete staging folder passes.
$dir = Build-StagingFixture
$threw = $false
try { Assert-StagedCompliance -StagingDir $dir } catch { $threw = $true }
Assert-True (-not $threw) 'complete staging passes'

# Each missing notice fails, and the error names it.
foreach ($f in $RequiredStagedNotices) {
    $d = Build-StagingFixture
    Remove-Item -LiteralPath (Join-Path $d $f)
    $msg = ''
    try { Assert-StagedCompliance -StagingDir $d } catch { $msg = $_.Exception.Message }
    Assert-True ($msg -like "*$f*") "missing $f fails"
    Remove-Item -LiteralPath $d -Recurse -Force
}

# The source record names the checked-out commit.
Write-SourceRecord -StagingDir $dir -RepoRoot $RepoRoot
$record = Get-Content -LiteralPath (Join-Path $dir 'SOURCE.txt') -Raw
$head = (& git -C $RepoRoot rev-parse HEAD).Trim()
Assert-True ($record -match "commit: $head") 'SOURCE.txt records HEAD'
Assert-True ($record -match "https://github.com/synthet/warp/tree/$head") 'SOURCE.txt links the source tree'
Assert-True ($record -match 'dirty: (true|false)') 'SOURCE.txt records the dirty state'
Remove-Item -LiteralPath $dir -Recurse -Force

if ($failures -gt 0) { Write-Output "$failures failure(s)"; exit 1 }
Write-Output 'all passed'
exit 0
