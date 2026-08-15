#!/usr/bin/env powershell
<#
.SYNOPSIS
    Overlay a local Synth Warp (OSS) build onto the installed Warp directory.

.DESCRIPTION
    Builds warp-oss (unless -SkipBuild), stages it as warp.exe plus the Windows
    payload DLLs/resources, then copies into C:\Program Files\Warp. Existing
    Start Menu shortcuts and unins000.* are left in place. Re-run after each
    build to update the live install.

.EXAMPLE
    .\script\windows\deploy.ps1
    .\script\windows\deploy.ps1 -SkipBuild
    .\script\windows\deploy.ps1 -Debug
#>
param(
    [switch]$Help,
    [switch]$SkipBuild,
    [switch]$Debug,
    [switch]$Release,
    [switch]$Restart,
    [switch]$NoRestart,
    [switch]$CopyOnly,
    [string]$InstallDir = '',
    [string]$Features = 'release_bundle,gui',
    [string]$Bin = 'warp-oss',
    [string]$Package = 'warp'
)

$ErrorActionPreference = 'Stop'
if (Get-Variable -Name PSNativeCommandUseErrorActionPreference -ErrorAction SilentlyContinue) {
    $PSNativeCommandUseErrorActionPreference = $false
}

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$TargetDir = Join-Path $RepoRoot 'target'
$StagingDir = Join-Path $TargetDir 'warp-deploy-staging'
$BuildScript = Join-Path $PSScriptRoot 'build.ps1'
$PrepareResourcesScript = Join-Path $PSScriptRoot 'prepare_bundled_resources.ps1'

if (-not $InstallDir) {
    $InstallDir = Join-Path $env:ProgramFiles 'Warp'
}

# -Release is the default; -Debug opts into a faster non-release overlay.
$UseRelease = -not $Debug
# -Restart is the default; -NoRestart skips relaunch.
$ShouldRestart = -not $NoRestart

function Show-Usage {
    Write-Output @'
Usage: .\script\windows\deploy.ps1 [options]

Build Synth Warp (OSS) and copy it over the live install at
C:\Program Files\Warp, keeping Start Menu shortcuts and unins000.*.

Options:
  -Help              Show this help.
  -SkipBuild         Do not compile; stage/copy the last warp-oss.exe.
  -Release           Build with --release (default unless -Debug).
  -Debug             Faster non-release overlay.
  -Restart           Relaunch warp.exe after copy (default).
  -NoRestart         Do not relaunch after copy.
  -InstallDir PATH   Install directory (default: %ProgramFiles%\Warp).
  -Features LIST     Cargo features (default: release_bundle,gui).
  -Bin NAME          Cargo binary (default: warp-oss).
  -Package NAME      Cargo package (default: warp).

Examples:
  .\script\windows\deploy.ps1
  .\script\windows\deploy.ps1 -SkipBuild
  .\script\windows\deploy.ps1 -Debug
'@
}

function Test-IsAdministrator {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = [Security.Principal.WindowsPrincipal]$identity
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Get-WindowsPayloadArch {
    if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64') {
        return 'arm64'
    }
    return 'x64'
}

function Test-DirectoryWritable {
    param([string]$Path)
    if (-not (Test-Path -LiteralPath $Path -PathType Container)) {
        try {
            New-Item -ItemType Directory -Path $Path -Force | Out-Null
        } catch {
            return $false
        }
    }
    $probe = Join-Path $Path '.warp-deploy-write-test'
    try {
        [System.IO.File]::WriteAllText($probe, 'ok')
        Remove-Item -LiteralPath $probe -Force
        return $true
    } catch {
        return $false
    }
}

function Get-BuiltBinaryPath {
    $profileDir = if ($UseRelease) { 'release' } else { 'debug' }
    $preferred = Join-Path $TargetDir "$profileDir\$Bin.exe"
    if (Test-Path -LiteralPath $preferred -PathType Leaf) {
        return $preferred
    }
    $fallbackProfile = if ($UseRelease) { 'debug' } else { 'release' }
    $fallback = Join-Path $TargetDir "$fallbackProfile\$Bin.exe"
    if (Test-Path -LiteralPath $fallback -PathType Leaf) {
        Write-Host "Using $fallback (expected $preferred was missing)."
        return $fallback
    }
    throw "Built binary not found: $preferred"
}

function Copy-PayloadFile {
    param(
        [string]$Source,
        [string]$Destination,
        [switch]$Required
    )
    if (Test-Path -LiteralPath $Source -PathType Leaf) {
        $destDir = Split-Path -Parent $Destination
        if (-not (Test-Path -LiteralPath $destDir -PathType Container)) {
            New-Item -ItemType Directory -Path $destDir | Out-Null
        }
        Copy-Item -LiteralPath $Source -Destination $Destination -Force
        return
    }
    if ($Required) {
        throw "Required payload file is missing: $Source"
    }
    Write-Output "Skipping missing optional payload: $Source"
}

function Write-CliShim {
    param(
        [string]$Path,
        [string]$ExePath
    )
    $dir = Split-Path -Parent $Path
    if (-not (Test-Path -LiteralPath $dir -PathType Container)) {
        New-Item -ItemType Directory -Path $dir | Out-Null
    }
    $content = "@echo off`r`nset `"WARP_CLI_MODE=1`"`r`n`"$ExePath`" %*`r`n"
    Set-Content -LiteralPath $Path -Value $content -Encoding ascii -NoNewline
}

function Invoke-PrepareResources {
    param([string]$DestinationDir, [string]$CargoProfile)
    $bundledSource = Join-Path $RepoRoot 'resources\bundled'
    $bundledDest = Join-Path $DestinationDir 'bundled'
    if (-not (Test-Path -LiteralPath $bundledSource -PathType Container)) {
        throw "Bundled resources not found at $bundledSource"
    }
    if (-not (Test-Path -LiteralPath $DestinationDir -PathType Container)) {
        New-Item -ItemType Directory -Path $DestinationDir | Out-Null
    }

    try {
        & $PrepareResourcesScript -DestinationDir $DestinationDir -Channel oss -CargoProfile $CargoProfile
        if ($LASTEXITCODE -and $LASTEXITCODE -ne 0) {
            throw "prepare_bundled_resources.ps1 exited $LASTEXITCODE"
        }
        return
    } catch {
        Write-Output "prepare_bundled_resources.ps1 failed ($($_.Exception.Message)); copying resources\bundled only."
        if (Test-Path -LiteralPath $bundledDest -PathType Container) {
            Remove-Item -LiteralPath $bundledDest -Recurse -Force
        }
        Copy-Item -LiteralPath $bundledSource -Destination $bundledDest -Recurse -Force
    }
}

function Initialize-DeployStaging {
    $arch = Get-WindowsPayloadArch
    $assetsDir = Join-Path $RepoRoot "app\assets\windows\$arch"
    $exeSource = Get-BuiltBinaryPath
    $installedExe = Join-Path $InstallDir 'warp.exe'

    if (Test-Path -LiteralPath $StagingDir -PathType Container) {
        Remove-Item -LiteralPath $StagingDir -Recurse -Force
    }
    New-Item -ItemType Directory -Path $StagingDir | Out-Null

    Write-Output "Staging $exeSource as warp.exe"
    Copy-PayloadFile -Source $exeSource -Destination (Join-Path $StagingDir 'warp.exe') -Required

    $requiredAssets = @(
        @{ Src = Join-Path $assetsDir 'conpty.dll'; Dst = Join-Path $StagingDir 'conpty.dll' },
        @{ Src = Join-Path $assetsDir 'vcruntime140.dll'; Dst = Join-Path $StagingDir 'vcruntime140.dll' },
        @{ Src = Join-Path $assetsDir 'vcruntime140_1.dll'; Dst = Join-Path $StagingDir 'vcruntime140_1.dll' },
        @{ Src = Join-Path $assetsDir 'msvcp140.dll'; Dst = Join-Path $StagingDir 'msvcp140.dll' },
        @{ Src = Join-Path $assetsDir 'dxcompiler.dll'; Dst = Join-Path $StagingDir 'dxcompiler.dll' },
        @{ Src = Join-Path $assetsDir 'dxil.dll'; Dst = Join-Path $StagingDir 'dxil.dll' },
        @{ Src = Join-Path $assetsDir 'OpenConsole.exe'; Dst = Join-Path $StagingDir "$arch\OpenConsole.exe" }
    )
    foreach ($asset in $requiredAssets) {
        $relative = $asset.Dst.Substring($StagingDir.Length).TrimStart('\')
        $installedCopy = Join-Path $InstallDir $relative
        if (Test-Path -LiteralPath $asset.Src -PathType Leaf) {
            Copy-PayloadFile -Source $asset.Src -Destination $asset.Dst -Required
        } elseif (Test-Path -LiteralPath $installedCopy -PathType Leaf) {
            Write-Output "Repo asset missing ($($asset.Src)); leaving installed copy in place."
        } else {
            throw "Required payload file is missing and not already installed: $($asset.Src)"
        }
    }

    $iconCandidates = @(
        (Join-Path $RepoRoot 'app\channels\oss\icon\no-padding\icon.ico'),
        (Join-Path $RepoRoot 'app\channels\stable\icon\no-padding\icon.ico')
    )
    $iconCopied = $false
    foreach ($icon in $iconCandidates) {
        if (Test-Path -LiteralPath $icon -PathType Leaf) {
            Copy-PayloadFile -Source $icon -Destination (Join-Path $StagingDir 'icon.ico')
            $iconCopied = $true
            break
        }
    }
    if (-not $iconCopied) {
        Write-Output 'No channel icon.ico in the checkout; leaving the installed icon in place.'
    }

    Copy-PayloadFile -Source (Join-Path $RepoRoot 'app\assets\bundled\bootstrap\pwsh.ps1') `
        -Destination (Join-Path $StagingDir 'pwsh.ps1') -Required

    $cargoProfile = if ($UseRelease) { 'release' } else { 'debug' }
    Write-Output 'Preparing bundled resources...'
    Invoke-PrepareResources -DestinationDir (Join-Path $StagingDir 'resources') -CargoProfile $cargoProfile

    Write-CliShim -Path (Join-Path $StagingDir 'bin\warp.cmd') -ExePath $installedExe
    Write-CliShim -Path (Join-Path $StagingDir 'bin\warp-oss.cmd') -ExePath $installedExe
}

function Stop-InstalledWarp {
    $exePath = Join-Path $InstallDir 'warp.exe'
    if (-not (Test-Path -LiteralPath $exePath -PathType Leaf)) {
        return
    }
    $target = [System.IO.Path]::GetFullPath($exePath)
    Write-Output "Stopping $target if it is running..."

    Get-Process -Name 'warp' -ErrorAction SilentlyContinue | ForEach-Object {
        $path = $null
        try { $path = $_.Path } catch { $path = $null }
        if ($path -and ([System.IO.Path]::GetFullPath($path) -eq $target)) {
            Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue
        }
    }
    try {
        Get-CimInstance Win32_Process -Filter "Name='warp.exe'" -ErrorAction SilentlyContinue |
            Where-Object {
                $_.ExecutablePath -and
                ([System.IO.Path]::GetFullPath($_.ExecutablePath) -eq $target) -and
                $_.CommandLine -like '*minidump-server*'
            } |
            ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }
    } catch {
        # CIM query is best-effort; file-lock wait below is the real gate.
    }

    $deadline = (Get-Date).AddSeconds(30)
    while ((Get-Date) -lt $deadline) {
        try {
            $stream = [System.IO.File]::Open($exePath, 'Open', 'ReadWrite', 'None')
            $stream.Close()
            return
        } catch {
            Start-Sleep -Milliseconds 400
        }
    }
    throw "warp.exe is still locked after 30s: $exePath. Close Warp and retry."
}

function Invoke-Robocopy {
    param(
        [string]$Source,
        [string]$Destination,
        [string[]]$ArgumentList
    )
    if (-not (Test-Path -LiteralPath $Destination -PathType Container)) {
        New-Item -ItemType Directory -Path $Destination | Out-Null
    }
    & robocopy.exe $Source $Destination @ArgumentList | Out-Null
    if ($LASTEXITCODE -ge 8) {
        throw "robocopy failed with exit code $LASTEXITCODE (`"$Source`" -> `"$Destination`")"
    }
}

function Copy-StagingToInstallDir {
    if (-not (Test-Path -LiteralPath (Join-Path $StagingDir 'warp.exe') -PathType Leaf)) {
        throw "Staged warp.exe is missing at $StagingDir. Run without -SkipBuild first."
    }
    if (-not (Test-Path -LiteralPath $InstallDir -PathType Container)) {
        New-Item -ItemType Directory -Path $InstallDir | Out-Null
    }

    Write-Output "Copying payload to $InstallDir (leaving unins000.* in place)"
    Invoke-Robocopy -Source $StagingDir -Destination $InstallDir -ArgumentList @('/E', '/XD', 'resources', '/NFL', '/NDL', '/NJH', '/NJS', '/NP')

    $stagedResources = Join-Path $StagingDir 'resources'
    if (Test-Path -LiteralPath $stagedResources -PathType Container) {
        $hasFiles = @(Get-ChildItem -LiteralPath $stagedResources -Recurse -File -ErrorAction SilentlyContinue).Count -gt 0
        if ($hasFiles) {
            Invoke-Robocopy -Source $stagedResources -Destination (Join-Path $InstallDir 'resources') -ArgumentList @('/E', '/MIR', '/NFL', '/NDL', '/NJH', '/NJS', '/NP')
        }
    }
}

function Update-WarpContextMenuScheme {
    $commandKeys = @(
        'Software\Classes\Directory\shell\WarpTab\command',
        'Software\Classes\Directory\Background\shell\WarpTab\command',
        'Software\Classes\Directory\shell\WarpWindow\command',
        'Software\Classes\Directory\Background\shell\WarpWindow\command'
    )
    $hives = @('HKCU:')
    if (Test-IsAdministrator) {
        $hives += 'HKLM:'
    }

    $exePath = Join-Path $InstallDir 'warp.exe'
    foreach ($hive in $hives) {
        foreach ($relative in $commandKeys) {
            $keyPath = Join-Path $hive $relative
            if (-not (Test-Path -LiteralPath $keyPath)) {
                continue
            }
            $current = [string](Get-Item -LiteralPath $keyPath).GetValue('')
            if ([string]::IsNullOrWhiteSpace($current)) {
                continue
            }
            $updated = $current -replace 'Warp://', 'warposs://' -replace 'warp://', 'warposs://'
            if ($updated -eq $current) {
                continue
            }
            Set-ItemProperty -LiteralPath $keyPath -Name '(default)' -Value $updated
            Write-Output "Rewrote context menu URI ($hive\$relative) to warposs://"
        }
    }

    $installKey = 'HKCU:\SOFTWARE\Warp.dev\Warp'
    if (Test-Path -LiteralPath $installKey) {
        Set-ItemProperty -LiteralPath $installKey -Name 'InstallationPath' -Value $exePath
    }
}

function Request-ElevationAndCopy {
    $hostExe = (Get-Process -Id $PID).Path
    $restartArg = if ($NoRestart) { '-NoRestart' } else { '-Restart' }
    $argString = "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`" -CopyOnly -SkipBuild -InstallDir `"$InstallDir`" $restartArg"
    Write-Output "Install dir is not writable; elevating to copy into $InstallDir"
    $proc = Start-Process -FilePath $hostExe -Verb RunAs -ArgumentList $argString -Wait -PassThru
    if ($null -eq $proc.ExitCode) {
        throw 'Elevated deploy process did not return an exit code.'
    }
    if ($proc.ExitCode -ne 0) {
        throw "Elevated deploy failed with exit code $($proc.ExitCode)"
    }
}

function Invoke-CopyAndFinish {
    Stop-InstalledWarp
    Copy-StagingToInstallDir
    Update-WarpContextMenuScheme
    $installed = Join-Path $InstallDir 'warp.exe'
    Write-Output "Deployed $installed"
    if ($ShouldRestart) {
        Write-Output 'Launching Warp...'
        Start-Process -FilePath $installed
    }
}

if ($Help) {
    Show-Usage
    exit 0
}

Set-Location $RepoRoot

if (-not $CopyOnly) {
    if (-not $SkipBuild) {
        $psExe = (Get-Process -Id $PID).Path
        $buildArgList = @(
            '-NoProfile',
            '-ExecutionPolicy', 'Bypass',
            '-File', $BuildScript,
            '-Bin', $Bin,
            '-Package', $Package,
            '-Features', $Features
        )
        if ($UseRelease) {
            $buildArgList += '-Release'
        }
        Write-Output "Building $Bin (release=$UseRelease features=$Features)"
        & $psExe @buildArgList
        if ($LASTEXITCODE -ne 0) {
            throw "build.ps1 failed with exit code $LASTEXITCODE"
        }
    }
    Initialize-DeployStaging
}

if ($CopyOnly -or (Test-DirectoryWritable -Path $InstallDir)) {
    Invoke-CopyAndFinish
    exit 0
}

if (Test-IsAdministrator) {
    throw "Install dir is not writable even as Administrator: $InstallDir"
}

Request-ElevationAndCopy
exit 0
