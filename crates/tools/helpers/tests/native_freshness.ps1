$ErrorActionPreference = 'Stop'
$buildScript = [IO.Path]::GetFullPath(
    (Join-Path $PSScriptRoot '..\..\..\libs\clang-sys\native\build.ps1')
)
. $buildScript

function Assert($Condition, $Message) {
    if (!$Condition) { throw $Message }
}

$scratch = Join-Path ([IO.Path]::GetTempPath()) "windows-clang-freshness-$([guid]::NewGuid())"
$sourceRoot = Join-Path $scratch 'native'
$bin = Join-Path $scratch 'cache\bin'
$stamp = Join-Path $bin 'build-inputs.txt'
$state = @{ Count = 0 }
$action = {
    $state.Count++
    New-Item -ItemType Directory -Force $bin | Out-Null
    [IO.File]::WriteAllText((Join-Path $bin 'libclang.dll'), 'runtime')
    [IO.File]::WriteAllText((Join-Path $bin 'libClangSharp.dll'), "companion-$($state.Count)")
}

function Ensure-Build($Version = '1.0', $Triple = 'x64') {
    Invoke-NativeBuild $sourceRoot $bin $Version $Triple -BuildAction $action -Ensure
}

function Assert-Rebuild {
    $before = $state.Count
    Ensure-Build
    Assert ($state.Count -eq $before + 1) 'Expected exactly one build'
    Ensure-Build
    Assert ($state.Count -eq $before + 1) 'Unchanged build was not reused'
}

function Assert-FailedBuild([scriptblock] $BuildAction, $Expected) {
    $message = ''
    try {
        Invoke-NativeBuild $sourceRoot $bin '1.0' 'x64' -BuildAction $BuildAction
    } catch {
        $message = $_.Exception.Message
    }
    Assert ($message -like "*$Expected*") "Wrong failure: $message"
    Assert (!(Test-Path -LiteralPath $stamp)) 'Failed build retained a freshness stamp'
    Assert-Rebuild
}

try {
    New-Item -ItemType Directory -Force $sourceRoot | Out-Null
    foreach ($name in @('extension.cpp', 'extension.h', 'CMakeLists.txt', 'build.ps1')) {
        [IO.File]::WriteAllText((Join-Path $sourceRoot $name), "original $name")
    }
    Assert-Rebuild
    $stampTime = [datetime]::new(2000, 1, 1, 0, 0, 0, [DateTimeKind]::Utc)
    [IO.File]::SetLastWriteTimeUtc($stamp, $stampTime)
    Ensure-Build
    Assert ([IO.File]::GetLastWriteTimeUtc($stamp) -eq $stampTime) 'Reuse rewrote the stamp'

    foreach ($name in @('extension.cpp', 'extension.h', 'CMakeLists.txt', 'build.ps1')) {
        $path = Join-Path $sourceRoot $name
        $time = [IO.File]::GetLastWriteTimeUtc($path)
        [IO.File]::WriteAllText($path, "modified $name")
        [IO.File]::SetLastWriteTimeUtc($path, $time)
        Assert-Rebuild
    }
    $extra = Join-Path $sourceRoot 'extra.h'
    [IO.File]::WriteAllText($extra, 'added source')
    Assert-Rebuild
    Remove-Item -LiteralPath $extra
    Assert-Rebuild

    $before = $state.Count
    Ensure-Build '2.0'
    Assert ($state.Count -eq $before + 1) 'LLVM version change was not detected'
    Assert-Rebuild
    $before = $state.Count
    Ensure-Build '1.0' 'arm64'
    Assert ($state.Count -eq $before + 1) 'Architecture change was not detected'
    Assert-Rebuild

    foreach ($name in @('libclang.dll', 'libClangSharp.dll')) {
        $path = Join-Path $bin $name
        [IO.File]::WriteAllText($path, 'replaced DLL')
        Assert-Rebuild
        Remove-Item -LiteralPath $path
        Assert-Rebuild
    }
    Remove-Item -LiteralPath $stamp
    Assert-Rebuild
    [IO.File]::WriteAllText($stamp, 'invalid stamp')
    Assert-Rebuild
    $before = $state.Count
    Invoke-NativeBuild $sourceRoot $bin '1.0' 'x64' -BuildAction $action
    Assert ($state.Count -eq $before + 1) 'Explicit build did not rebuild'

    Assert-FailedBuild { throw 'fixture build failure' } 'fixture build failure'
    Assert-FailedBuild {
        & $action
        [IO.File]::WriteAllText((Join-Path $sourceRoot 'extension.cpp'), 'changed during build')
    } 'inputs changed during the build'
    Assert-FailedBuild {
        & $action
        Remove-Item -LiteralPath (Join-Path $bin 'libClangSharp.dll')
    } 'did not produce both DLLs'

    $lock = [IO.File]::Open((Join-Path (Split-Path $bin) 'build.lock'), 'Open', 'ReadWrite', 'None')
    $worker = [PowerShell]::Create()
    try {
        $ready = Join-Path $scratch 'worker-ready'
        $null = $worker.AddScript({
            param($BuildScript, $SourceRoot, $Bin, $Ready)
            . $BuildScript
            [IO.File]::WriteAllText($Ready, 'ready')
            Invoke-NativeBuild $SourceRoot $Bin '1.0' 'x64' -Ensure -BuildAction {
                throw 'Concurrent check did not reuse the completed build'
            }
        }).AddArgument($buildScript).AddArgument($sourceRoot).AddArgument($bin).AddArgument($ready)
        $pending = $worker.BeginInvoke()
        $deadline = [datetime]::UtcNow.AddSeconds(30)
        while (!(Test-Path -LiteralPath $ready)) {
            if ($pending.IsCompleted) { throw "Worker failed: $($worker.Streams.Error)" }
            Assert ([datetime]::UtcNow -lt $deadline) 'Worker did not start'
            Start-Sleep -Milliseconds 10
        }
        Assert (!$pending.AsyncWaitHandle.WaitOne(100)) 'Concurrent check bypassed the cache lock'
        $lock.Dispose()
        Assert ($pending.AsyncWaitHandle.WaitOne(30000)) 'Concurrent check did not resume'
        $null = $worker.EndInvoke($pending)
        Assert (!$worker.HadErrors) "Concurrent check failed: $($worker.Streams.Error)"
    } finally {
        $lock.Dispose()
        $worker.Stop()
        $worker.Dispose()
    }

    Write-Output 'Native freshness regressions passed'
} finally {
    if (Test-Path -LiteralPath $scratch) { Remove-Item -LiteralPath $scratch -Recurse -Force }
}
