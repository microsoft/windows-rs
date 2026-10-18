using System.Collections.Generic;
using System.Linq;
using Collections;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();

        IList<int> numbers = widget.Numbers();
        Check(numbers.Count == 3);
        numbers.Add(4);
        Check(numbers.Count == 4);
        Check(numbers[3] == 4);

        Check(widget.Lookup(new Dictionary<string, int> { ["one"] = 1 }, "one") == 1);

        Check(widget.Values().SequenceEqual(["one", "two"]));
    }
}
