using System;
using Errors;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();

        // CsWinRT maps `E_BOUNDS` to `ArgumentOutOfRangeException`.
        try
        {
            widget.Fail();
            Check(false);
        }
        catch (ArgumentOutOfRangeException e)
        {
            Check(e.HResult == E_BOUNDS);
        }

        try
        {
            widget.FailWithMessage("component");
            Check(false);
        }
        catch (ArgumentOutOfRangeException e)
        {
            Check(e.HResult == E_BOUNDS);
            // CsWinRT appends the message to the system description of `E_BOUNDS`.
            Check(e.Message.Contains("component"));
        }

        widget.Check(new Callback());
    }

    const int E_BOUNDS = unchecked((int)0x8000000B);
}

class Callback : ICallback
{
    public void Fail() => throw new Exception("client") { HResult = unchecked((int)0x8000000B) };
}
