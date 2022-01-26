cargo build
copy appx\* ..\..\..\target\debug
cd ..\..\..\target\debug
powershell -command "Get-AppxPackage *942b16b2-c2af-4092-ba34-e4f0c2df308b* | Remove-AppxPackage"
powershell -command "Add-AppxPackage -Register AppxManifest.xml"
cd ..\..\crates\samples\xaml_app
