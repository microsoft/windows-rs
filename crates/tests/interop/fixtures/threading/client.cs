using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using Threading;
using static Harness;

public static class Client
{
    const int Threads = 8;
    const int Calls = 1000;

    [DllImport("kernel32.dll")]
    static extern uint GetCurrentThreadId();

    public static void Test()
    {
        var widget = new Widget();

        // The widget is created on this thread but called directly from threads in other apartments.
        foreach (var apartment in new[] { ApartmentState.STA, ApartmentState.MTA })
        {
            uint thread = 0;
            uint current = 0;
            var worker = new Thread(() =>
            {
                thread = GetCurrentThreadId();
                current = widget.CurrentThread();
            });
            worker.SetApartmentState(apartment);
            worker.Start();
            worker.Join();
            Check(thread == current);
        }

        // Threads start together so calls overlap, and every call gets a unique count.
        using (var start = new Barrier(Threads))
        {
            var counts = Enumerable.Range(0, Threads).Select(_ => Task.Factory.StartNew(() =>
            {
                start.SignalAndWait();
                return Enumerable.Range(0, Calls).Select(_ => widget.Increment()).ToArray();
            }, TaskCreationOptions.LongRunning)).ToArray();

            var unique = counts.SelectMany(task => task.GetAwaiter().GetResult()).ToHashSet();
            Check(unique.SetEquals(Enumerable.Range(1, Threads * Calls)));
            Check(widget.Count == Threads * Calls);
        }

        // The component calls handlers on background threads.
        var sum = 0;
        var handlerThreads = new HashSet<uint>();
        Handler handler = value =>
        {
            Interlocked.Add(ref sum, value);
            lock (handlerThreads)
            {
                handlerThreads.Add(GetCurrentThreadId());
            }
        };

        var actions = Enumerable.Range(1, Threads).Select(value => widget.InvokeOnBackground(handler, value).AsTask()).ToArray();
        Task.WaitAll(actions);
        Check(sum == 36);
        Check(!handlerThreads.Contains(GetCurrentThreadId()));

        // A handler's error fails the action.
        try
        {
            widget.InvokeOnBackground(_ => throw new ArgumentOutOfRangeException(), 0).AsTask().GetAwaiter().GetResult();
            Check(false);
        }
        catch (ArgumentOutOfRangeException)
        {
        }
    }
}