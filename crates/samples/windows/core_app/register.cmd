cargo build
copy appx\* ..\..\..\..\target\debug
cd ..\..\..\..\target\debug
powershell -command "Get-AppxPackage *0f8c5510-182f-4208-a48e-4215050a0453* | Remove-AppxPackage"
powershell -command "Add-AppxPackage -Register AppxManifest.xml"
cd ..\..\crates\samples\windows\core_app
