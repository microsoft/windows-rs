using System;
using System.Runtime.CompilerServices;
using WeakReferences;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();

        var observer = new Observer();
        widget.Observe(observer);
        Check(widget.Notify(1));
        Check(observer.Value == 1);
        GC.KeepAlive(observer);

        // The widget's weak reference does not keep a collected observer alive.
        ObserveTemporary(widget);
        GC.Collect();
        GC.WaitForPendingFinalizers();
        GC.Collect();
        Check(!widget.Notify(2));
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    static void ObserveTemporary(Widget widget) => widget.Observe(new Observer());
}

class Observer : IObserver
{
    public int Value { get; private set; }

    public void Notify(int value) => Value = value;
}