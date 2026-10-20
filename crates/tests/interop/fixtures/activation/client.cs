using System.Runtime.InteropServices;
using Activation;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Activation.Nested.Widget();
        Check(widget.Value == 123);

        Check(Settings.Version == 456);

        try
        {
            new Missing();
            Check(false);
        }
        // Unlike C++/WinRT and windows-rs, CsWinRT reports the component's error rather than
        // REGDB_E_CLASSNOTREG.
        catch (COMException e) when (e.HResult == unchecked((int)0x80040111))
        {
        }
    }
}
