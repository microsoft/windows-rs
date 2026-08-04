# windows-csharp Minesweeper

This is a C# port of `crates/samples/composition/minesweeper`. It hosts a
`Windows.UI.Composition` visual tree in a plain Win32 window and uses only the generated
`windows-csharp` projection. It has no CsWinRT, Windows App SDK, or NuGet dependency.
The project also disables the implicit Windows SDK projection runtime, so
`Microsoft.Windows.SDK.NET.dll` and `WinRT.Runtime.dll` are not copied beside the application.

From the repository root:

```powershell
dotnet run --project crates\samples\csharp\minesweeper\minesweeper.csproj `
    -c Release -p:Platform=x64
```

The MSBuild target runs `cargo build -p csharp_minesweeper` before C# compilation. The Cargo build
script reads the repository Windows metadata plus the installed
`C:\Windows\System32\WinMetadata\Windows.Foundation.winmd`, then regenerates the committed
`Windows.cs` projection. No separate Cargo command is needed.

Left-click reveals a tile. Right-click cycles flag, question, and empty. The first reveal generates
40 mines while excluding that tile. The 16x16 board scales with the client area, and hitting a mine
plays the spiral composition animation. Click again after a win or loss to reset.

Use the bounded smoke mode to create the window, dispatcher queue, desktop composition target,
board, brushes, shapes, and input path before exiting:

```powershell
dotnet run --project crates\samples\csharp\minesweeper\minesweeper.csproj `
    -c Release -p:Platform=x64 -- --smoke
```

`--smoke-visible` runs the same bounded check with the window shown for 500 ms.
