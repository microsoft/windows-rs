using System.Collections.Generic;
using System.Linq;
using Enums;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        Check(widget.Next(Color.Red) == Color.Green);
        Check(widget.Next(Color.Blue) == Color.Red);
        Check(widget.Next(Color.Unknown) == Color.Unknown);
        Check(widget.Next((Color)7) == Color.Unknown);

        Check(widget.Styles == (Styles.Bold | Styles.Underline));
        widget.Styles = Styles.Italic;
        Check(widget.Styles == Styles.Italic);

        Check(widget.Colors().SequenceEqual(new[] { Color.Red, Color.Green, Color.Blue }));
        Check(widget.Combine(new List<Styles> { Styles.Bold, Styles.Underline }) == (Styles.Bold | Styles.Underline));

        Check((Color)widget.BoxColor(Color.Blue) == Color.Blue);
        Check(widget.UnboxStyles(Styles.Italic | Styles.Underline) == (Styles.Italic | Styles.Underline));
    }
}
