using System;
using System.Linq;
using NontrivialArrays;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        foreach (var count in new[] { 0, 1, 3 })
        {
            string[] strings = ["first\0tail", "", "last"];
            var stringsFill = new string[count];
            var stringsReturned = widget.Strings(strings.Take(count).ToArray(), stringsFill, out var stringsOut);
            Check(stringsOut.SequenceEqual(strings.Take(count)));
            Check(stringsFill.SequenceEqual(strings.Take(count).Reverse()));
            Check(stringsReturned.SequenceEqual(stringsFill));

            Entry[] entries =
            [
                new("first\0tail", '\u03a9', true, -7),
                new("", '\0', false, null),
                new("last", '\uffff', false, 42)
            ];
            var entriesFill = new Entry[count];
            var entriesReturned = widget.Entries(entries.Take(count).ToArray(), entriesFill, out var entriesOut);
            Check(entriesOut.SequenceEqual(entries.Take(count)));
            Check(entriesFill.SequenceEqual(entries.Take(count).Reverse()));
            Check(entriesReturned.SequenceEqual(entriesFill));

            var valuesFill = new IValue[count];
            var valuesReturned = widget.Values(
                new IValue[] { widget.CreateValue(7), null!, new Value() }.Take(count).ToArray(),
                valuesFill, out var valuesOut);
            GC.Collect();
            GC.WaitForPendingFinalizers();
            int?[] numbers = [7, null, 42];
            Check(valuesOut.Select(value => (int?)value?.Number).SequenceEqual(numbers.Take(count)));
            Check(valuesFill.Select(value => (int?)value?.Number).SequenceEqual(numbers.Take(count).Reverse()));
            Check(valuesReturned.Select(value => (int?)value?.Number).SequenceEqual(numbers.Take(count).Reverse()));
        }
    }
}

class Value : IValue
{
    public int Number => 42;
}
