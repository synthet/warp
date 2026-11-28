#!/usr/bin/env powershell
<#
.SYNOPSIS
    Build Synth Term (OSS) and install it into its own per-user folder.

.DESCRIPTION
    Builds warp-oss (unless -SkipBuild), stages it with the Windows payload DLLs and resources,
    then copies it into %LOCALAPPDATA%\Programs\SynthTerm. Synth Term runs side by side with any
    official Warp install: the official folder, its processes, shortcuts and registry entries are
    never touched. Re-run after each build to update the install.

.EXAMPLE
    .\script\windows\deploy.ps1
    .\script\windows\deploy.ps1 -SkipBuild
    .\script\windows\deploy.ps1 -WhatIf
#>
param(
    [switch]$Help,
    [switch]$SkipBuild,
    [switch]$Debug,
    [Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSReviewUnusedParameter', 'Release')]
    [switch]$Release,
    [Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSReviewUnusedParameter', 'Restart')]
    [switch]$Restart,
    [switch]$NoRestart,
    [switch]$NoShortcut,
    [switch]$NoContextMenu,
    [switch]$Force,
    [switch]$WhatIf,
    [string]$InstallDir = '',
    [string]$Features = 'release_bundle,gui',
    [string]$Bin = 'warp-oss',
    [string]$Package = 'warp'
)

$ErrorActionPreference = 'Stop'
if (Get-Variable -Name PSNativeCommandUseErrorActionPreference -ErrorAction SilentlyContinue) {
    $PSNativeCommandUseErrorActionPreference = $false
}

$ProductName = 'Synth Term'
# Must match ChannelState::app_id() for the OSS build so the taskbar and notifications attribute
# windows to this shortcut.
$AppUserModelId = 'io.github.synthet.Warp'
$UrlScheme = 'warposs'
$RegistryKeyName = 'SynthTerm'

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$TargetDir = Join-Path $RepoRoot 'target'
$StagingDir = Join-Path $TargetDir 'warp-deploy-staging'
$BuildScript = Join-Path $PSScriptRoot 'build.ps1'
$PrepareResourcesScript = Join-Path $PSScriptRoot 'prepare_bundled_resources.ps1'
. (Join-Path $PSScriptRoot 'deploy_compliance.ps1')

if (-not $InstallDir) {
    $InstallDir = Join-Path $env:LOCALAPPDATA 'Programs\SynthTerm'
}
$InstallDir = [System.IO.Path]::GetFullPath($InstallDir)
$ExeName = "$Bin.exe"
$InstalledExe = Join-Path $InstallDir $ExeName

# -Release is the default; -Debug opts into a faster non-release build.
$UseRelease = if ($Release) { $true } else { -not $Debug }
# -Restart is the default; -NoRestart skips the launch.
$ShouldRestart = if ($Restart) { $true } else { -not $NoRestart }

function Show-Usage {
    Write-Output @'
Usage: .\script\windows\deploy.ps1 [options]

Build Synth Term (OSS) and install it into %LOCALAPPDATA%\Programs\SynthTerm, side by side
with any official Warp install, which is never touched.

Options:
  -Help              Show this help.
  -SkipBuild         Do not compile; install the last warp-oss.exe.
  -Release           Build with --release (default unless -Debug).
  -Debug             Faster non-release build.
  -Restart           Launch Synth Term after the copy (default).
  -NoRestart         Do not launch after the copy.
  -NoShortcut        Do not create the "Synth Term" Start menu shortcut.
  -NoContextMenu     Do not add the "Open Synth Term" Explorer folder entries.
  -Force             Install into a non-empty folder that has no warp-oss.exe.
  -WhatIf            Print the plan without stopping processes or writing files.
  -InstallDir PATH   Install directory (default: %LOCALAPPDATA%\Programs\SynthTerm).
  -Features LIST     Cargo features (default: release_bundle,gui).
  -Bin NAME          Cargo binary (default: warp-oss).
  -Package NAME      Cargo package (default: warp).

Examples:
  .\script\windows\deploy.ps1
  .\script\windows\deploy.ps1 -SkipBuild
  .\script\windows\deploy.ps1 -WhatIf
'@
}

function Write-Plan {
    param([string]$Action)
    Write-Output "What if: $Action"
}

function Get-WindowsPayloadArch {
    if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64') {
        return 'arm64'
    }
    return 'x64'
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

function Invoke-PrepareResource {
    param([string]$DestinationDir, [string]$SchemaExecutable)
    $bundledSource = Join-Path $RepoRoot 'resources\bundled'
    $bundledDest = Join-Path $DestinationDir 'bundled'
    if (-not (Test-Path -LiteralPath $bundledSource -PathType Container)) {
        throw "Bundled resources not found at $bundledSource"
    }
    if (-not (Test-Path -LiteralPath $DestinationDir -PathType Container)) {
        New-Item -ItemType Directory -Path $DestinationDir | Out-Null
    }

    try {
        # prepare_bundled_resources.ps1 runs the built app to dump the settings schema, as bundle.ps1 does.
        $previousSchemaExe = $env:SETTINGS_SCHEMA_EXECUTABLE
        $env:SETTINGS_SCHEMA_EXECUTABLE = $SchemaExecutable
        try {
            & $PrepareResourcesScript -DestinationDir $DestinationDir -Channel oss
        } finally {
            $env:SETTINGS_SCHEMA_EXECUTABLE = $previousSchemaExe
        }
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


function Test-InstallLayout {
    if (-not (Test-Path -LiteralPath $InstallDir)) {
        return $true
    }
    if (Test-Path -LiteralPath $InstalledExe -PathType Leaf) {
        return $true
    }
    @(Get-ChildItem -LiteralPath $InstallDir -Force).Count -eq 0
}

function Initialize-DeployStaging {
    $arch = Get-WindowsPayloadArch
    $assetsDir = Join-Path $RepoRoot "app\assets\windows\$arch"
    $exeSource = Get-BuiltBinaryPath

    if (Test-Path -LiteralPath $StagingDir -PathType Container) {
        Remove-Item -LiteralPath $StagingDir -Recurse -Force
    }
    New-Item -ItemType Directory -Path $StagingDir | Out-Null

    Write-Output "Staging $exeSource as $ExeName"
    Copy-PayloadFile -Source $exeSource -Destination (Join-Path $StagingDir $ExeName) -Required

    foreach ($asset in @(
            @{ Src = 'conpty.dll'; Dst = 'conpty.dll' },
            @{ Src = 'vcruntime140.dll'; Dst = 'vcruntime140.dll' },
            @{ Src = 'vcruntime140_1.dll'; Dst = 'vcruntime140_1.dll' },
            @{ Src = 'msvcp140.dll'; Dst = 'msvcp140.dll' },
            @{ Src = 'dxcompiler.dll'; Dst = 'dxcompiler.dll' },
            @{ Src = 'dxil.dll'; Dst = 'dxil.dll' },
            @{ Src = 'OpenConsole.exe'; Dst = "$arch\OpenConsole.exe" }
        )) {
        Copy-PayloadFile -Source (Join-Path $assetsDir $asset.Src) -Destination (Join-Path $StagingDir $asset.Dst) -Required
    }

    Copy-PayloadFile -Source (Join-Path $RepoRoot 'app\channels\oss\icon\no-padding\icon.ico') `
        -Destination (Join-Path $StagingDir 'icon.ico') -Required
    Copy-PayloadFile -Source (Join-Path $RepoRoot 'app\assets\bundled\bootstrap\pwsh.ps1') `
        -Destination (Join-Path $StagingDir 'pwsh.ps1') -Required
    foreach ($notice in @('LICENSE-AGPL', 'LICENSE-MIT', 'legal\synth-warp-notice.md')) {
        Copy-PayloadFile -Source (Join-Path $RepoRoot $notice) -Destination (Join-Path $StagingDir (Split-Path -Leaf $notice))
    }

    Write-Output 'Preparing bundled resources...'
    Invoke-PrepareResource -DestinationDir (Join-Path $StagingDir 'resources') -SchemaExecutable $exeSource
    Write-SourceRecord -StagingDir $StagingDir -RepoRoot $RepoRoot
    Assert-StagedCompliance -StagingDir $StagingDir

    Write-CliShim -Path (Join-Path $StagingDir "bin\$Bin.cmd") -ExePath $InstalledExe
}

function Invoke-StopSynthWarp {
    # Only processes started from this install, so official Warp keeps running.
    $prefix = $InstallDir.TrimEnd('\') + '\'
    $procs = @(Get-Process -ErrorAction SilentlyContinue | Where-Object {
            $path = $null
            try { $path = $_.Path } catch { $path = $null }
            $path -and $path.StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)
        })
    if ($procs.Count -eq 0) {
        return
    }
    $ids = ($procs | ForEach-Object { $_.Id }) -join ', '
    if ($WhatIf) {
        Write-Plan "stop $ProductName processes $ids"
        return
    }
    Write-Output "Stopping $ProductName processes: $ids"
    $procs | Stop-Process -Force -ErrorAction SilentlyContinue

    $deadline = (Get-Date).AddSeconds(30)
    while ((Get-Date) -lt $deadline) {
        try {
            $stream = [System.IO.File]::Open($InstalledExe, 'Open', 'ReadWrite', 'None')
            $stream.Close()
            return
        } catch [System.IO.FileNotFoundException] {
            return
        } catch {
            Start-Sleep -Milliseconds 400
        }
    }
    throw "$InstalledExe is still locked after 30s. Close $ProductName and retry."
}

function Copy-StagingToInstallDir {
    if ($WhatIf) {
        Write-Plan "copy $StagingDir to $InstallDir"
        return
    }
    Write-Output "Copying payload to $InstallDir"
    Invoke-Robocopy -Source $StagingDir -Destination $InstallDir -ArgumentList @('/E', '/XD', 'resources', '/NFL', '/NDL', '/NJH', '/NJS', '/NP')
    $stagedResources = Join-Path $StagingDir 'resources'
    if (Test-Path -LiteralPath $stagedResources -PathType Container) {
        Invoke-Robocopy -Source $stagedResources -Destination (Join-Path $InstallDir 'resources') -ArgumentList @('/E', '/MIR', '/NFL', '/NDL', '/NJH', '/NJS', '/NP')
    }
}

function Invoke-SetShortcutAppUserModelId {
    param([string]$LinkPath, [string]$AppId)
    if (-not ('SynthWarp.ShortcutAppId' -as [type])) {
        Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
namespace SynthWarp {
    [ComImport, Guid("886D8EEB-8CF2-4446-8D02-CDBA1DBDCF99"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
    interface IPropertyStore {
        int GetCount(out uint count);
        int GetAt(uint index, out PropertyKey key);
        int GetValue(ref PropertyKey key, [Out] PropVariant value);
        int SetValue(ref PropertyKey key, [In] PropVariant value);
        int Commit();
    }
    [StructLayout(LayoutKind.Sequential, Pack = 4)]
    public struct PropertyKey { public Guid FormatId; public uint PropertyId; }
    [StructLayout(LayoutKind.Sequential)]
    public sealed class PropVariant : IDisposable {
        ushort vt; ushort r1; ushort r2; ushort r3; IntPtr value; IntPtr value2;
        public PropVariant(string text) { vt = 31; value = Marshal.StringToCoTaskMemUni(text); }
        public void Dispose() { if (value != IntPtr.Zero) { Marshal.FreeCoTaskMem(value); value = IntPtr.Zero; } }
    }
    public static class ShortcutAppId {
        [DllImport("shell32.dll", CharSet = CharSet.Unicode, PreserveSig = false)]
        static extern void SHGetPropertyStoreFromParsingName(string path, IntPtr bindContext, int flags, ref Guid iid, [MarshalAs(UnmanagedType.Interface)] out IPropertyStore store);
        public static void Set(string linkPath, string appId) {
            Guid iid = typeof(IPropertyStore).GUID;
            IPropertyStore store;
            SHGetPropertyStoreFromParsingName(linkPath, IntPtr.Zero, 2 /* GPS_READWRITE */, ref iid, out store);
            PropertyKey key = new PropertyKey { FormatId = new Guid("9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3"), PropertyId = 5 };
            using (PropVariant value = new PropVariant(appId)) {
                Marshal.ThrowExceptionForHR(store.SetValue(ref key, value));
            }
            Marshal.ThrowExceptionForHR(store.Commit());
            Marshal.ReleaseComObject(store);
        }
    }
}
'@
    }
    [SynthWarp.ShortcutAppId]::Set($LinkPath, $AppId)
}

function Invoke-CreateStartMenuShortcut {
    $link = Join-Path ([Environment]::GetFolderPath('Programs')) "$ProductName.lnk"
    if ($WhatIf) {
        Write-Plan "create Start menu shortcut $link -> $InstalledExe"
        return
    }
    $shell = [Activator]::CreateInstance([Type]::GetTypeFromProgID('WScript.Shell'))
    $shortcut = $shell.CreateShortcut($link)
    $shortcut.TargetPath = $InstalledExe
    $shortcut.WorkingDirectory = $InstallDir
    $shortcut.IconLocation = (Join-Path $InstallDir 'icon.ico')
    $shortcut.Description = $ProductName
    $shortcut.Save()
    try {
        Invoke-SetShortcutAppUserModelId -LinkPath $link -AppId $AppUserModelId
    } catch {
        Write-Warning "Shortcut created, but setting its AppUserModelID failed: $($_.Exception.Message)"
    }
    Write-Output "Start menu shortcut: $link"
}

function Register-ExplorerEntry {
    $root = 'HKCU:\Software\Classes\Directory'
    $entries = @(
        @{ Key = "shell\$($RegistryKeyName)Tab"; Label = "Open $ProductName in new tab"; Action = 'new_tab'; Path = '%1' },
        @{ Key = "Background\shell\$($RegistryKeyName)Tab"; Label = "Open $ProductName in new tab"; Action = 'new_tab'; Path = '%V' },
        @{ Key = "shell\$($RegistryKeyName)Window"; Label = "Open $ProductName in new window"; Action = 'new_window'; Path = '%1' },
        @{ Key = "Background\shell\$($RegistryKeyName)Window"; Label = "Open $ProductName in new window"; Action = 'new_window'; Path = '%V' }
    )
    foreach ($entry in $entries) {
        $key = Join-Path $root $entry.Key
        $command = "`"$InstalledExe`" `"$($UrlScheme)://action/$($entry.Action)?path=$($entry.Path)`""
        if ($WhatIf) {
            Write-Plan "set $key -> $command"
            continue
        }
        New-Item -Path "$key\command" -Force | Out-Null
        Set-ItemProperty -LiteralPath $key -Name '(default)' -Value $entry.Label
        Set-ItemProperty -LiteralPath $key -Name 'Icon' -Value (Join-Path $InstallDir 'icon.ico')
        Set-ItemProperty -LiteralPath "$key\command" -Name '(default)' -Value $command
    }
    $appPaths = "HKCU:\Software\Microsoft\Windows\CurrentVersion\App Paths\$ExeName"
    if ($WhatIf) {
        Write-Plan "set $appPaths -> $InstalledExe"
        return
    }
    New-Item -Path $appPaths -Force | Out-Null
    Set-ItemProperty -LiteralPath $appPaths -Name '(default)' -Value $InstalledExe
}

if ($Help) {
    Show-Usage
    exit 0
}

Set-Location $RepoRoot

if (-not $Force -and -not (Test-InstallLayout)) {
    throw "$InstallDir is not empty and has no $ExeName. Pass -Force to install anyway, or -InstallDir to choose another folder."
}

Write-Output "Deploying $ProductName"
Write-Output "  install dir: $InstallDir"
Write-Output "  build:       $(-not $SkipBuild) (release=$UseRelease features=$Features)"

if (-not $SkipBuild) {
    if ($WhatIf) {
        Write-Plan "build $Bin (release=$UseRelease features=$Features)"
    } else {
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
}

if ($WhatIf) {
    Write-Plan "stage $Bin.exe, payload DLLs, icon, notices and resources in $StagingDir"
    Write-Plan "record the source commit in SOURCE.txt; fail if any of $($RequiredStagedNotices -join ', ') is missing"
} else {
    Initialize-DeployStaging
}

Invoke-StopSynthWarp
Copy-StagingToInstallDir
if (-not $NoShortcut) {
    Invoke-CreateStartMenuShortcut
}
if (-not $NoContextMenu) {
    Register-ExplorerEntry
}

if ($WhatIf) {
    exit 0
}

$info = (Get-Item -LiteralPath $InstalledExe).VersionInfo
Write-Output "Installed $($info.ProductName) $($info.FileVersion) at $InstalledExe"
if ($ShouldRestart) {
    Write-Output "Launching $ProductName..."
    Start-Process -FilePath $InstalledExe
}
exit 0
