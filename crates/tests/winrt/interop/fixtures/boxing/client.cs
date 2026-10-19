using Boxing;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        Check(widget.Describe(1) == "i32 1");
        Check(widget.Describe("a") == "String a");
        Check(widget.Describe(null) == "null");

        Check((int)widget.BoxInt32(2) == 2);
        Check((string)widget.BoxString("b") == "b");

        Check(widget.Optional == null);
        widget.Optional = 3;
        Check(widget.Optional == 3);
        widget.Optional = null;
        Check(widget.Optional == null);
    }
}
