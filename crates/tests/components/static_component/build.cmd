md ..\..\..\..\.windows\%3
md ..\..\..\..\.windows\winmd

copy "..\target\%2\%1\static_component.lib" ..\..\..\..\.windows\%3

pushd .metadata
dotnet clean && dotnet build /p:Platform=%2 /p:Configuration=%1 /p:OutputWinmd=..\..\target\%2\%1\StaticComponent.Win32.winmd
popd
copy "..\target\%2\%1\StaticComponent.Win32.winmd" ..\..\..\..\.windows\winmd
