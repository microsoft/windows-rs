using StructDirections;
using static Harness;

public static class Client
{
    static Pair ProcessPair(Pair value, in Pair borrowed, out Pair output)
    {
        output = new(value.X + borrowed.X, value.Y + borrowed.Y);
        return new(value.X - borrowed.X, value.Y - borrowed.Y);
    }

    static Label ProcessLabel(Label value, in Label borrowed, out Label output)
    {
        var position = ProcessPair(value.Position, in borrowed.Position, out var sum);
        output = new(value.Text + borrowed.Text, sum, value.Width);
        return new(borrowed.Text + value.Text, position, borrowed.Width);
    }

    public static void Test()
    {
        var widget = new Widget();
        var value = new Pair(3, 0x1_0000_0000);
        var borrowed = new Pair(-5, 7);
        var returned = widget.ProcessPair(value, in borrowed, out var output);
        Check(output == new Pair(-2, 0x1_0000_0007));
        Check(returned == new Pair(8, 0xffff_fff9));

        var left = new Label("left\0tail", value, 17);
        var right = new Label("right", borrowed, null);
        var labelReturned = widget.ProcessLabel(left, in right, out var labelOutput);
        Check(labelOutput == new Label("left\0tailright", new Pair(-2, 0x1_0000_0007), 17));
        Check(labelReturned == new Label("rightleft\0tail", new Pair(8, 0xffff_fff9), null));

        widget.CheckPair(ProcessPair);
        widget.CheckLabel(ProcessLabel);
    }
}
