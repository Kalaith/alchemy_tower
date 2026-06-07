# RustGames project publisher wrapper.
# Build/deploy behavior lives in the workspace root publish.ps1.

param(
    [switch]$SkipBuild = $false,
    [switch]$WindowsOnly = $false,
    [switch]$WebGLOnly = $false,
    [switch]$DeployOnly = $false,
    [Alias('p')] [switch]$Production = $false,
    [switch]$FTP = $false,
    [switch]$DryRun = $false
)

$ErrorActionPreference = "Stop"
$rootPublisher = Join-Path (Split-Path $PSScriptRoot -Parent) "publish.ps1"

if (-not (Test-Path $rootPublisher)) {
    Write-Error "RustGames root publisher not found: $rootPublisher"
    exit 1
}

if (-not $SkipBuild -and -not $DeployOnly) {
    Push-Location $PSScriptRoot
    try {
        cargo test --bin alchemy_tower
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Alchemy Tower validation tests failed."
            exit 1
        }
    } finally {
        Pop-Location
    }
}

& $rootPublisher -RustGamePublish -ProjectDir $PSScriptRoot `
    -SkipBuild:$SkipBuild `
    -WindowsOnly:$WindowsOnly `
    -WebGLOnly:$WebGLOnly `
    -DeployOnly:$DeployOnly `
    -Production:$Production `
    -FTP:$FTP `
    -DryRun:$DryRun

if (-not $?) { exit 1 }
