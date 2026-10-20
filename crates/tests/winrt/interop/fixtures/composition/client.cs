using System;
using System.Runtime.CompilerServices;
using System.Threading;
using Composition;
using Windows.Foundation;
using static Harness;

// Overrides `Name` and adds an interface that the base class does not implement.
class Derived : Base, IStringable
{
    public static int Live;

    public Derived(int value) : base(value)
    {
        Interlocked.Increment(ref Live);
    }

    ~Derived()
    {
        Interlocked.Decrement(ref Live);
    }

    protected override string Name() => "Cs:" + base.Name();

    public override string ToString() => "Stringable";

    // The base class's protected interface is only available to a derived class.
    public int ProtectedDoubled() => Doubled();
}

// Derives without overriding anything.
class Plain : Base
{
}

public static class Client
{
    public static void Test()
    {
        Check(Base.Instances == 0);
        CheckBase();
        CheckDerived();
        CheckPlain();

        // The C# objects release the native objects when they are finalized.
        Collect();
        Check(Base.Instances == 0);
        Check(Derived.Live == 0);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    static void CheckBase()
    {
        var base_ = new Base(5);
        Check(base_.Value == 5);
        Check(base_.Describe() == "Base(5)");
        Check(base_.Stringify() == "");
        Check(Base.Instances == 1);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    static void CheckDerived()
    {
        var derived = new Derived(7);
        Base base_ = derived;

        // The base calls the override through the outer object.
        Check(base_.Value == 7);
        Check(base_.Describe() == "Cs:Base(7)");
        Check(base_.Stringify() == "Stringable");
        Check(derived.ProtectedDoubled() == 14);
        Check(Derived.Live == 1);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    static void CheckPlain()
    {
        var plain = new Plain();
        Check(plain.Describe() == "Base(0)");

        // CsWinRT implements `IStringable` with `Object.ToString` for every C# object.
        Check(plain.Stringify() == "Plain");
    }

    static void Collect()
    {
        for (var i = 0; i < 2; i++)
        {
            GC.Collect();
            GC.WaitForPendingFinalizers();
        }
    }
}
