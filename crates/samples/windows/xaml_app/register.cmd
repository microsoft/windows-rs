@echo off
setlocal

cargo build || exit /b 1

set TARGET=..\..\..\..\target\debug

if not exist "%TARGET%\sample_xaml_app.pdb" (
    echo ERROR: %TARGET%\sample_xaml_app.pdb is missing.
    echo Symbols are required to make sense of crash dumps. Make sure
    echo [profile.dev] in the workspace Cargo.toml does not set debug = 0
    echo or strip = "debuginfo".
    exit /b 1
)

copy /Y appx\* "%TARGET%" >nul || exit /b 1

REM Enable Windows Error Reporting LocalDumps for sample_xaml_app.exe so that
REM a .dmp file is captured on crash. Dumps land in %LOCALAPPDATA%\CrashDumps\
REM and can then be analyzed with dump.cmd. Requires Administrator (HKLM).
net session >nul 2>&1
if errorlevel 1 (
    echo WARNING: not running as Administrator; skipping WER LocalDumps setup.
    echo Re-run register.cmd from an elevated x64 Native Tools prompt to enable
    echo automatic crash dump capture for sample_xaml_app.exe.
) else (
    reg add "HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\LocalDumps\sample_xaml_app.exe" /v DumpType   /t REG_DWORD    /d 2 /f >nul
    reg add "HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\LocalDumps\sample_xaml_app.exe" /v DumpCount  /t REG_DWORD    /d 5 /f >nul
    reg add "HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\LocalDumps\sample_xaml_app.exe" /v DumpFolder /t REG_EXPAND_SZ /d "%%LOCALAPPDATA%%\CrashDumps" /f >nul
)

pushd "%TARGET%"
powershell -command "Get-AppxPackage *942b16b2-c2af-4092-ba34-e4f0c2df308b* | Remove-AppxPackage"
powershell -command "Add-AppxPackage -Register AppxManifest.xml"
popd
