pushd .metadata
if "%3" == "x64" (
	setlocal EnableDelayedExpansion
	set props=/p:Platform=%2 /p:Configuration=%1
	dotnet clean && dotnet build /p:OutputWinmd=..\target\%2\%1\Component.Win32.winmd !props!
	dotnet clean && dotnet build /p:LibraryForm=static /p:OutputWinmd=..\target\%2\%1\Component.Win32.Static.winmd !props!
	endlocal
)
popd

copy target\%2\%1\Component.dll ..\..\.windows\%3
copy target\%2\%1\Component.lib ..\..\.windows\%3
copy target\%2\%1\Component.static.lib ..\..\.windows\%3
copy target\%2\%1\Component.winmd ..\..\.windows\winmd
copy target\%2\%1\Component.Win32.winmd ..\..\.windows\winmd
copy target\%2\%1\Component.Win32.Static.winmd ..\..\.windows\winmd