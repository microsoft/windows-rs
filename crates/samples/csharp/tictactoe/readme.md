# windows-csharp WinUI Tic-Tac-Toe

This sample ports `crates/samples/reactor/apps/examples/tictactoe.rs` to direct WinUI 3 C#. It uses
the generated `windows-csharp` projection and the same pinned Windows App SDK metadata, runtime,
bootstrap DLL, and `resources.pri` as `windows-reactor`. It does not use CsWinRT,
Microsoft.Windows.SDK.NET projections, Windows App SDK NuGet packages, or `windows-reactor` at
runtime.

From the repository root:

```powershell
dotnet run --project crates\samples\csharp\tictactoe\tictactoe.csproj `
    -c Release -p:Platform=x64
```

MSBuild invokes the thin Cargo package before C# compilation. Its build script regenerates the
committed `Windows.cs` from the reactor metadata and uses `windows-reactor-setup` to stage the
matching framework-dependent bootstrap DLL and resources. MSBuild then copies those files next to
the C# executable.

The generated `Application.Start` callback constructs the base WinUI `Application` and queues
window creation on the WinUI dispatcher. This is enough for the programmatic control tree and
avoids a native host or managed `Application` subclass. Event revokers are released before their
controls, all projected owners are disposed on the UI thread, bootstrap shutdown runs after WinUI
teardown, and COM is uninitialized last.

Use the bounded smoke mode to bootstrap WinUI, create and activate the complete window, invoke the
generated click delegates to exercise a win, reset, exercise a draw, reset again, close the window,
and exit:

```powershell
dotnet run --project crates\samples\csharp\tictactoe\tictactoe.csproj `
    -c Release -p:Platform=x64 -- --smoke
```

The game logic has a package-free test project:

```powershell
dotnet run --project crates\samples\csharp\tictactoe\tests\tictactoe.tests.csproj `
    -c Release
```

The game behavior matches the reactor sample. The direct version uses the standard WinUI title bar
and default button theme instead of reactor's in-content title bar and explicit theme references.
