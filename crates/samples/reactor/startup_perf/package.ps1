[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [string] $Publisher,

    [string] $PublisherDisplayName = "Microsoft Corporation",

    [ValidatePattern('^\d{1,5}(\.\d{1,5}){3}$')]
    [string] $Version = "1.0.0.0",

    [string] $OutputPath,

    [string] $CertificatePath,

    [Security.SecureString] $CertificatePassword,

    [string] $TimestampUrl,

    [switch] $SkipBuild
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Invoke-Checked {
    param(
        [Parameter(Mandatory)]
        [string] $FilePath,

        [Parameter(Mandatory)]
        [string[]] $Arguments
    )

    & $FilePath @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$FilePath failed with exit code $LASTEXITCODE"
    }
}

function Find-WindowsSdkTool {
    param(
        [Parameter(Mandatory)]
        [string] $Name
    )

    $command = Get-Command $Name -ErrorAction SilentlyContinue
    if ($command) {
        return $command.Source
    }

    $kitsRoot = (Get-ItemProperty `
        "HKLM:\SOFTWARE\Microsoft\Windows Kits\Installed Roots" `
        -ErrorAction SilentlyContinue).KitsRoot10
    if (!$kitsRoot) {
        throw "$Name was not found and the Windows SDK installation could not be located"
    }

    $candidate = Get-ChildItem (Join-Path $kitsRoot "bin") -Directory |
        Sort-Object Name -Descending |
        ForEach-Object { Join-Path $_.FullName "x64\$Name" } |
        Where-Object { Test-Path -LiteralPath $_ } |
        Select-Object -First 1
    if (!$candidate) {
        throw "$Name was not found in the Windows SDK"
    }
    return $candidate
}

function New-PackageLogo {
    param(
        [Parameter(Mandatory)]
        [string] $Path,

        [Parameter(Mandatory)]
        [int] $Size
    )

    $bitmap = [System.Drawing.Bitmap]::new($Size, $Size)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    try {
        $graphics.Clear([System.Drawing.Color]::FromArgb(0, 120, 212))
        $bitmap.Save($Path, [System.Drawing.Imaging.ImageFormat]::Png)
    }
    finally {
        $graphics.Dispose()
        $bitmap.Dispose()
    }
}

foreach ($part in $Version.Split(".")) {
    if ([uint32] $part -gt [uint16]::MaxValue) {
        throw "Each package version component must be between 0 and 65535"
    }
}

$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot "..\..\..\.."))
$target = Join-Path $repoRoot "target\release"
$work = Join-Path $repoRoot "target\reactor-startup-msix"
$layout = Join-Path $work "layout"
$template = Join-Path $PSScriptRoot "package\AppxManifest.xml.template"

if (!$OutputPath) {
    $OutputPath = Join-Path $work "BlankWindowsReactor_x64.msix"
}
$OutputPath = [IO.Path]::GetFullPath($OutputPath)

if (!$SkipBuild) {
    Push-Location $repoRoot
    try {
        Invoke-Checked "cargo" @(
            "build",
            "-p",
            "reactor_startup_perf",
            "--release",
            "--quiet"
        )
    }
    finally {
        Pop-Location
    }
}

if (Test-Path -LiteralPath $layout) {
    Remove-Item -LiteralPath $layout -Recurse -Force
}
$null = New-Item -ItemType Directory -Path $layout
$null = New-Item -ItemType Directory -Path (Split-Path $OutputPath -Parent) -Force

$payload = @(
    "BlankWindowsReactor.exe",
    "microsoft.windowsappruntime.bootstrap.dll",
    "resources.pri"
)
foreach ($name in $payload) {
    $source = Join-Path $target $name
    if (!(Test-Path -LiteralPath $source)) {
        throw "Required package file not found: $source"
    }
    Copy-Item -LiteralPath $source -Destination (Join-Path $layout $name)
}

$manifest = Get-Content -LiteralPath $template -Raw
$manifest = $manifest.Replace(
    "{{PUBLISHER}}",
    [Security.SecurityElement]::Escape($Publisher)
)
$manifest = $manifest.Replace(
    "{{PUBLISHER_DISPLAY_NAME}}",
    [Security.SecurityElement]::Escape($PublisherDisplayName)
)
$manifest = $manifest.Replace("{{VERSION}}", $Version)
Set-Content -LiteralPath (Join-Path $layout "AppxManifest.xml") `
    -Value $manifest `
    -Encoding utf8NoBOM

Add-Type -AssemblyName System.Drawing
New-PackageLogo (Join-Path $layout "StoreLogo.png") 50
New-PackageLogo (Join-Path $layout "Square44x44Logo.png") 44
New-PackageLogo (Join-Path $layout "Square150x150Logo.png") 150

$makeAppx = Find-WindowsSdkTool "makeappx.exe"
Invoke-Checked $makeAppx @(
    "pack",
    "/d",
    $layout,
    "/p",
    $OutputPath,
    "/o"
)

if ($CertificatePath) {
    $CertificatePath = Convert-Path -LiteralPath $CertificatePath
    $stagedCertificatePath = $null
    if ($CertificatePath.StartsWith("\\")) {
        $stagedCertificatePath = Join-Path $work "signing-$([guid]::NewGuid()).pfx"
        Copy-Item -LiteralPath $CertificatePath -Destination $stagedCertificatePath
        $CertificatePath = $stagedCertificatePath
    }
    $signTool = Find-WindowsSdkTool "signtool.exe"
    $arguments = @("sign", "/fd", "SHA256", "/f", $CertificatePath)
    $passwordPointer = [IntPtr]::Zero
    try {
        if ($CertificatePassword) {
            $passwordPointer = [Runtime.InteropServices.Marshal]::SecureStringToBSTR(
                $CertificatePassword
            )
            $passwordText = [Runtime.InteropServices.Marshal]::PtrToStringBSTR($passwordPointer)
            if ($passwordText.Length -ne 0) {
                $arguments += @("/p", $passwordText)
            }
        }
        if ($TimestampUrl) {
            $arguments += @("/tr", $TimestampUrl, "/td", "SHA256")
        }
        $arguments += $OutputPath
        Invoke-Checked $signTool $arguments

        $signature = Get-AuthenticodeSignature -LiteralPath $OutputPath
        if (!$signature.SignerCertificate) {
            throw "The package was not signed"
        }
        if ($signature.SignerCertificate.Subject -ne $Publisher) {
            throw "The signing certificate subject does not match the manifest publisher"
        }
    }
    finally {
        if ($passwordPointer -ne [IntPtr]::Zero) {
            [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($passwordPointer)
        }
        if ($stagedCertificatePath) {
            Remove-Item -LiteralPath $stagedCertificatePath -Force
        }
    }
}
else {
    Write-Warning "Created an unsigned package; pass -CertificatePath to sign it"
}

$package = Get-Item -LiteralPath $OutputPath
$hash = Get-FileHash -LiteralPath $OutputPath -Algorithm SHA256
Write-Host "Package: $($package.FullName)"
Write-Host "Size:    $($package.Length) bytes"
Write-Host "SHA256:  $($hash.Hash)"
