using System.Runtime.InteropServices;
using Robotics;
using WinRT;

var robot = new Robot();
robot.Speak("Hello from cs land");

// CreateRobotFromHandle is a Win32 export rather than a WinRT method.
[DllImport("robotics.dll")]
static extern int CreateRobotFromHandle(nint handle, out IntPtr robot);

Marshal.ThrowExceptionForHR(CreateRobotFromHandle(0x1c8, out IntPtr handyAbi));
var handyRobot = MarshalInspectable<Robot>.FromAbi(handyAbi);
handyRobot.Speak("Hello handy");

// Query the non-WinRT interop interface through the raw ABI pointer.
Guid iid = new Guid("ae60832b-0bc8-57b0-8a69-f82ebc1560ed");
Marshal.ThrowExceptionForHR(Marshal.QueryInterface(handyAbi, iid, out IntPtr interopPtr));
nint vtable = Marshal.ReadIntPtr(interopPtr);
nint handleFnPtr = Marshal.ReadIntPtr(vtable + 3 * IntPtr.Size);
nint handle = Marshal.GetDelegateForFunctionPointer<HandleFunc>(handleFnPtr)(interopPtr);
Marshal.Release(interopPtr);
Console.WriteLine($"interop handle: 0x{handle:x}");

[UnmanagedFunctionPointer(CallingConvention.StdCall)]
delegate nint HandleFunc(IntPtr self);
