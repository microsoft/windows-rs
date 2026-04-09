using System.Runtime.InteropServices;
using Robotics;

// CsWinRT projects Robot as a proper .NET class with WinRT activation,
// HSTRING marshaling, and COM QI all handled automatically.
var robot = new Robot();
robot.Speak("Hello from cs land");

// CreateRobotFromHandle is a plain Win32 DLL export, not a WinRT method,
// so we use P/Invoke and receive the result as a raw ABI pointer.
[DllImport("robotics.dll")]
static extern int CreateRobotFromHandle(nint handle, out nint robot);

// IRobotInterop is a COM interface (not WinRT), so we declare it with ComImport
// and obtain it via a COM QI cast from the projected Robot object.
[ComImport]
[Guid("ae60832b-0bc8-57b0-8a69-f82ebc1560ed")]
[InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
interface IRobotInterop
{
    nint Handle();
}

// Create a robot from a raw handle value and wrap it as the projected Robot type.
Marshal.ThrowExceptionForHR(CreateRobotFromHandle(0x1c8, out nint handyAbi));
var handyRobot = Robot.FromAbi(handyAbi);
handyRobot.Speak("Hello handy");

// CsWinRT projected classes support COM QI, so casting to a ComImport interface
// performs a QueryInterface on the underlying COM object.
var interop = (IRobotInterop)handyRobot;
var handle = interop.Handle();
Console.WriteLine($"interop handle: 0x{handle:x}");
