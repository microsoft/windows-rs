using System.Linq;
using Strings;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();

        Check(widget.Length("") == 0);
        Check(widget.Echo("") == "");
        // WinRT does not distinguish null and empty strings.
        Check(widget.Length(null!) == 0);
        Check(widget.Echo(null!) == "");

        var embedded = "a\0b";
        Check(widget.Length(embedded) == 3);
        Check(widget.Echo(embedded) == embedded);

        var pair = "\U0001F600";
        Check(widget.Length(pair) == 2);
        Check(widget.Echo(pair) == pair);

        var unpaired = "\ud800";
        Check(widget.Length(unpaired) == 1);
        Check(widget.Echo(unpaired) == unpaired);

        var large = widget.Repeat("ab", 100_000);
        Check(large.Length == 200_000);
        Check(large == string.Concat(Enumerable.Repeat("ab", 100_000)));
        Check(widget.Echo(large) == large);

        var greeting = widget.Greeting();
        Check(greeting == "h\u00e9llo \U0001F600");
        Check(greeting.Length == 8);

        Check(widget.Text == "");
        widget.Text = embedded;
        Check(widget.Text == embedded);
        widget.Text = "";
        Check(widget.Text == "");
    }
}