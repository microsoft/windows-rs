pushd .metadata
if "%3" == "x64" dotnet clean && dotnet build /p:OutputWinmd=..\target\%2\%1\Component.Win32.winmd /p:Platform=%2 /p:Configuration=%1
popd

copy target\%2\%1\Component.dll ..\..\.windows\%3
copy target\%2\%1\Component.lib ..\..\.windows\%3
copy target\%2\%1\Component.winmd ..\..\.windows\winmd
copy target\%2\%1\Component.Win32.winmd ..\..\.windows\winmd
del Component.lib