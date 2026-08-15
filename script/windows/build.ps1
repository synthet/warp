#!/usr/bin/env powershell
<#
.SYNOPSIS
    Build Synth Warp on Windows, resuming incremental Cargo artifacts after OOM or interrupt.

.DESCRIPTION
    Compiles warp-oss (GUI) by default. Re-running this script continues from target/
    instead of starting over. Transient failures (page-file exhaustion, rustc crash)
    are retried with fewer jobs.

.EXAMPLE
    .\script\windows\build.ps1
    .\script\windows\build.ps1 -Release
    .\script\windows\build.ps1 -Clean -Jobs 2
    .\script\windows\build.ps1 -Bin warp-oss -Features gui -Retries 8
#>
param(
    [switch]$Help,
    [switch]$Clean,
    [switch]$Release,
    [int]$Jobs = 0,
    [int]$Retries = 8,
    [int]$RetryWaitSeconds = 15,
    [string]$Bin = 'warp-oss',
    [string]$Features = 'gui',
    [string]$Package = 'warp'
)

$ErrorActionPreference = 'Stop'
if (Get-Variable -Name PSNativeCommandUseErrorActionPreference -ErrorAction SilentlyContinue) {
    $PSNativeCommandUseErrorActionPreference = $false
}

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$TargetDir = Join-Path $RepoRoot 'target'
$StatePath = Join-Path $TargetDir 'synth-build-resume.json'
$LogPath = Join-Path $TargetDir 'synth-build.log'

function Show-Usage {
    Write-Output @'
Usage: .\script\windows\build.ps1 [options]

Build Synth Warp and resume from target\ after an interrupt or out-of-memory crash.

Options:
  -Help                 Show this help.
  -Clean                cargo clean before building (do not resume).
  -Release              Build with --release.
  -Jobs N               Cargo parallelism (default: based on free RAM, min 1 max 4).
  -Retries N            Extra attempts after a retryable failure (default: 8).
  -RetryWaitSeconds N   Pause between retries (default: 15).
  -Bin NAME             Cargo binary (default: warp-oss).
  -Features LIST        Cargo features (default: gui).
  -Package NAME         Cargo package (default: warp).

Re-run the same command to resume. Incremental artifacts in target\ are reused
unless -Clean is passed. Log: target\synth-build.log
'@
}

function Add-DirectoryToPathIfPresent {
    param([string]$Path)
    if (-not $Path -or -not (Test-Path -Path $Path -PathType Container)) {
        return
    }
    $pathEntries = $env:PATH -split ';'
    if ($pathEntries -notcontains $Path) {
        $env:PATH = "$Path;$env:PATH"
    }
}

function Add-WinGetPackageCommandToPath {
    param(
        [string]$CommandName,
        [string]$PackageId
    )
    if (Get-Command -Name $CommandName -Type Application -ErrorAction SilentlyContinue) {
        return
    }
    $winGetPackagesDir = Join-Path $env:LOCALAPPDATA 'Microsoft\WinGet\Packages'
    if (-not (Test-Path -Path $winGetPackagesDir -PathType Container)) {
        return
    }
    $escapedPackageId = [WildcardPattern]::Escape($PackageId)
    $packageDirs = Get-ChildItem -Path $winGetPackagesDir -Directory -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -like "$escapedPackageId*" }
    foreach ($packageDir in $packageDirs) {
        $command = Get-ChildItem -Path $packageDir.FullName -Filter "$CommandName.exe" -Recurse -ErrorAction SilentlyContinue |
            Select-Object -First 1
        if ($command) {
            Add-DirectoryToPathIfPresent $command.DirectoryName
            return
        }
    }
}

function Use-LibclangIfInstalled {
    foreach ($dir in @("$env:ProgramFiles\LLVM\bin", "${env:ProgramFiles(x86)}\LLVM\bin")) {
        if (Test-Path -Path (Join-Path $dir 'libclang.dll') -PathType Leaf) {
            Add-DirectoryToPathIfPresent $dir
            $env:LIBCLANG_PATH = $dir
            return
        }
    }
    $winGetPackagesDir = Join-Path $env:LOCALAPPDATA 'Microsoft\WinGet\Packages'
    if (-not (Test-Path -Path $winGetPackagesDir -PathType Container)) {
        return
    }
    $packageDirs = Get-ChildItem -Path $winGetPackagesDir -Directory -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -like 'LLVM.LLVM*' }
    foreach ($packageDir in $packageDirs) {
        $libclang = Get-ChildItem -Path $packageDir.FullName -Filter 'libclang.dll' -Recurse -ErrorAction SilentlyContinue |
            Select-Object -First 1
        if ($libclang) {
            Add-DirectoryToPathIfPresent $libclang.DirectoryName
            $env:LIBCLANG_PATH = $libclang.DirectoryName
            return
        }
    }
}

function Import-VsDevCmd {
    $vsdev = 'C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat'
    if (-not (Test-Path -Path $vsdev -PathType Leaf)) {
        $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
        if (Test-Path -Path $vswhere -PathType Leaf) {
            $installPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
            if ($installPath) {
                $candidate = Join-Path $installPath 'Common7\Tools\VsDevCmd.bat'
                if (Test-Path -Path $candidate -PathType Leaf) {
                    $vsdev = $candidate
                }
            }
        }
    }
    if (-not (Test-Path -Path $vsdev -PathType Leaf)) {
        throw 'MSVC build tools not found. Run .\script\windows\bootstrap.ps1 first.'
    }
    cmd /c "`"$vsdev`" -arch=amd64 -host_arch=amd64 && set" | ForEach-Object {
        if ($_ -match '^(.*?)=(.*)$') {
            [System.Environment]::SetEnvironmentVariable($matches[1], $matches[2], 'Process')
        }
    }
}

function Initialize-BuildEnvironment {
    Add-DirectoryToPathIfPresent (Join-Path $env:USERPROFILE '.cargo\bin')
    Add-DirectoryToPathIfPresent 'C:\Program Files\CMake\bin'
    Add-DirectoryToPathIfPresent 'C:\Program Files\Git\bin'
    Add-DirectoryToPathIfPresent 'C:\Program Files\Git\usr\bin'
    Add-WinGetPackageCommandToPath -CommandName 'protoc' -PackageId 'Google.Protobuf'
    Use-LibclangIfInstalled
    Import-VsDevCmd
    Add-DirectoryToPathIfPresent (Join-Path $env:USERPROFILE '.cargo\bin')
    Add-DirectoryToPathIfPresent 'C:\Program Files\CMake\bin'
    Add-DirectoryToPathIfPresent 'C:\Program Files\LLVM\bin'
    Add-WinGetPackageCommandToPath -CommandName 'protoc' -PackageId 'Google.Protobuf'
    $msvcHostBin = Get-ChildItem 'C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC' -Directory -ErrorAction SilentlyContinue |
        Sort-Object Name -Descending |
        Select-Object -First 1 |
        ForEach-Object { Join-Path $_.FullName 'bin\Hostx64\x64' }
    if (-not $msvcHostBin) {
        $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
        if (Test-Path -Path $vswhere -PathType Leaf) {
            $installPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
            if ($installPath) {
                $msvcHostBin = Get-ChildItem (Join-Path $installPath 'VC\Tools\MSVC') -Directory -ErrorAction SilentlyContinue |
                    Sort-Object Name -Descending |
                    Select-Object -First 1 |
                    ForEach-Object { Join-Path $_.FullName 'bin\Hostx64\x64' }
            }
        }
    }
    Add-DirectoryToPathIfPresent $msvcHostBin
    Add-DirectoryToPathIfPresent 'C:\Program Files\Git\bin'
    Add-DirectoryToPathIfPresent 'C:\Program Files\Git\usr\bin'
    if (-not (Get-Command -Name cargo -Type Application -ErrorAction SilentlyContinue)) {
        throw 'cargo not found. Run .\script\windows\bootstrap.ps1 first.'
    }
}

function Get-FreeRamGb {
    try {
        $os = Get-CimInstance Win32_OperatingSystem
        return [math]::Round($os.FreePhysicalMemory / 1MB, 1)
    } catch {
        return 8
    }
}

function Get-DefaultJobs {
    $freeGb = Get-FreeRamGb
    $jobs = [int][math]::Floor($freeGb / 6)
    if ($jobs -lt 1) { $jobs = 1 }
    if ($jobs -gt 2) { $jobs = 2 }
    return $jobs
}

function Test-RetryableFailure {
    param(
        [string]$Text,
        [int]$ExitCode
    )
    if ($ExitCode -lt 0) { return $true }
    # NTSTATUS: STACK_BUFFER_OVERRUN, NO_MEMORY, STACK_OVERFLOW
    if ($ExitCode -in @(3221226505, 3221225495, 3221225725)) { return $true }
    return $Text -match 'os error 1455|paging file|Файл подкачки|memory allocation of|STATUS_STACK_BUFFER_OVERRUN|0xc0000409|internal compiler error|fatal runtime error|failed to mmap|signal: 9|\bKilled\b|STATUS_NO_MEMORY|file lock on package cache|Blocking waiting for file lock'
}

function Save-BuildState {
    param(
        [int]$Attempt,
        [int]$JobCount,
        [int]$ExitCode,
        [string]$Reason
    )
    if (-not (Test-Path -Path $TargetDir -PathType Container)) {
        New-Item -ItemType Directory -Path $TargetDir | Out-Null
    }
    $state = [ordered]@{
        bin        = $Bin
        features   = $Features
        release    = [bool]$Release
        jobs       = $JobCount
        attempt    = $Attempt
        last_exit  = $ExitCode
        last_error = $Reason
        log        = $LogPath
        updated_at = (Get-Date).ToString('o')
        resume     = ".\script\windows\build.ps1$(if ($Release) { ' -Release' }) -Jobs $JobCount -Bin $Bin -Features `"$Features`""
    }
    $state | ConvertTo-Json | Set-Content -Path $StatePath -Encoding utf8
}

function Read-BuildState {
    if (-not (Test-Path -Path $StatePath -PathType Leaf)) {
        return $null
    }
    try {
        return Get-Content -Raw -Path $StatePath | ConvertFrom-Json
    } catch {
        return $null
    }
}

function Invoke-CargoBuild {
    param([int]$JobCount)
    $cargoArgs = @(
        'build',
        '-p', $Package,
        '--bin', $Bin,
        '--features', $Features,
        '-j', "$JobCount"
    )
    if ($Release) {
        $cargoArgs += '--release'
    }
    Write-Host "cargo $($cargoArgs -join ' ')"
    Add-Content -Path $LogPath -Value "cargo $($cargoArgs -join ' ')"
    $oldEap = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        # Invoke cargo directly. Start-Process -NoNewWindow -Wait can hang after
        # cargo exits when attached to this console; piping 2>&1 turns cargo
        # stderr into NativeCommandError in PowerShell 7+.
        $cargoExe = (Get-Command cargo -Type Application).Source
        & $cargoExe @cargoArgs
        $code = $LASTEXITCODE
    } finally {
        $ErrorActionPreference = $oldEap
    }
    if ($null -eq $code) { $code = 1 }
    Add-Content -Path $LogPath -Value "cargo exit $code"
    return $code
}

if ($Help) {
    Show-Usage
    exit 0
}

Set-Location $RepoRoot
if (-not (Test-Path -Path $TargetDir -PathType Container)) {
    New-Item -ItemType Directory -Path $TargetDir | Out-Null
}

Initialize-BuildEnvironment

$autoJobs = $Jobs -le 0
if ($autoJobs) {
    $Jobs = Get-DefaultJobs
}
$previous = Read-BuildState
if ($Clean) {
    Write-Output 'Cleaning target\ (-Clean); this will not resume previous artifacts.'
    & cargo clean
    if (Test-Path -Path $StatePath) { Remove-Item -Path $StatePath -Force }
    if (Test-Path -Path $LogPath) { Remove-Item -Path $LogPath -Force }
} elseif ($previous) {
    Write-Output "Resuming previous build: bin=$($previous.bin) jobs=$($previous.jobs) attempt=$($previous.attempt) last_error=$($previous.last_error)"
    Write-Output 'Incremental artifacts in target\ will be reused.'
    if ($autoJobs -and $previous.jobs) {
        $Jobs = [int]$previous.jobs
    }
} else {
    Write-Output 'Starting build. Re-run this script to resume if it is interrupted.'
}
$env:CARGO_BUILD_JOBS = "$Jobs"
$env:CARGO_INCREMENTAL = '1'

Write-Output "cargo=$(cargo --version)"
Write-Output "rustc=$(rustc --version)"
Write-Output "jobs=$Jobs freeRAM=$(Get-FreeRamGb)GB log=$LogPath"
Add-Content -Path $LogPath -Value "`n==== $(Get-Date -Format o) bin=$Bin features=$Features jobs=$Jobs release=$Release ===="

$maxAttempts = [Math]::Max(1, $Retries + 1)
$exitCode = 1
for ($attempt = 1; $attempt -le $maxAttempts; $attempt++) {
    Write-Output "Attempt $attempt / $maxAttempts (jobs=$Jobs)"
    Save-BuildState -Attempt $attempt -JobCount $Jobs -ExitCode 0 -Reason 'running'
    $exitCode = Invoke-CargoBuild -JobCount $Jobs
    if ($exitCode -eq 0) {
        Save-BuildState -Attempt $attempt -JobCount $Jobs -ExitCode 0 -Reason 'ok'
        $profileDir = if ($Release) { 'release' } else { 'debug' }
        $exe = Join-Path $TargetDir "$profileDir\$Bin.exe"
        Write-Output "Build succeeded: $exe"
        exit 0
    }

    $tail = Get-Content -Path $LogPath -Tail 80 -ErrorAction SilentlyContinue | Out-String
    $retryable = Test-RetryableFailure -Text $tail -ExitCode $exitCode
    if (-not $retryable -or $attempt -ge $maxAttempts) {
        Save-BuildState -Attempt $attempt -JobCount $Jobs -ExitCode $exitCode -Reason $(if ($retryable) { 'retryable-exhausted' } else { 'compile-error' })
        Write-Output "Build failed with exit $exitCode. Resume with: .\script\windows\build.ps1"
        exit $exitCode
    }

    $nextJobs = [Math]::Max(1, $Jobs - 1)
    Write-Output "Retryable failure (OOM/rustc crash). Waiting ${RetryWaitSeconds}s then retrying with jobs=$nextJobs"
    $Jobs = $nextJobs
    $env:CARGO_BUILD_JOBS = "$Jobs"
    Save-BuildState -Attempt $attempt -JobCount $Jobs -ExitCode $exitCode -Reason 'oom-or-crash'
    Start-Sleep -Seconds $RetryWaitSeconds
}

exit $exitCode
