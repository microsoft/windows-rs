using System;
using System.Linq;
using BoxedArrays;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        foreach (var count in new[] { 0, 3 })
        {
            var numbers = new[] { int.MinValue, 0, int.MaxValue }.Take(count).ToArray();
            Check(((int[])widget.BoxNumbers(numbers)).SequenceEqual(numbers));
            Check(widget.Numbers(numbers).SequenceEqual(numbers));

            var booleans = new[] { true, false, true }.Take(count).ToArray();
            Check(((bool[])widget.BoxBooleans(booleans)).SequenceEqual(booleans));
            Check(widget.Booleans(booleans).SequenceEqual(booleans));

            var strings = new[] { "first\0tail", "", "last" }.Take(count).ToArray();
            Check(((string[])widget.BoxStrings(strings)).SequenceEqual(strings));
            Check(widget.Strings(strings).SequenceEqual(strings));
        }

        try
        {
            widget.Numbers(new[] { "not a number" });
            Check(false);
        }
        catch (InvalidCastException)
        {
        }
    }
}
