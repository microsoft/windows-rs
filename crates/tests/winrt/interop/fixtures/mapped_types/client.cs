using System;
using MappedTypes;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var time = new DateTimeOffset(2000, 1, 1, 0, 0, 0, TimeSpan.Zero);
        var span = TimeSpan.FromMinutes(90);
        var closed = Widget.Closed;

        using (var widget = new Widget())
        {
            Check(widget.Ticks(time) == 125_911_584_000_000_000);
            Check(widget.Duration(span) == span.Ticks);
            Check(widget.Add(time, span) == time + span);

            Check(widget.Host(new Uri("https://example.com/path")) == "example.com");
            Check(widget.CreateUri("https://example.com/path") == new Uri("https://example.com/path"));

            Check(widget.ToString() == "widget");
        }

        Check(Widget.Closed == closed + 1);
    }
}