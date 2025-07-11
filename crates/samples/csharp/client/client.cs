using System.Runtime.InteropServices;

[DllImport("csharp_component.dll", PreserveSig = false)]
static extern uint Add(uint left, uint right);

Console.WriteLine("Add: {0}", Add(4, 5));

[DllImport("csharp_component.dll", PreserveSig = false)]
[return: MarshalAs(UnmanagedType.BStr)]
static extern string Concat([MarshalAs(UnmanagedType.LPWStr)] string left, [MarshalAs(UnmanagedType.LPWStr)] string right);

Console.WriteLine("Concat: {0}", Concat("hello ", "world"));
