using System;
using System.Runtime.InteropServices.WindowsRuntime;
using System.Threading.Tasks;
using AsyncMethods;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        widget.Action().AsTask().GetAwaiter().GetResult();
        Check(widget.Operation(1).AsTask().GetAwaiter().GetResult() == 1);
        Check(widget.StringOperation("a").AsTask().GetAwaiter().GetResult() == "a");

        try
        {
            widget.Fail().AsTask().GetAwaiter().GetResult();
            Check(false);
        }
        catch (ArgumentOutOfRangeException)
        {
        }

        var operation = AsyncInfo.Run(async _ =>
        {
            await Task.Yield();
            return 41;
        });

        Check(widget.Increment(operation).AsTask().GetAwaiter().GetResult() == 42);
    }
}
