$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true

$repo = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..\..")).Path
$package = Join-Path $repo "target\package\windows-reactor-0.100.0.crate"
$temporary = Join-Path $repo "target\reactor-package-validation-$PID"
$source = Join-Path $temporary "windows-reactor-0.100.0"
$consumer = Join-Path $temporary "consumer"

$patches = @(
    "--config", "patch.crates-io.windows.path=`"crates/libs/windows`""
    "--config", "patch.crates-io.windows-collections.path=`"crates/libs/collections`""
    "--config", "patch.crates-io.windows-composition.path=`"crates/libs/composition`""
    "--config", "patch.crates-io.windows-core.path=`"crates/libs/core`""
    "--config", "patch.crates-io.windows-future.path=`"crates/libs/future`""
    "--config", "patch.crates-io.windows-numerics.path=`"crates/libs/numerics`""
    "--config", "patch.crates-io.windows-reference.path=`"crates/libs/reference`""
    "--config", "patch.crates-io.windows-threading.path=`"crates/libs/threading`""
    "--config", "patch.crates-io.windows-time.path=`"crates/libs/time`""
    "--config", "patch.crates-io.windows-canvas.path=`"crates/libs/canvas`""
    "--config", "patch.crates-io.windows-webview.path=`"crates/libs/webview`""
)

try {
    Push-Location $repo
    & cargo package -p windows-reactor --allow-dirty --no-verify @patches
    if ($LASTEXITCODE -ne 0) {
        throw "failed to package windows-reactor"
    }

    New-Item -ItemType Directory -Path $temporary | Out-Null
    & tar -xf $package -C $temporary
    if ($LASTEXITCODE -ne 0) {
        throw "failed to extract the windows-reactor package"
    }
    Add-Content -Path (Join-Path $source "Cargo.toml") -Value "`n[workspace]"

    & cargo check --manifest-path (Join-Path $source "Cargo.toml") --lib --quiet @patches
    if ($LASTEXITCODE -ne 0) {
        throw "the default packaged library does not compile"
    }
    & cargo check --manifest-path (Join-Path $source "Cargo.toml") --lib --all-features --quiet @patches
    if ($LASTEXITCODE -ne 0) {
        throw "the all-feature packaged library does not compile"
    }

    New-Item -ItemType Directory -Path (Join-Path $consumer "src") | Out-Null
    @"
[package]
name = "windows-reactor-package-consumer"
version = "0.0.0"
edition = "2024"

[dependencies]
windows-reactor = { path = "$($source.Replace('\', '/'))" }

[workspace]
"@ | Set-Content -Path (Join-Path $consumer "Cargo.toml") -Encoding utf8
    @'
use windows_reactor::{Application, Window, text_block};

fn main() {
    let root =
        Application::new([Window::new("Packaged source", text_block("Content"), || {}).build()])
            .build();
    drop(root);
}
'@ | Set-Content -Path (Join-Path $consumer "src\main.rs") -Encoding utf8

    & cargo run --manifest-path (Join-Path $consumer "Cargo.toml") --quiet @patches
    if ($LASTEXITCODE -ne 0) {
        throw "the packaged public-API consumer failed"
    }
}
finally {
    Pop-Location
    if (Test-Path $temporary) {
        Remove-Item -Recurse -Force $temporary
    }
}
