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

$jdk17 = @(
    "C:\Program Files\Eclipse Adoptium\jdk-17.0.20.101-hotspot",
    "C:\Program Files\Java\jdk-17"
) | Where-Object { Test-Path $_ } | Select-Object -First 1
if ($jdk17) {
    $env:JAVA_HOME = $jdk17
    $env:Path = "$jdk17\bin;" + $env:Path
    Write-Log "JAVA_HOME=$jdk17 (Gradle requires JDK 17 for Android)"
} elseif ($env:JAVA_HOME -match 'jdk-2[789]|jdk-21') {
    Write-Log "WARN: JAVA_HOME looks too new for Android Gradle; install JDK 17"
}

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
    Write-Log "dx bundle starting..."
    $dxLog = Join-Path $env:TEMP ("mdu-android-dx-{0}.log" -f [guid]::NewGuid().ToString('N'))
    $output = & $dx bundle --android --package-types apk -r -p mdu_android --out-dir $artifacts 2>&1
    $output | Out-File -FilePath $dxLog -Encoding utf8
    $output | ForEach-Object { Write-Host $_ }
    Write-Log "dx log: $dxLog"
    $apk = Get-ChildItem -Path $artifacts -Recurse -Filter "*.apk" -ErrorAction SilentlyContinue | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if (-not $apk) {
        $apk = Get-ChildItem -Path (Join-Path $root "apps") -Recurse -Filter "*.apk" -ErrorAction SilentlyContinue | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    }
    if ($apk) {
        $dest = Join-Path $artifacts "MarkDown-Ultimate.apk"
        if ($apk.FullName -ne (Resolve-Path $dest -ErrorAction SilentlyContinue)) {
            Copy-Item -Force $apk.FullName $dest
        } elseif ($apk.Name -ne "MarkDown-Ultimate.apk") {
            Copy-Item -Force $apk.FullName $dest
        }
        Write-Log "APK ready: release/artifacts/MarkDown-Ultimate.apk"
    } else {
        Write-Log "No APK produced. See android-build.log"
    }
} catch {
    Write-Log $_.Exception.Message
} finally {
    Pop-Location
}
