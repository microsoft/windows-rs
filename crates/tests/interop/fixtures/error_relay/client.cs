using System;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
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

        using var checkpoints = new CountdownEvent(2);
        var workers = new Task[2];
        for (var index = 0; index < workers.Length; ++index)
        {
            var caller = index;
            workers[index] = Task.Run(() =>
            {
                var concurrent = new Callback(() =>
                {
                    checkpoints.Signal();
                    Check(checkpoints.Wait(TimeSpan.FromSeconds(10)));
                });
                var code = unchecked((int)0x8004A123) + caller;
                var message = $"thread-{caller}";
                try
                {
                    widget.RelayPreserved(concurrent, code, message);
                }
                catch (Exception error)
                {
                    Check(error.HResult == code);
                    Check(error.Message.Contains(message, StringComparison.Ordinal));
                    Check(concurrent.Calls == 2);
                    return;
                }
                Check(false);
            });
        }
        Task.WaitAll(workers);
        Check(checkpoints.IsSet);

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

class Callback(Action? checkpoint = null) : ICallback
{
    public int Calls { get; private set; }

    public void Fail(int code, string message)
    {
        ++Calls;
        throw new COMException(message, code);
    }

    public void Checkpoint()
    {
        Check(checkpoint != null);
        checkpoint!();
    }
}
