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

        widget.Action(1).AsTask().GetAwaiter().GetResult();
        widget.ActionWithProgress(1).AsTask(new Progress<uint>()).GetAwaiter().GetResult();
        Check(widget.Operation(1).AsTask().GetAwaiter().GetResult() == 1);
        Check(widget.OperationWithProgress(1).AsTask(new Progress<uint>()).GetAwaiter().GetResult() == 1);

        CheckBounds(() => widget.Action(-1).AsTask());
        CheckBounds(() => widget.ActionWithProgress(-1).AsTask());
        CheckBounds(() => widget.Operation(-1).AsTask());
        CheckBounds(() => widget.OperationWithProgress(-1).AsTask());

        var action = AsyncInfo.Run(async _ => await Task.Yield());
        widget.AwaitAction(action).AsTask().GetAwaiter().GetResult();

        var actionWithProgress = AsyncInfo.Run<uint>(async (_, progress) =>
        {
            await Task.Yield();
            progress.Report(1);
        });
        widget.AwaitActionWithProgress(actionWithProgress).AsTask().GetAwaiter().GetResult();

        var operation = AsyncInfo.Run(async _ =>
        {
            await Task.Yield();
            return 41;
        });
        Check(widget.AwaitOperation(operation).AsTask().GetAwaiter().GetResult() == 42);

        var operationWithProgress = AsyncInfo.Run<int, uint>(async (_, progress) =>
        {
            await Task.Yield();
            progress.Report(1);
            return 41;
        });
        Check(widget.AwaitOperationWithProgress(operationWithProgress).AsTask().GetAwaiter().GetResult() == 42);
    }

    static void CheckBounds(Func<Task> start)
    {
        try
        {
            start().GetAwaiter().GetResult();
            Check(false);
        }
        catch (ArgumentOutOfRangeException)
        {
        }
    }
}
