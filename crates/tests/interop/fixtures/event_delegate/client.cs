using EventDelegate;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        Widget? sender = null;
        int received = 0;

        ChangedHandler handler = (s, value) =>
        {
            sender = s;
            received = value;
        };

        widget.Changed += handler;
        widget.Raise(1);
        Check(sender == widget);
        Check(received == 1);

        widget.Changed -= handler;
        widget.Raise(2);
        Check(received == 1);
    }
}
