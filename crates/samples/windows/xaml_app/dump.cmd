@echo off
setlocal

set EXENAME=sample_xaml_app.exe
set DUMPDIR=%LOCALAPPDATA%\CrashDumps
set SYMDIR=..\..\..\..\target\debug

REM Pick the newest dump for our app.
set DUMP=
for /f "delims=" %%F in ('dir /b /o-d "%DUMPDIR%\%EXENAME%.*.dmp" 2^>nul') do (
    if not defined DUMP set "DUMP=%DUMPDIR%\%%F"
)

if not defined DUMP (
    echo No dump found in %DUMPDIR% matching %EXENAME%.*.dmp
    echo.
    echo Make sure register.cmd was run from an elevated prompt so that
    echo Windows Error Reporting LocalDumps is enabled for %EXENAME%,
    echo then reproduce the crash and re-run dump.cmd.
    exit /b 1
)

echo Analyzing %DUMP%

REM Locate cdb.exe from the Windows 10/11 SDK.
set CDB=
for %%D in (
    "%ProgramFiles(x86)%\Windows Kits\10\Debuggers\x64\cdb.exe"
    "%ProgramFiles%\Windows Kits\10\Debuggers\x64\cdb.exe"
) do if not defined CDB if exist %%D set "CDB=%%~D"

if not defined CDB (
    echo cdb.exe not found. Install the Windows 10/11 SDK Debugging Tools.
    exit /b 1
)

REM Symbol path: our PDB next to the exe + Microsoft public symbol server,
REM cached locally. This resolves both sample_xaml_app frames and combase /
REM Windows.UI.Xaml frames in the dump.
set SYMCACHE=%LOCALAPPDATA%\Symbols
set SYMPATH=cache*%SYMCACHE%;%SYMDIR%;srv*%SYMCACHE%*https://msdl.microsoft.com/download/symbols

REM .symfix tops up the path with the default MS symbol server, .reload /f
REM forces full symbol load so frames in combase / Windows.UI.Xaml resolve.
REM ~* kpn 40 dumps every thread (the originating raise frame for shutdown
REM AVs is rarely on the thread that .ecxr lands on); !analyze -v gives the
REM bucket; lmv lists module versions so PDB mismatches are obvious.
"%CDB%" -z "%DUMP%" -y "%SYMPATH%" -c ".symfix+ %SYMCACHE%; .reload /f; .ecxr; !analyze -v; kpn 40; ~* kpn 40; lmv; q"
