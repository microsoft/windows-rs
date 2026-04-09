using System.Runtime.InteropServices;
using Robotics;
using WinRT;

// CsWinRT projects Robot as a proper .NET class with WinRT activation,
// HSTRING marshaling, and COM QI all handled automatically.
var robot = new Robot();
robot.Speak("Hello from cs land");

// CreateRobotFromHandle is a plain Win32 DLL export, not a WinRT method,
// so we use P/Invoke and receive the result as a raw ABI pointer.
[DllImport("robotics.dll")]
static extern int CreateRobotFromHandle(nint handle, out IntPtr robot);

// Create a robot from a raw handle value and wrap it as the projected Robot type.
// MarshalInspectable<T>.FromAbi is the CsWinRT API for wrapping raw WinRT ABI pointers.
Marshal.ThrowExceptionForHR(CreateRobotFromHandle(0x1c8, out IntPtr handyAbi));
var handyRobot = MarshalInspectable<Robot>.FromAbi(handyAbi);
handyRobot.Speak("Hello handy");

// CsWinRT projected classes support COM QI, so casting to a ComImport interface
// performs a QueryInterface on the underlying COM object.
var interop = (IRobotInterop)handyRobot;
var handle = interop.Handle();
Console.WriteLine($"interop handle: 0x{handle:x}");

// IRobotInterop is a COM interface (not WinRT), so we declare it with ComImport
// and obtain it via a COM QI cast from the projected Robot object.
// Type declarations must follow all top-level statements in C#.
[ComImport]
[Guid("ae60832b-0bc8-57b0-8a69-f82ebc1560ed")]
[InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
interface IRobotInterop
{
    nint Handle();
}
