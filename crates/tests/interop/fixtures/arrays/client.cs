using System.Linq;
using Arrays;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        Check(widget.Sum([1, 2, 3]) == 6);
        Check(widget.Sum([]) == 0);
        Check(widget.Join(["a", "b"]) == "a,b");

        var values = new int[3];
        widget.Fill(values);
        Check(values.SequenceEqual([1, 2, 3]));

        Check(widget.Range(3).SequenceEqual([1, 2, 3]));
        Check(widget.Range(0).Length == 0);
        Check(widget.Split("a,b").SequenceEqual(["a", "b"]));

        widget.RangeOut(2, out var range);
        Check(range.SequenceEqual([1, 2]));
    }
}
