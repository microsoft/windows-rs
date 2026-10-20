using System;
using Statics;
using static Harness;

public static class Client
{
    public static void Test()
    {
        Check(Widget.Add(1, 2) == 3);

        var changed = 0;
        EventHandler<int> handler = (sender, value) =>
        {
            Check(sender == null);
            changed = value;
        };

        Widget.Changed += handler;
        Widget.Value = 5;
        Check(Widget.Value == 5);
        Check(changed == 5);

        Widget.Changed -= handler;
        Widget.Value = 6;
        Check(Widget.Value == 6);
        Check(changed == 5);
    }
}