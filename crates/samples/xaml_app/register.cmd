cargo build
copy appx\* ..\..\..\target\debug
cd ..\..\..\target\debug
powershell -command "Get-AppxPackage *942b16b2* | Remove-AppxPackage"
powershell -command "Add-AppxPackage -Register AppxManifest.xml"
cd ..\..\..\crates\samples\xaml_app
