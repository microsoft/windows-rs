copy "..\target\%2\%1\Component.dll" ..\..\.windows\%3
copy "..\target\%2\%1\Component.lib" ..\..\.windows\%3
copy "..\target\%2\%1\Component.winmd" ..\..\.windows\winmd

if "%3" == "x64" (
	pushd .metadata
	dotnet clean && dotnet build /p:Platform=%2 /p:Configuration=%1 /p:OutputWinmd=..\..\target\%2\%1\Component.Win32.winmd
	popd
	copy "..\target\%2\%1\Component.Win32.winmd" ..\..\.windows\winmd
)
