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

// CsWinRT's ComWrappers projects all IInspectable pointers as WinRT types, so casting
// a projected class to a [ComImport] interface doesn't compile.  Instead, QueryInterface
// the raw ABI pointer directly and invoke Handle() through the COM vtable.
Guid iid = new Guid("ae60832b-0bc8-57b0-8a69-f82ebc1560ed");
Marshal.ThrowExceptionForHR(Marshal.QueryInterface(handyAbi, iid, out IntPtr interopPtr));
nint vtable = Marshal.ReadIntPtr(interopPtr);
nint handleFnPtr = Marshal.ReadIntPtr(vtable + 3 * IntPtr.Size);
nint handle = Marshal.GetDelegateForFunctionPointer<HandleFunc>(handleFnPtr)(interopPtr);
Marshal.Release(interopPtr);
Console.WriteLine($"interop handle: 0x{handle:x}");

// HandleFunc matches IRobotInterop::Handle() — vtable slot 3 after
// QueryInterface/AddRef/Release from IUnknown.
// Type declarations must follow all top-level statements in C#.
[UnmanagedFunctionPointer(CallingConvention.StdCall)]
delegate nint HandleFunc(IntPtr self);
