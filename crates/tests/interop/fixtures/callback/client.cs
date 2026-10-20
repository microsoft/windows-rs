using Callback;

public static class Client
{
    public static void Test()
    {
        new Widget().Check(new Value(0));
    }
}

// A null `other` fails with `E_POINTER` because .NET reports a `NullReferenceException` with it.
class Value(int current) : IValue
{
    public int Current { get; set; } = current;

    public int Add(IValue other) => Current + other.Current;

    public void Create(int current, out IValue result) => result = new Value(current);
}
