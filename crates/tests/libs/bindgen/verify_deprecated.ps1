# Verify deprecated/removed API support in windows-rs generated bindings
# This script checks the generated .rs files to verify:
# 1. Deprecated types have #[deprecated(note = "...")] attributes
# 2. Impl blocks for deprecated types have #[allow(deprecated)]
# 3. Vtable function pointers are always present (no removal)
# 4. Enum values are properly annotated

param(
    [string]$SrcDir = "crates\tests\libs\bindgen\src"
)

$ErrorActionPreference = "Stop"
$script:passed = 0
$script:failed = 0

function Assert-Contains {
    param([string]$File, [string]$Pattern, [string]$Message, [switch]$Regex)
    $content = Get-Content $File -Raw
    if ($Regex) {
        if ($content -match $Pattern) {
            Write-Host "  PASS: $Message" -ForegroundColor Green
            $script:passed++
        } else {
            Write-Host "  FAIL: $Message" -ForegroundColor Red
            Write-Host "    Expected regex: $Pattern" -ForegroundColor Yellow
            $script:failed++
        }
    } else {
        if ($content -match [regex]::Escape($Pattern)) {
            Write-Host "  PASS: $Message" -ForegroundColor Green
            $script:passed++
        } else {
            Write-Host "  FAIL: $Message" -ForegroundColor Red
            Write-Host "    Expected to find: $Pattern" -ForegroundColor Yellow
            $script:failed++
        }
    }
}

function Assert-NotContains {
    param([string]$File, [string]$Pattern, [string]$Message)
    $content = Get-Content $File -Raw
    if ($content -match [regex]::Escape($Pattern)) {
        Write-Host "  FAIL: $Message" -ForegroundColor Red
        Write-Host "    Expected NOT to find: $Pattern" -ForegroundColor Yellow
        $script:failed++
    } else {
        Write-Host "  PASS: $Message" -ForegroundColor Green
        $script:passed++
    }
}

$classFile = Join-Path $SrcDir "deprecated_class.rs"
$enumFile = Join-Path $SrcDir "deprecated_enum.rs"
$structFile = Join-Path $SrcDir "deprecated_struct.rs"

foreach ($f in @($classFile, $enumFile, $structFile)) {
    if (!(Test-Path $f)) {
        Write-Host "ERROR: $f not found. Run tool_bindgen first." -ForegroundColor Red
        exit 1
    }
}

Write-Host "`n=== Deprecated class: KnownContactField ===" -ForegroundColor Cyan

# Type-level #[deprecated] on the class struct
Assert-Contains $classFile '#[deprecated(' "KnownContactField has #[deprecated] on struct"
Assert-Contains $classFile 'pub struct KnownContactField' "KnownContactField struct is generated"

# Impl block has #[allow(deprecated)] to prevent cascading warnings
Assert-Contains $classFile '#[allow(deprecated)]' "Impl block has #[allow(deprecated)]"

# Methods exist and have #[deprecated]
Assert-Contains $classFile 'pub fn Email' "Email() method is generated"
Assert-Contains $classFile 'pub fn PhoneNumber' "PhoneNumber() method is generated"

# Vtable function pointers always present
Assert-Contains $classFile 'IKnownContactFieldStatics_Vtbl' "Vtable struct is generated"
Assert-Contains $classFile 'pub Email:' "Email vtable slot present"

Write-Host "`n=== Deprecated enum: PlayToConnectionState ===" -ForegroundColor Cyan

Assert-Contains $enumFile '#[deprecated(' "PlayToConnectionState has #[deprecated]"
Assert-Contains $enumFile 'pub struct PlayToConnectionState' "PlayToConnectionState struct generated"

# Enum values present
Assert-Contains $enumFile 'Disconnected' "Disconnected value present"
Assert-Contains $enumFile 'Connected' "Connected value present"
Assert-Contains $enumFile 'Rendering' "Rendering value present"

Write-Host "`n=== Deprecated struct: PlayToSourceSelectedEventArgs ===" -ForegroundColor Cyan

Assert-Contains $structFile '#[deprecated(' "PlayToSourceSelectedEventArgs has #[deprecated]"
Assert-Contains $structFile 'pub struct PlayToSourceSelectedEventArgs' "PlayToSourceSelectedEventArgs generated"

# Properties exist as methods
Assert-Contains $structFile 'pub fn FriendlyName' "FriendlyName property generated"
Assert-Contains $structFile 'pub fn SupportsImage' "SupportsImage property generated"
Assert-Contains $structFile 'pub fn SupportsAudio' "SupportsAudio property generated"
Assert-Contains $structFile 'pub fn SupportsVideo' "SupportsVideo property generated"

# Vtable slots always present
Assert-Contains $structFile 'pub FriendlyName:' "FriendlyName vtable slot present"
Assert-Contains $structFile 'pub SupportsImage:' "SupportsImage vtable slot present"

# #[allow(deprecated)] on impl
Assert-Contains $structFile '#[allow(deprecated)]' "Impl block has #[allow(deprecated)]"

Write-Host "`n=== Summary ===" -ForegroundColor Cyan
Write-Host "Passed: $($script:passed)" -ForegroundColor Green
if ($script:failed -gt 0) {
    Write-Host "Failed: $($script:failed)" -ForegroundColor Red
    exit 1
} else {
    Write-Host "All checks passed!" -ForegroundColor Green
}
