param([ValidateSet('X64', 'Arm64')][string] $Architecture)
$ErrorActionPreference = 'Stop'
$root = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..\..\..\..'))
$versionSource = Get-Content (Join-Path $PSScriptRoot '..\src\lib.rs') -Raw
$version = [regex]::Match($versionSource, 'LIBCLANG_VERSION: &str = "([^"]+)"').Groups[1].Value
if (!$version) { throw 'LIBCLANG_VERSION is missing' }
$commit = '0c1c4098a7aab7cb392a2367b75d447bbac05d58'
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
$source = Join-Path $cache "ClangSharp-$commit"
$build = Join-Path $cache 'build'
$bin = Join-Path $cache 'bin'
New-Item -ItemType Directory -Force $cache | Out-Null

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

if (!(Test-Path (Join-Path $sdk 'lib\cmake\clang\ClangConfig.cmake'))) {
    Fetch-Archive `
        "https://github.com/llvm/llvm-project/releases/download/llvmorg-$version/clang%2Bllvm-$version-$triple.tar.xz" `
        (Join-Path $cache 'llvm.tar.xz') $sdk
}
if (!(Test-Path (Join-Path $source 'CMakeLists.txt'))) {
    Fetch-Archive "https://api.github.com/repos/dotnet/ClangSharp/tarball/$commit" `
        (Join-Path $cache "clangsharp-$commit.tar.gz") $source
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
$command = "set `"PATH=$(Split-Path $vswhere);%PATH%`" && " +
    "call `"$devcmd`" -arch=$nativeArch -host_arch=$hostArch >nul && " +
    "`"$cmake`" -S `"$PSScriptRoot`" -B `"$build`" -G Ninja -DCMAKE_BUILD_TYPE=Release " +
    "`"-DPATH_TO_LLVM=$sdk`" `"-DCLANGSHARP_SOURCE=$source`" && " +
    "`"$cmake`" --build `"$build`" --target ClangSharp --parallel 4"
& $env:ComSpec /d /c $command
if ($LASTEXITCODE) { throw 'Native build failed' }
New-Item -ItemType Directory -Force $bin | Out-Null
Copy-Item (Join-Path $sdk 'bin\libclang.dll') $bin -Force
Copy-Item (Join-Path $build 'bin\libClangSharp.dll') $bin -Force
Copy-Item (Join-Path $source 'LICENSE.md') (Join-Path $bin 'ClangSharp-LICENSE.md') -Force
Copy-Item (Join-Path $source 'NOTICE.md') (Join-Path $bin 'ClangSharp-NOTICE.md') -Force
Copy-Item (Join-Path $sdk 'include\llvm\Support\LICENSE.TXT') (Join-Path $bin 'LLVM-LICENSE.txt') -Force
Write-Output $bin
