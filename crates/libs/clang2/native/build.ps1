param([Parameter(Mandatory = $true)][string]$Version)
$ErrorActionPreference = 'Stop'
$pin = Get-Content -Raw (Join-Path $PSScriptRoot 'llvm.json') | ConvertFrom-Json
if ($Version -ne $pin.Version) { throw 'Native LLVM pin differs from the generator pin' }
if ($env:PROCESSOR_ARCHITECTURE -ne 'AMD64') { throw 'The native layout build supports x64 hosts' }
$repository = (Resolve-Path (Join-Path $PSScriptRoot '..\..\..\..')).Path
$cache = Join-Path $repository "target\tool-clang\native-layout\$Version"
New-Item -ItemType Directory -Force $cache | Out-Null
$archive = Join-Path $cache $pin.Archive
if (!(Test-Path $archive)) {
    $url = "https://github.com/llvm/llvm-project/releases/download/llvmorg-$Version/$($pin.Archive)"
    & curl.exe --fail --silent --show-error --location --max-time 600 --output $archive $url
    if ($LASTEXITCODE -ne 0) { throw 'LLVM development download failed' }
}
$algorithm = [Security.Cryptography.SHA256]::Create()
$stream = [IO.File]::OpenRead($archive)
try {
    $hash = [BitConverter]::ToString($algorithm.ComputeHash($stream)).Replace('-', '').ToLowerInvariant()
} finally {
    $stream.Dispose()
    $algorithm.Dispose()
}
if ($hash -ne $pin.Sha256) { throw "LLVM development checksum mismatch: $hash" }
$sdk = Join-Path $cache ([IO.Path]::GetFileNameWithoutExtension(
    [IO.Path]::GetFileNameWithoutExtension($pin.Archive)))
$complete = Join-Path $cache 'sdk-complete'
if (Test-Path $complete) {
    if ((Get-Content -Raw $complete).Trim() -ne $hash) { throw 'LLVM extraction pin mismatch' }
} else {
    & tar.exe -xf $archive -C $cache
    if ($LASTEXITCODE -ne 0) { throw 'LLVM development extraction failed' }
    Set-Content $complete $hash
}
$source = Join-Path $cache 'source'
if (!(Test-Path $source)) {
    & git clone --quiet --filter=blob:none --no-checkout --depth 1 `
        --branch "llvmorg-$Version" https://github.com/llvm/llvm-project $source
    if ($LASTEXITCODE -ne 0) { throw 'LLVM source checkout failed' }
    & git -C $source sparse-checkout set --cone clang/tools/libclang
    if ($LASTEXITCODE -ne 0) { throw 'LLVM source selection failed' }
    & git -C $source checkout --quiet
    if ($LASTEXITCODE -ne 0) { throw 'LLVM source materialization failed' }
}
$revision = & git -C $source rev-parse HEAD
if ($LASTEXITCODE -ne 0 -or $revision -ne $pin.Revision) { throw 'LLVM source revision mismatch' }
$changes = & git -C $source status --porcelain
if ($LASTEXITCODE -ne 0 -or $changes) { throw 'Pinned LLVM source checkout is not clean' }
$finder = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
$visualStudio = & $finder -latest -products '*' `
    -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if ($LASTEXITCODE -ne 0 -or !$visualStudio) { throw 'MSVC build tools are unavailable' }
$installationVersion = & $finder -latest -products '*' `
    -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationVersion
if ($LASTEXITCODE -ne 0) { throw 'MSVC version discovery failed' }
$major = ([version]$installationVersion).Major
$generator = switch ($major) {
    17 { 'Visual Studio 17 2022' }
    18 { 'Visual Studio 18 2026' }
    default { throw "Unsupported MSVC generator version $major" }
}
$cmake = Join-Path $visualStudio 'Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe'
$build = Join-Path $cache 'build'
& $cmake -S $PSScriptRoot -B $build -G $generator -A x64 `
    "-DLLVM_DIR=$sdk\lib\cmake\llvm" "-DClang_DIR=$sdk\lib\cmake\clang" `
    "-DLIBCLANG_SOURCE=$source\clang\tools\libclang"
if ($LASTEXITCODE -ne 0) { throw 'Native layout configuration failed' }
& $cmake --build $build --config Release --parallel 2
if ($LASTEXITCODE -ne 0) { throw 'Native layout compilation failed' }
$native = Join-Path $sdk 'bin'
Copy-Item (Join-Path $build 'Release\windows_clang_layout.dll') $native -Force
foreach ($name in 'bad_abi', 'bad_hash') {
    Copy-Item (Join-Path $build "Release\windows_clang_layout_$name.dll") $native -Force
}
Write-Output $native
