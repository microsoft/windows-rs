using Constructors;
using static Harness;

public static class Client
{
    public static void Test()
    {
        Check(new Widget().Value == 0);
        Check(new Widget(123).Value == 123);

        Check(new Base().Value == 0);
        Check(new Base(456).Value == 456);
    }
}
