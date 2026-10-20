using System.Collections.Generic;
using System.Linq;
using Structs;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        Check(widget.Offset(new Point(1, 2), 3) == new Point(4, 5));

        var mixed = widget.Step(new Mixed(true, 0x1_0000_0005, 254, 1.5f));
        Check(mixed == new Mixed(false, 0x1_0000_0006, 255, 2.5f));

        var label = widget.Label;
        Check(label.Text == "label" && label.Position == new Point(1, 2) && label.Width == null);
        widget.Label = new Label("text", new Point(3, 4), 5);
        Check(widget.Describe(widget.Label) == "text (3, 4) 5");
        Check(widget.Describe(default) == " (0, 0) none");

        Check(widget.Points().SequenceEqual(new[] { new Point(1, 2), new Point(3, 4) }));
        Check(widget.Sum(new List<Point> { new(1, 2), new(3, 4) }) == new Point(4, 6));

        var labels = widget.Labels(2);
        Check(labels.Length == 2);
        Check(labels[1].Text == "1" && labels[1].Position == new Point(1, 1) && labels[1].Width == 1);
        Check(widget.Labels(0).Length == 0);

        Check((Point)widget.BoxPoint(new Point(5, 6)) == new Point(5, 6));
        Check(widget.UnboxPoint(new Point(7, 8)) == new Point(7, 8));
    }
}
