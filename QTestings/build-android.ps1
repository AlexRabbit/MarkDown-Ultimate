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

Write-Log "Android phone APK (arm64-v8a for real devices)"

$jdk17 = @(
    "C:\Program Files\Eclipse Adoptium\jdk-17.0.20.101-hotspot",
    "C:\Program Files\Java\jdk-17"
) | Where-Object { Test-Path $_ } | Select-Object -First 1
if ($jdk17) {
    $env:JAVA_HOME = $jdk17
    $env:Path = "$env:JAVA_HOME\bin;" + $env:Path
    Write-Log "JAVA_HOME=$jdk17"
}

if (-not $env:ANDROID_HOME) {
    $guess = Join-Path $env:LOCALAPPDATA "Android\Sdk"
    if (Test-Path $guess) { $env:ANDROID_HOME = $guess }
}

$dx = Get-Command dx -ErrorAction SilentlyContinue
if (-not $dx) {
    $candidate = Join-Path $env:USERPROFILE ".cargo\bin\dx.exe"
    if (Test-Path $candidate) { $dx = $candidate }
}
if (-not $dx) {
    Write-Log "BLOCKER: cargo install dioxus-cli --locked"
    exit 1
}

Write-Log "Installing Rust target aarch64-linux-android..."
rustup target add aarch64-linux-android 2>&1 | Out-Null

Push-Location (Join-Path $root "apps")
try {
    cargo fetch --locked 2>&1 | Out-Null

    Write-Log "Building APK (must use --target aarch64-linux-android, not PC x86_64)..."
    $out = & $dx bundle --android --package-types apk -r -p mdu_android --target aarch64-linux-android --out-dir $artifacts 2>&1
    $out | ForEach-Object { Write-Host $_ }

    $gradleRoot = Join-Path $root "apps\target\dx\mdu_android\release\android\app"
    $apk = Join-Path $gradleRoot "app\build\outputs\apk\debug\app-debug.apk"
    if (-not (Test-Path $apk)) { throw "APK not found: $apk" }

    $dest = Join-Path $artifacts "MarkDown-Ultimate.apk"
    Copy-Item -Force $apk $dest
    Write-Log "APK: $dest"

    $so = Join-Path $gradleRoot "app\src\main\jniLibs\arm64-v8a\libmain.so"
    if (Test-Path $so) { Write-Log "Native: $so" } else { Write-Log "WARN: missing arm64-v8a lib" }
} catch {
    Write-Log $_.Exception.Message
    exit 1
} finally {
    Pop-Location
}
