cargo build
copy appx\* ..\..\target\debug
cd ..\..\target\debug
powershell -command "Add-AppxPackage -Register AppxManifest.xml"
cd ..\..\examples\core_app
