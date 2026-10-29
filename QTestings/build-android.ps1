$ErrorActionPreference = "Continue"
$root = Split-Path $PSScriptRoot -Parent
$artifacts = Join-Path $root "release\artifacts"
$log = Join-Path $artifacts "android-build.log"
New-Item -ItemType Directory -Path $artifacts -Force | Out-Null

function Write-Log($msg) {
    $line = "[$(Get-Date -Format o)] $msg"
    Add-Content -Path $log -Value $line
    Write-Host $line
}

Write-Log "Android APK build (dx bundle --package-types apk)"

if (-not $env:ANDROID_HOME) {
    $guess = Join-Path $env:LOCALAPPDATA "Android\Sdk"
    if (Test-Path $guess) { $env:ANDROID_HOME = $guess; Write-Log "ANDROID_HOME=$guess" }
}

$dx = Get-Command dx -ErrorAction SilentlyContinue
if (-not $dx) {
    $candidate = Join-Path $env:USERPROFILE ".cargo\bin\dx.exe"
    if (Test-Path $candidate) { $dx = $candidate }
}
if (-not $dx) {
    Write-Log "BLOCKER: cargo install dioxus-cli --locked"
    return
}

Push-Location (Join-Path $root "apps")
try {
    Write-Log "Prefetch: cargo fetch --locked"
    cargo fetch --locked 2>&1 | Out-Null
    Write-Log "Warmup: cargo metadata"
    cargo metadata --format-version 1 2>&1 | Out-Null
    Push-Location (Join-Path $root "apps\android")
    & $dx bundle --android --package-types apk -r --out-dir $artifacts 2>&1 | Tee-Object -FilePath $log -Append
    Pop-Location
    $apk = Get-ChildItem -Path $artifacts -Recurse -Filter "*.apk" -ErrorAction SilentlyContinue | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if (-not $apk) {
        $apk = Get-ChildItem -Path (Join-Path $root "apps") -Recurse -Filter "*.apk" -ErrorAction SilentlyContinue | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    }
    if ($apk) {
        Copy-Item -Force $apk.FullName (Join-Path $artifacts "MarkDown-Ultimate.apk")
        Write-Log "APK ready: release/artifacts/MarkDown-Ultimate.apk"
    } else {
        Write-Log "No APK produced. See android-build.log"
    }
} catch {
    Write-Log $_.Exception.Message
} finally {
    Pop-Location
}
