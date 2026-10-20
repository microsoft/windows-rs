using Overloads;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        Check(widget.Sum() == 0);
        Check(widget.Sum(1) == 1);
        Check(widget.Sum(1, 2) == 3);
        Check(widget.Sum(1, 2, 3) == 6);
        Check(widget.Format(1) == "int 1");
        Check(widget.Format("a") == "text a");
    }
}
