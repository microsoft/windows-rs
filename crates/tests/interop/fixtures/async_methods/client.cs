using System;
using System.Runtime.InteropServices;
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

        var failingAction = AsyncInfo.Run(_ => Task.FromException(new ArgumentOutOfRangeException()));
        CheckBounds(() => widget.AwaitAction(failingAction).AsTask());
        var failingActionWithProgress = AsyncInfo.Run<uint>((_, _) =>
            Task.FromException(new ArgumentOutOfRangeException()));
        CheckBounds(() => widget.AwaitActionWithProgress(failingActionWithProgress).AsTask());

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

        CheckError(() => { _ = widget.RejectOperation(); }, "component-sync");
        var failed = widget.FailOperation();
        CheckError(() => failed.AsTask().GetAwaiter().GetResult(), "component-async");
        var clientFailure = AsyncInfo.Run<int>(async _ =>
        {
            await Task.Yield();
            throw new COMException("client-async", CustomError);
        });
        var relayed = widget.AwaitOperation(clientFailure);
        CheckError(() => relayed.AsTask().GetAwaiter().GetResult(), "client-async");
    }

    const int CustomError = unchecked((int)0x8004A123);

    static void CheckError(Action call, string message)
    {
        try
        {
            call();
        }
        catch (COMException error)
        {
            Check(error.HResult == CustomError);
            Check(error.Message.Contains(message, StringComparison.Ordinal));
            return;
        }
        Check(false);
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
