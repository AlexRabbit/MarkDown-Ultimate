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

Write-Log "Android build attempt (Dioxus mobile scaffold)"

if (-not $env:ANDROID_HOME) {
    $guess = Join-Path $env:LOCALAPPDATA "Android\Sdk"
    if (Test-Path $guess) {
        $env:ANDROID_HOME = $guess
        Write-Log "ANDROID_HOME set to $guess"
    }
}

$dx = Get-Command dx -ErrorAction SilentlyContinue
if (-not $dx) {
    Write-Log "BLOCKER: dioxus CLI (dx) not on PATH. Install: cargo install dioxus-cli"
    $blocker = Join-Path $artifacts "ANDROID_BUILD_BLOCKER.md"
    @'
# Android APK - not built on this machine

- Missing: Dioxus CLI (dx) and/or Android SDK/NDK
- Set ANDROID_HOME after installing Android Studio SDK + NDK
- Steps:
  1. cargo install dioxus-cli
  2. cd apps/android
  3. dx build android --release
  4. Copy APK to release/artifacts/MarkDown-Ultimate.apk
'@ | Set-Content -Path $blocker -Encoding UTF8
    return
}

if (-not $env:ANDROID_HOME) {
    Write-Log "BLOCKER: ANDROID_HOME not set"
    return
}

Push-Location (Join-Path $root "apps\android")
try {
    & dx build android --release 2>&1 | Tee-Object -FilePath $log -Append
    $apk = Get-ChildItem -Path . -Recurse -Filter "*.apk" -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($apk) {
        Copy-Item -Force $apk.FullName (Join-Path $artifacts "MarkDown-Ultimate.apk")
        Write-Log "APK copied to release/artifacts/MarkDown-Ultimate.apk"
    } else {
        Write-Log "dx finished but no APK found - see log file"
    }
} catch {
    Write-Log ("Android build failed: " + $_.Exception.Message)
} finally {
    Pop-Location
}
