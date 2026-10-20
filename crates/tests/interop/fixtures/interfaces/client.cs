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

        var square = new Square();
        Check(square.Area() == 4);
        Check(ReferenceEquals(widget.Echo(square), square));
        shape = square;
        Check(shape.Area() == 4);
    }
}

class Shape : IShape
{
    public int Area() => 2;
}
