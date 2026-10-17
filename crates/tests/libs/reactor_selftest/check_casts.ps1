param(
    [Parameter(Mandatory)]
    [string[]]$Path
)

$ErrorActionPreference = "Stop"
$location = ':\d+:\d+ returned the same interface pointer;'
$allowed = @(
    # Agility checks query a capability, not an inherited interface.
    'windows-core: cast::<windows_core::imp::bindings::IAgileObject> at ',
    # COM identity requires querying the canonical IUnknown.
    "windows-core: cast::<windows_core::unknown::IUnknown> at .*crates/libs/(core/src/unknown|reactor/src/native/winui/mod)\.rs$location",
    # Activation returns IInspectable, not the requested class interface.
    "windows-core: cast::<.+> at .*crates/libs/core/src/imp/generic_factory\.rs$location",
    # Virtual item indices and data/selection strings must be unboxed from IInspectable.
    "windows-core: cast::<windows_reference::reference::IReference<i32>> at .*crates/libs/reactor/src/native/winui/mod\.rs$location",
    "windows-core: cast::<windows_reference::reference::IReference<windows_strings::hstring::HSTRING>> at .*crates/libs/reactor/src/(native/(?:winui/\.\./)?generated|(?:native/winui/\.\./\.\./)?test_support/native/winui)\.rs$location",
    # Generated item collections convert between UIElement and the collection's item type.
    "windows-core: cast::<windows_reactor::native::bindings::(UIElement|ICommandBarElement|MenuBarItem|SelectorBarItem)> at .*crates/libs/reactor/src/native/(?:winui/\.\./)?generated\.rs$location"
)

$diagnostics = @(Get-Content -LiteralPath $Path | Where-Object { $_.Contains("windows-core: cast::") })
$unexpected = @(
    $diagnostics |
        Where-Object { $_.Replace('\', '/') -notmatch ($allowed -join '|') } |
        Sort-Object -Unique
)
if ($unexpected.Count -ne 0) {
    $unexpected | Write-Output
    throw "$($unexpected.Count) unexpected cast diagnostic(s)."
}

Write-Output "Cast diagnostics: $($diagnostics.Count) expected warning(s), no unexpected casts."
