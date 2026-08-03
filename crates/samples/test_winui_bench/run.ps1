param(
    [int]$Runs = 3,
    [int]$Iterations = 100000,
    [int]$CreateIterations = 1000,
    [int]$TreeIterations = 20,
    [int]$TreeSize = 100,
    [ValidateRange(1, [int]::MaxValue)]
    [int]$StressIterations = 100,
    [ValidateRange(1, [int]::MaxValue)]
    [int]$SustainedSeconds = 3,
    [ValidateRange(0, 100)]
    [int]$SustainedPercent = 10,
    [ValidateRange(0, 4900)]
    [int]$SustainedChurn = 0,
    [ValidateRange(1, [int]::MaxValue)]
    [int]$BenchmarkBudgetMs = 30000,
    [int]$SettleMs = 750,
    [switch]$Headless
)

$ErrorActionPreference = "Stop"
$root = (Resolve-Path "$PSScriptRoot\..\..\..").Path

Push-Location $root
try {
    cargo build --release -p test_winui_bench_host -p test_winui_bench_rust `
        -p test_winui_bench_csharp --quiet
    if ($LASTEXITCODE -ne 0) {
        throw "Rust build failed."
    }

    dotnet build `
        crates\samples\test_winui_bench\csharp\test_winui_bench_csharp.csproj `
        -c Release -p:Platform=x64 --nologo --verbosity:quiet
    if ($LASTEXITCODE -ne 0) {
        throw "windows-csharp build failed."
    }

    dotnet build `
        crates\samples\test_winui_bench\cswinrt\test_winui_bench_cswinrt.csproj `
        -c Release -p:Platform=x64 --nologo --verbosity:quiet
    if ($LASTEXITCODE -ne 0) {
        throw "CsWinRT build failed."
    }

    $target = Join-Path $root "target\release"
    $outputs = @{
        "windows-rs" = $target
        "windows-csharp" = Join-Path $root `
            "crates\samples\test_winui_bench\csharp\bin\x64\Release\net10.0-windows10.0.19041.0"
        "cswinrt" = Join-Path $root `
            "crates\samples\test_winui_bench\cswinrt\bin\x64\Release\net10.0-windows10.0.19041.0"
    }

    foreach ($consumer in @("windows-csharp", "cswinrt")) {
        Copy-Item "$target\test_winui_bench_host.dll" $outputs[$consumer] -Force
        Copy-Item "$target\resources.pri" $outputs[$consumer] -Force
    }
    Copy-Item "$target\Microsoft.WindowsAppRuntime.Bootstrap.dll" `
        $outputs["windows-csharp"] -Force

    $executables = @{
        "windows-rs" = "$target\test_winui_bench_rust.exe"
        "windows-csharp" = Join-Path $outputs["windows-csharp"] "test_winui_bench_csharp.exe"
        "cswinrt" = Join-Path $outputs["cswinrt"] "test_winui_bench_cswinrt.exe"
    }

    $results = @()
    function Invoke-Consumer([string]$Consumer, [string[]]$Arguments) {
        $start = [System.Diagnostics.ProcessStartInfo]::new()
        $start.FileName = $executables[$Consumer]
        $start.UseShellExecute = $false
        $start.CreateNoWindow = $true
        $start.RedirectStandardOutput = $true
        $start.RedirectStandardError = $true
        foreach ($argument in $Arguments) {
            $start.ArgumentList.Add($argument)
        }
        $process = [System.Diagnostics.Process]::Start($start)
        $stderr = $process.StandardError.ReadToEndAsync()
        $timeoutMs = [Math]::Max(
            30000,
            $SustainedSeconds * 1000 + $SettleMs + $BenchmarkBudgetMs)
        $result = $process.StandardOutput.ReadLineAsync()
        if (-not $result.Wait($timeoutMs)) {
            Stop-Process -Id $process.Id -Force
            $process.WaitForExit()
            throw "$Consumer timed out before producing a benchmark result:`n$($stderr.Result)"
        }
        $line = $result.Result
        if ($null -eq $line -or -not $line.StartsWith("WINUI_BENCH_JSON ")) {
            $process.WaitForExit()
            throw "$Consumer produced no benchmark result:`n$line`n$($stderr.Result)"
        }
        $exited = $process.WaitForExit([Math]::Max(5000, $SettleMs + 5000))
        if (-not $exited) {
            Stop-Process -Id $process.Id -Force
            $process.WaitForExit()
        }
        $stderr.Wait()
        if ($exited -and $process.ExitCode -ne 0) {
            throw "$Consumer exited with code $($process.ExitCode):`n$($stderr.Result)"
        }
        return $line.Substring("WINUI_BENCH_JSON ".Length) | ConvertFrom-Json
    }

    $consumers = @("windows-rs", "windows-csharp", "cswinrt")
    foreach ($run in 1..$Runs) {
        $offset = ($run - 1) % $consumers.Count
        foreach ($index in 0..($consumers.Count - 1)) {
            $consumer = $consumers[($index + $offset) % $consumers.Count]
            $projectionArguments = @(
                "--iterations", $Iterations,
                "--create-iterations", $CreateIterations,
                "--tree-iterations", $TreeIterations,
                "--tree-size", $TreeSize,
                "--stress-iterations", $StressIterations,
                "--sustained-seconds", 1,
                "--sustained-percent", $SustainedPercent,
                "--sustained-churn", 0,
                "--settle-ms", 0,
                "--headless"
            )
            $result = Invoke-Consumer $consumer $projectionArguments

            $sustainedArguments = @(
                "--iterations", 1,
                "--create-iterations", 1,
                "--tree-iterations", 1,
                "--tree-size", 1,
                "--stress-iterations", 1,
                "--sustained-seconds", $SustainedSeconds,
                "--sustained-percent", $SustainedPercent,
                "--sustained-churn", $SustainedChurn,
                "--settle-ms", $SettleMs
            )
            if ($Headless) {
                $sustainedArguments += "--headless"
            }
            $sustained = Invoke-Consumer $consumer $sustainedArguments
            foreach ($name in @(
                "sustainedTicks",
                "sustainedUpdateMs",
                "sustainedUpdateBytes",
                "sustainedFrames",
                "sustainedFps",
                "sustainedChurn",
                "sustainedWorkingSet"
            )) {
                $result.$name = $sustained.$name
            }
            $results += $result
        }
    }

    function Get-Median([object[]]$Values) {
        $sorted = @($Values | Sort-Object)
        $middle = [int][Math]::Floor($sorted.Count / 2)
        if (($sorted.Count % 2) -eq 1) {
            return [double]$sorted[$middle]
        }
        return ([double]$sorted[$middle - 1] + [double]$sorted[$middle]) / 2
    }

    $medians = foreach ($consumer in @("windows-rs", "windows-csharp", "cswinrt")) {
        $rows = @($results | Where-Object consumer -eq $consumer)
        [pscustomobject]@{
            Consumer = $consumer
            MainMs = Get-Median @($rows.mainMs)
            HostStartMs = Get-Median @($rows.hostStartMs)
            StartupMs = Get-Median @($rows.startupMs)
            WindowMs = Get-Median @($rows.windowMs)
            WorkingSetMiB = (Get-Median @($rows.workingSet)) / 1MB
            CreateNs = Get-Median @($rows.createNs)
            CreateBytes = Get-Median @($rows.createBytes)
            UpdateNs = Get-Median @($rows.updateNs)
            UpdateBytes = Get-Median @($rows.updateBytes)
            CastNs = Get-Median @($rows.castNs)
            CastBytes = Get-Median @($rows.castBytes)
            TreeNs = Get-Median @($rows.treeNs)
            TreeBytes = Get-Median @($rows.treeBytes)
            BatchUpdateNs = Get-Median @($rows.batchUpdateNs)
            BatchUpdateBytes = Get-Median @($rows.batchUpdateBytes)
            ChurnNs = Get-Median @($rows.churnNs)
            ChurnBytes = Get-Median @($rows.churnBytes)
            TeardownNs = Get-Median @($rows.teardownNs)
            TeardownBytes = Get-Median @($rows.teardownBytes)
            EventNs = Get-Median @($rows.eventNs)
            EventBytes = Get-Median @($rows.eventBytes)
            BooleanNs = Get-Median @($rows.booleanNs)
            BooleanBytes = Get-Median @($rows.booleanBytes)
            StressBuildMs = (Get-Median @($rows.stressBuildNs)) / 1e6
            StressBuildMiB = (Get-Median @($rows.stressBuildBytes)) / 1MB
            StressWorkingSetMiB = (Get-Median @($rows.stressWorkingSet)) / 1MB
            Stress0Ms = Get-Median @($rows.stress0Ms)
            Stress0Bytes = Get-Median @($rows.stress0Bytes)
            Stress10Ms = Get-Median @($rows.stress10Ms)
            Stress10Bytes = Get-Median @($rows.stress10Bytes)
            Stress50Ms = Get-Median @($rows.stress50Ms)
            Stress50Bytes = Get-Median @($rows.stress50Bytes)
            Stress100Ms = Get-Median @($rows.stress100Ms)
            Stress100Bytes = Get-Median @($rows.stress100Bytes)
            SustainedTicks = Get-Median @($rows.sustainedTicks)
            SustainedUpdateMs = Get-Median @($rows.sustainedUpdateMs)
            SustainedUpdateBytes = Get-Median @($rows.sustainedUpdateBytes)
            SustainedFrames = Get-Median @($rows.sustainedFrames)
            SustainedFps = Get-Median @($rows.sustainedFps)
            SustainedChurn = Get-Median @($rows.sustainedChurn)
            SustainedWorkingSetMiB = (Get-Median @($rows.sustainedWorkingSet)) / 1MB
        }
    }

    Write-Output ""
    Write-Output "Median of $Runs fresh-process runs"
    $medians |
        Select-Object Consumer, MainMs, HostStartMs, StartupMs, WindowMs |
        Format-Table -AutoSize
    $medians |
        Select-Object Consumer, WorkingSetMiB, CreateNs, CreateBytes, `
            UpdateNs, UpdateBytes, CastNs, CastBytes |
        Format-Table -AutoSize
    $medians |
        Select-Object Consumer, TreeNs, TreeBytes |
        Format-Table -AutoSize
    $medians |
        Select-Object Consumer, BatchUpdateNs, BatchUpdateBytes, ChurnNs, ChurnBytes |
        Format-Table -AutoSize
    $medians |
        Select-Object Consumer, BooleanNs, BooleanBytes, EventNs, EventBytes |
        Format-Table -AutoSize
    $medians |
        Select-Object Consumer, TeardownNs, TeardownBytes |
        Format-Table -AutoSize
    $medians |
        Select-Object Consumer, StressBuildMs, StressBuildMiB, StressWorkingSetMiB |
        Format-Table -AutoSize
    $medians |
        Select-Object Consumer, Stress0Ms, Stress10Ms, Stress50Ms, Stress100Ms |
        Format-Table -AutoSize
    $medians |
        Select-Object Consumer, Stress0Bytes, Stress10Bytes, Stress50Bytes, Stress100Bytes |
        Format-Table -AutoSize
    $medians |
        Select-Object Consumer, SustainedTicks, SustainedUpdateMs, SustainedUpdateBytes, `
            SustainedFrames, SustainedFps, SustainedChurn, SustainedWorkingSetMiB |
        Format-Table -AutoSize
}
finally {
    Pop-Location
}
