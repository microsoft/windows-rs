using System;
using System.Runtime.InteropServices;
using ErrorRelay;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        var callback = new Callback();
        Verify(unchecked((int)0x8000000B), "relay-bounds");
        Verify(unchecked((int)0x8004A123), "relay-custom");

        void Verify(int code, string message)
        {
            try
            {
                widget.Relay(callback, code, message);
            }
            catch (Exception error)
            {
                Check(error.HResult == code);
                Check(error.Message.Contains(message, StringComparison.Ordinal));
                return;
            }
            Check(false);
        }
    }
}

class Callback : ICallback
{
    public void Fail(int code, string message) => throw new COMException(message, code);
}
