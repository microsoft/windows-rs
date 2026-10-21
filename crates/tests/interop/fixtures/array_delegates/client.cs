using System.Linq;
using ArrayDelegates;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        widget.CheckStrings((string[] input, string[] fill, out string[] output) =>
        {
            Check(input.Length == fill.Length);
            input.Reverse().ToArray().CopyTo(fill, 0);
            output = input.ToArray();
            return fill.ToArray();
        });
        widget.CheckEntries((Entry[] input, Entry[] fill, out Entry[] output) =>
        {
            Check(input.Length == fill.Length);
            input.Reverse().ToArray().CopyTo(fill, 0);
            output = input.ToArray();
            return fill.ToArray();
        });
    }
}
