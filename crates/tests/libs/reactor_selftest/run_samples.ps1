[CmdletBinding()]
param(
    [ValidateRange(1, 60)]
    [int]$RunSeconds = 3,

    [ValidateSet("reactor", "composition", "webview")]
    [string[]]$Family = @("reactor", "composition", "webview"),

    [string[]]$Package = @(),

    [ValidateRange(1, 30)]
    [int]$CloseTimeoutSeconds = 3,

    [switch]$Release,

    [switch]$KeepLogs
)

$ErrorActionPreference = "Stop"

function Read-Log([string]$Path) {
    if (!(Test-Path $Path)) {
        return ""
    }
    $content = Get-Content $Path -Raw
    if ($null -eq $content) {
        return ""
    }
    $content.Trim()
}

function Write-Indented([string]$Label, [string]$Text) {
    if ([string]::IsNullOrWhiteSpace($Text)) {
        return
    }
    Write-Host "  ${Label}:"
    $Text -split "\r?\n" | ForEach-Object {
        Write-Host "    $_"
    }
}

function Stop-SampleProcess(
    [System.Diagnostics.Process]$Process,
    [int]$TimeoutSeconds
) {
    $Process.Refresh()
    if ($Process.HasExited) {
        return
    }

    $closed = $false
    try {
        $closed = $Process.CloseMainWindow()
    } catch {
        $closed = $false
    }

    if ($closed -and $Process.WaitForExit($TimeoutSeconds * 1000)) {
        return
    }

    Stop-Process -Id $Process.Id -Force
    $Process.WaitForExit($TimeoutSeconds * 1000) | Out-Null
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..\..")).Path
$cargo = (Get-Command cargo).Source
$profile = if ($Release) { "release" } else { "debug" }
$logRoot = Join-Path ([System.IO.Path]::GetTempPath()) (
    "windows-rs-sample-smoke-" + [System.Guid]::NewGuid().ToString("N")
)
New-Item -ItemType Directory -Path $logRoot | Out-Null

$failures = New-Object System.Collections.Generic.List[string]

Push-Location $repoRoot
try {
    $metadataJson = cargo metadata --no-deps --format-version 1
    if ($LASTEXITCODE -ne 0) {
        throw "cargo metadata failed with exit code $LASTEXITCODE"
    }
    $metadata = $metadataJson | ConvertFrom-Json

    $samples = foreach ($cargoPackage in $metadata.packages) {
        $manifest = $cargoPackage.manifest_path.Replace("\", "/")
        $match = [regex]::Match(
            $manifest,
            "/crates/samples/(reactor|composition|webview)/"
        )
        if (!$match.Success -or $Family -notcontains $match.Groups[1].Value) {
            continue
        }
        if ($Package.Count -ne 0 -and $Package -notcontains $cargoPackage.name) {
            continue
        }

        $bins = @($cargoPackage.targets | Where-Object {
            if ($_.kind -notcontains "bin") {
                return $false
            }
            $property = $_.PSObject.Properties["required-features"]
            $null -eq $property -or @($property.Value).Count -eq 0
        })
        if ($bins.Count -ne 1) {
            throw "$($cargoPackage.name) has $($bins.Count) default binary targets"
        }

        [pscustomobject]@{
            Family = $match.Groups[1].Value
            Package = $cargoPackage.name
            Binary = $bins[0].name
        }
    }
    $samples = @($samples | Sort-Object Family, Package)

    if ($samples.Count -eq 0) {
        throw "no matching sample packages were found"
    }
    if ($Package.Count -ne 0) {
        $found = @($samples | ForEach-Object { $_.Package })
        $missing = @($Package | Where-Object { $found -notcontains $_ })
        if ($missing.Count -ne 0) {
            throw "unknown or excluded sample package(s): $($missing -join ', ')"
        }
    }

    Write-Host "Running $($samples.Count) sample smoke tests for $RunSeconds second(s) each"
    Write-Host "Logs: $logRoot"

    for ($index = 0; $index -lt $samples.Count; $index++) {
        $sample = $samples[$index]
        $label = "[$($index + 1)/$($samples.Count)] $($sample.Package)"
        Write-Host $label

        $prefix = "$($sample.Family)-$($sample.Package)-$($sample.Binary)"
        $buildOut = Join-Path $logRoot "$prefix-build.stdout.txt"
        $buildErr = Join-Path $logRoot "$prefix-build.stderr.txt"
        $stdout = Join-Path $logRoot "$prefix.stdout.txt"
        $stderr = Join-Path $logRoot "$prefix.stderr.txt"
        $buildArguments = @(
            "build",
            "--quiet",
            "-p",
            $sample.Package,
            "--bin",
            $sample.Binary
        )
        if ($Release) {
            $buildArguments += "--release"
        }

        $build = Start-Process `
            -FilePath $cargo `
            -ArgumentList $buildArguments `
            -WorkingDirectory $repoRoot `
            -NoNewWindow `
            -RedirectStandardOutput $buildOut `
            -RedirectStandardError $buildErr `
            -Wait `
            -PassThru
        if ($build.ExitCode -ne 0) {
            $message = "build exited with code $($build.ExitCode)"
            $failures.Add("$($sample.Package): $message")
            Write-Host "  FAILED: $message"
            Write-Indented "build stdout" (Read-Log $buildOut)
            Write-Indented "build stderr" (Read-Log $buildErr)
            continue
        }

        $executable = Join-Path $metadata.target_directory (
            "$profile\$($sample.Binary).exe"
        )
        if (!(Test-Path $executable)) {
            $message = "built executable was not found at $executable"
            $failures.Add("$($sample.Package): $message")
            Write-Host "  FAILED: $message"
            continue
        }

        $process = $null
        $earlyExit = $false
        $exitCode = $null
        try {
            $process = Start-Process `
                -FilePath $executable `
                -WorkingDirectory $repoRoot `
                -RedirectStandardOutput $stdout `
                -RedirectStandardError $stderr `
                -PassThru
            Start-Sleep -Seconds $RunSeconds
            $process.Refresh()
            if ($process.HasExited) {
                $earlyExit = $true
                $exitCode = $process.ExitCode
            }
        } catch {
            $failures.Add("$($sample.Package): launch failed: $($_.Exception.Message)")
            Write-Host "  FAILED: launch failed: $($_.Exception.Message)"
        } finally {
            if ($null -ne $process) {
                try {
                    Stop-SampleProcess $process $CloseTimeoutSeconds
                } catch {
                    $failures.Add(
                        "$($sample.Package): shutdown failed: $($_.Exception.Message)"
                    )
                    Write-Host "  FAILED: shutdown failed: $($_.Exception.Message)"
                }
            }
        }

        $runtimeOut = Read-Log $stdout
        $runtimeErr = Read-Log $stderr
        Write-Indented "stdout" $runtimeOut
        Write-Indented "stderr" $runtimeErr

        if ($earlyExit) {
            $message = "exited before $RunSeconds second(s) with code $exitCode"
            $failures.Add("$($sample.Package): $message")
            Write-Host "  FAILED: $message"
        } elseif (![string]::IsNullOrWhiteSpace($runtimeErr)) {
            $message = "wrote to stderr"
            $failures.Add("$($sample.Package): $message")
            Write-Host "  FAILED: $message"
        } elseif ($null -ne $process) {
            Write-Host "  passed"
        }
    }
} finally {
    Pop-Location
    if (!$KeepLogs -and $failures.Count -eq 0) {
        Remove-Item -Path $logRoot -Recurse -Force
    }
}

if ($failures.Count -ne 0) {
    Write-Host ""
    Write-Host "$($failures.Count) sample smoke test(s) failed:"
    $failures | ForEach-Object {
        Write-Host "  $_"
    }
    Write-Host "Logs: $logRoot"
    exit 1
}

Write-Host ""
Write-Host "All $($samples.Count) sample smoke tests passed."
