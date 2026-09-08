@echo off
setlocal

set "sample=%~dp0"
set "layout=%sample%..\..\..\..\target\debug"

cargo build --manifest-path "%sample%Cargo.toml" || exit /b 1
copy /y "%sample%appx\*" "%layout%\" >nul || exit /b 1

powershell -NoProfile -Command "$package = Get-AppxPackage -Name '0f8c5510-182f-4208-a48e-4215050a0453'; if ($package) { $package | Remove-AppxPackage }; Add-AppxPackage -Register '%layout%\AppxManifest.xml'" || exit /b 1

echo Registered successfully. Launch "Rust CoreApp" from Start.
