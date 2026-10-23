param(
    [ValidateSet('X64', 'Arm64')][string] $Architecture,
    [switch] $Ensure
)
$ErrorActionPreference = 'Stop'

function Get-NativeFileHash($Path) {
    $hash = [Security.Cryptography.SHA256]::Create()
    try {
        $stream = [IO.File]::OpenRead($Path)
        try {
            return [BitConverter]::ToString($hash.ComputeHash($stream)).Replace('-', '')
        } finally {
            $stream.Dispose()
        }
    } finally {
        $hash.Dispose()
    }
}

function Get-NativeBuildInputs($SourceRoot, $Version, $Triple) {
    $SourceRoot = [IO.Path]::GetFullPath($SourceRoot).TrimEnd('\')
    $inputs = @('native-build-v1', "llvm=$Version", "target=$Triple")
    foreach ($file in Get-ChildItem -LiteralPath $SourceRoot -File -Recurse | Sort-Object FullName) {
        $name = $file.FullName.Substring($SourceRoot.Length + 1)
        $hash = Get-NativeFileHash $file.FullName
        $inputs += "$name=$hash"
    }
    return ($inputs -join "`n") + "`n"
}

function Get-NativeBuildOutputs($Bin) {
    $outputs = @()
    foreach ($name in @('libclang.dll', 'windows_clang.dll')) {
        $path = Join-Path $Bin $name
        if (!(Test-Path -LiteralPath $path -PathType Leaf)) { return $null }
        $outputs += "$name=$(Get-NativeFileHash $path)"
    }
    return ($outputs -join "`n") + "`n"
}

function Invoke-NativeBuild(
    $SourceRoot, $Bin, $Version, $Triple, [scriptblock] $BuildAction, [switch] $Ensure
) {
    $cache = Split-Path $Bin
    New-Item -ItemType Directory -Force $cache | Out-Null
    $lockPath = Join-Path $cache 'build.lock'
    while ($true) {
        try {
            $lock = [IO.File]::Open($lockPath, 'OpenOrCreate', 'ReadWrite', 'None')
            break
        } catch [IO.IOException] {
            if (($_.Exception.GetBaseException().HResult -band 0xffff) -notin @(32, 33)) { throw }
            Start-Sleep -Milliseconds 100
        }
    }
    try {
        $stamp = Join-Path $Bin 'build-inputs.txt'
        $inputs = Get-NativeBuildInputs $SourceRoot $Version $Triple
        $outputs = Get-NativeBuildOutputs $Bin
        if ($Ensure -and $outputs -and (Test-Path -LiteralPath $stamp -PathType Leaf) -and
            [IO.File]::ReadAllText($stamp) -ceq ($inputs + $outputs)) {
            return
        }
        if (Test-Path -LiteralPath $stamp) { Remove-Item -LiteralPath $stamp }
        [Console]::Error.WriteLine('Building windows-clang native libraries (missing, stale, or forced).')
        & $BuildAction
        if ((Get-NativeBuildInputs $SourceRoot $Version $Triple) -cne $inputs) {
            throw 'Native build inputs changed during the build; no freshness stamp was written'
        }
        $outputs = Get-NativeBuildOutputs $Bin
        if (!$outputs) { throw 'Native build did not produce both DLLs' }
        [IO.File]::WriteAllText($stamp, $inputs + $outputs)
    } finally {
        $lock.Dispose()
    }
}

# Dot-sourcing exposes the build contract to SDK-independent provisioning tests.
if ($MyInvocation.InvocationName -eq '.') { return }

$root = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..\..\..\..'))
$versionSource = Get-Content (Join-Path $PSScriptRoot '..\src\lib.rs') -Raw
$version = [regex]::Match($versionSource, 'LIBCLANG_VERSION: &str = "([^"]+)"').Groups[1].Value
if (!$version) { throw 'LIBCLANG_VERSION is missing' }
$arch = if ($Architecture) { $Architecture } else {
    [Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()
}
$triple = switch ($arch) {
    'X64' { 'x86_64-pc-windows-msvc' }
    'Arm64' { 'aarch64-pc-windows-msvc' }
    default { throw "Unsupported native host architecture: $arch" }
}
$cache = Join-Path $root "target\tool-clang\native\$version\$triple"
$sdk = Join-Path $cache 'llvm'
$build = Join-Path $cache 'build'
$bin = Join-Path $cache 'bin'

function Fetch-Archive($url, $archive, $destination) {
    if (!(Test-Path $archive)) {
        & curl.exe --fail --location --retry 3 --silent --show-error $url --output "$archive.partial"
        if ($LASTEXITCODE) { throw "Download failed: $url" }
        Move-Item "$archive.partial" $archive
    }
    New-Item -ItemType Directory -Force $destination | Out-Null
    & tar.exe -xf $archive -C $destination --strip-components=1
    if ($LASTEXITCODE) { throw "Extraction failed: $archive" }
}

Invoke-NativeBuild $PSScriptRoot $bin $version $triple -Ensure:$Ensure -BuildAction {
    if (!(Test-Path (Join-Path $sdk 'lib\cmake\clang\ClangConfig.cmake'))) {
        Fetch-Archive `
            "https://github.com/llvm/llvm-project/releases/download/llvmorg-$version/clang%2Bllvm-$version-$triple.tar.xz" `
            (Join-Path $cache 'llvm.tar.xz') $sdk
    }
    $vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
    $vs = & $vswhere -latest -prerelease -property installationPath
    if (!$vs) { throw 'Visual Studio C++ build tools are required' }
    $devcmd = Join-Path $vs 'Common7\Tools\VsDevCmd.bat'
    $cmake = Join-Path $vs 'Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe'
    $nativeArch = if ($arch -eq 'Arm64') { 'arm64' } else { 'amd64' }
    $hostArch = if ([Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString() -eq 'Arm64') {
        'arm64'
    } else { 'amd64' }
    # Changed fingerprints can retain timestamps older than Ninja's object files.
    $command = "set `"PATH=$(Split-Path $vswhere);%PATH%`" && " +
        "call `"$devcmd`" -arch=$nativeArch -host_arch=$hostArch >nul && " +
        "`"$cmake`" -S `"$PSScriptRoot`" -B `"$build`" -G Ninja -DCMAKE_BUILD_TYPE=Release " +
        "`"-DPATH_TO_LLVM=$sdk`" && " +
        "`"$cmake`" --build `"$build`" --target windows_clang --clean-first --parallel 4"
    & $env:ComSpec /d /c $command
    if ($LASTEXITCODE) { throw 'Native build failed' }
    New-Item -ItemType Directory -Force $bin | Out-Null
    Copy-Item (Join-Path $sdk 'bin\libclang.dll') $bin -Force
    Copy-Item (Join-Path $build 'windows_clang.dll') $bin -Force
    Copy-Item (Join-Path $sdk 'include\llvm\Support\LICENSE.TXT') (Join-Path $bin 'LLVM-LICENSE.txt') -Force
}
Write-Output $bin
