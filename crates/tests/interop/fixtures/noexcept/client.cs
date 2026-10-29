using System;
using Noexcept;
using static Harness;

public static class Client
{
    public static void Test()
    {
        Check(StaticValue.Number == 0);
        StaticValue.Number = 123;
        Check(StaticValue.Number == 123);
        Check(StaticValue.Text == "static");
        Check(StaticValue.Object(false) == null);
        Check(StaticValue.Object(true).Number == 0);
        var numbers = StaticValue.Numbers();
        Check(numbers.Length == 3 && numbers[0] == 2 && numbers[1] == 3 && numbers[2] == 5);
        Check(StaticValue.Checked(7) == 7);
        try
        {
            StaticValue.Checked(-1);
            Check(false);
        }
        catch (ArgumentException)
        {
        }
        StaticValue.Number = 0;

        var widget = new Widget();

        Check(widget.Number == 0);
        widget.Number = 123;
        Check(widget.Number == 123);

        Check(widget.Text == "");
        widget.Text = "abc";
        Check(widget.Text == "abc");

        Check(widget.Next == null);
        widget.Next = widget;
        Check(ReferenceEquals(widget.Next, widget));

        Check(widget.Limit == 0);
        widget.Limit = 5;
        Check(widget.Limit == 5);

        try
        {
            widget.Limit = -1;
            Check(false);
        }
        catch (ArgumentException)
        {
        }

        Check(widget.Limit == 5);

        // Clearing releases the widget's reference to itself.
        widget.Clear();
        Check(widget.Number == 0);
        Check(widget.Text == "");
        Check(widget.Next == null);
        Check(widget.Limit == 0);

        widget.Check(new Value());
    }
}

class Value : IValue
{
    int limit;

    public int Number { get; set; }

    public string Text { get; set; } = "";

    public IValue? Next { get; set; }

    public int Limit
    {
        get => limit;
        set => limit = value >= 0 ? value : throw new ArgumentException();
    }

    public void Clear()
    {
        Number = 0;
        Text = "";
        Next = null;
        limit = 0;
    }
}
