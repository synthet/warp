# Release compliance for script/windows/deploy.ps1 (dot-sourced).
# Synth Term is AGPL-3.0 (app) + MIT (warpui crates): every install must carry the licenses, the fork
# notice and the third-party attributions, and must say which source commit it was built from.

# Paths relative to the staging folder. THIRD_PARTY_LICENSES.txt comes from
# prepare_bundled_resources.ps1 (cargo about); its fallback path does not create it.
$RequiredStagedNotices = @(
    'LICENSE-AGPL',
    'LICENSE-MIT',
    'synth-warp-notice.md',
    'resources\THIRD_PARTY_LICENSES.txt'
)

function Assert-StagedCompliance {
    param([Parameter(Mandatory)][string]$StagingDir)
    $missing = @($RequiredStagedNotices | Where-Object {
            -not (Test-Path -LiteralPath (Join-Path $StagingDir $_) -PathType Leaf)
        })
    if ($missing.Count -gt 0) {
        throw "Release compliance: missing from $StagingDir`: $($missing -join ', ')"
    }
}

function Write-SourceRecord {
    param(
        [Parameter(Mandatory)][string]$StagingDir,
        [Parameter(Mandatory)][string]$RepoRoot
    )
    $commit = (& git -C $RepoRoot rev-parse HEAD).Trim()
    if ($LASTEXITCODE -ne 0 -or -not $commit) {
        throw 'Release compliance: could not read the source commit'
    }
    $dirty = [bool](& git -C $RepoRoot status --porcelain --untracked-files=no)
    $lines = @(
        'Synth Term corresponding source (AGPL-3.0-only app, MIT warpui crates)',
        "commit: $commit",
        "dirty: $($dirty.ToString().ToLowerInvariant())",
        "source: https://github.com/synthet/warp/tree/$commit"
    )
    Set-Content -LiteralPath (Join-Path $StagingDir 'SOURCE.txt') -Value $lines -Encoding utf8
}
