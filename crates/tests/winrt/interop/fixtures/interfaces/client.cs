using Interfaces;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        Check(widget.Area() == 1);

        IShape shape = widget;
        Check(shape.Area() == 1);
        Check(ReferenceEquals(widget.Echo(shape), widget));

        shape = new Shape();
        Check(ReferenceEquals(widget.Echo(shape), shape));
    }
}

class Shape : IShape
{
    public int Area() => 2;
}
