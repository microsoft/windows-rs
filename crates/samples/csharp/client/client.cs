using System.Runtime.InteropServices;

[DllImport("csharp_component.dll", PreserveSig = false)]
static extern uint Add(uint left, uint right);

Console.WriteLine("result = {0:D}", Add(4, 5));
