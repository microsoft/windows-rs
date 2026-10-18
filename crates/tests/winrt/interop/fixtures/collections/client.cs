using System.Collections.Generic;
using Test;

public static class Client
{
    public static int Use(Widget widget)
    {
        IList<int> numbers = widget.Numbers();
        numbers.Add(1);

        int total = widget.Lookup(new Dictionary<string, int> { ["one"] = 1 });

        foreach (string value in widget.Values())
        {
            total += value.Length;
        }

        return total + numbers.Count;
    }
}
