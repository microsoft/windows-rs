using System.Runtime.InteropServices;

// HSTRING helper functions from the Windows Runtime string API.
[DllImport("api-ms-win-core-winrt-string-l1-1-0.dll", CharSet = CharSet.Unicode, PreserveSig = false)]
static extern void WindowsCreateString(string sourceString, uint length, out nint hstring);

[DllImport("api-ms-win-core-winrt-string-l1-1-0.dll", PreserveSig = false)]
static extern void WindowsDeleteString(nint hstring);

// IActivationFactory is the WinRT interface used to activate runtime classes.
// Its vtable begins with IUnknown (3 slots, handled automatically), then the
// three IInspectable methods, then ActivateInstance.
[ComImport]
[Guid("00000035-0000-0000-c000-000000000046")]
[InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
interface IActivationFactory
{
    [PreserveSig] int GetIids(out int count, out nint iids);
    [PreserveSig] int GetRuntimeClassName(out nint name);
    [PreserveSig] int GetTrustLevel(out int level);
    [PreserveSig] int ActivateInstance([MarshalAs(UnmanagedType.IUnknown)] out object instance);
}

// IRobot is a WinRT interface (extends IInspectable), so its vtable starts
// with three IUnknown slots (automatic), three IInspectable slots, then Speak.
[ComImport]
[Guid("d93d56c9-37a7-537e-becc-236d421cc48f")]
[InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
interface IRobot
{
    [PreserveSig] int GetIids(out int count, out nint iids);
    [PreserveSig] int GetRuntimeClassName(out nint name);
    [PreserveSig] int GetTrustLevel(out int level);
    [PreserveSig] int Speak(nint message);
}

// IRobotInterop is a plain COM interface (extends IUnknown only).
[ComImport]
[Guid("ae60832b-0bc8-57b0-8a69-f82ebc1560ed")]
[InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
interface IRobotInterop
{
    nint Handle();
}

// Obtain the activation factory from the robotics DLL directly.
[DllImport("robotics.dll", PreserveSig = false)]
static extern void DllGetActivationFactory(nint name, [MarshalAs(UnmanagedType.IUnknown)] out object factory);

// Create a robot from an arbitrary handle value.
[DllImport("robotics.dll", PreserveSig = false)]
static extern void CreateRobotFromHandle(nint handle, [MarshalAs(UnmanagedType.IUnknown)] out object robot);

// Activate the Robot class through its WinRT factory.
WindowsCreateString("Robotics.Robot", (uint)"Robotics.Robot".Length, out var hname);
DllGetActivationFactory(hname, out var factoryObj);
WindowsDeleteString(hname);
var factory = (IActivationFactory)factoryObj;
factory.ActivateInstance(out var robotObj);
var robot = (IRobot)robotObj;

// Call Speak on the activated robot.
WindowsCreateString("Hello from cs land", (uint)"Hello from cs land".Length, out var hstr1);
robot.Speak(hstr1);
WindowsDeleteString(hstr1);

// Create a second robot from a raw handle and call Speak on it.
CreateRobotFromHandle(0x1c8, out var handyObj);
var handyRobot = (IRobot)handyObj;
WindowsCreateString("Hello handy", (uint)"Hello handy".Length, out var hstr2);
handyRobot.Speak(hstr2);
WindowsDeleteString(hstr2);

// Query for the IRobotInterop interface and retrieve the stored handle.
var interop = (IRobotInterop)handyObj;
var handle = interop.Handle();
Console.WriteLine($"interop handle: 0x{handle:x}");
