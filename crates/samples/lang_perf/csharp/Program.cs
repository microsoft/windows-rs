using System.Diagnostics;
using LangPerf;

ulong iterations = ParseIterations(args);

var o = new Class();
Console.WriteLine($"# C# consumer -> {o.Lang()} component - {iterations} iterations");

var timer = Stopwatch.StartNew();
for (ulong i = 0; i < iterations; i++)
{
    _ = new Class();
}
Report("Create", timer);

timer = Stopwatch.StartNew();
for (ulong i = 0; i < iterations; i++)
{
    o.Int32Property = 123;
    _ = o.Int32Property;
}
Report("Int32", timer);

timer = Stopwatch.StartNew();
for (ulong i = 0; i < iterations; i++)
{
    o.StringProperty = "value";
    _ = o.StringProperty;
}
Report("String", timer);

timer = Stopwatch.StartNew();
for (ulong i = 0; i < iterations; i++)
{
    o.ObjectProperty = o;
    _ = o.ObjectProperty;
}
Report("Object", timer);

timer = Stopwatch.StartNew();
for (ulong i = 0; i < iterations; i++)
{
    _ = ((INonDefault)o.ObjectProperty).Value();
}
Report("Cast", timer);

LangPerf.Handler handler = (sender, args) => { };
o.Event += handler;
timer = Stopwatch.StartNew();
for (ulong i = 0; i < iterations; i++)
{
    o.Raise();
}
Report("Event", timer);
o.Event -= handler;

timer = Stopwatch.StartNew();
for (ulong i = 0; i < iterations; i++)
{
    LangPerf.Handler h = (sender, args) => { };
    o.Event += h;
    o.Event -= h;
}
Report("AddRemove", timer);

{
    uint count = iterations > uint.MaxValue ? uint.MaxValue : (uint)iterations;
    var items = o.Items(count);

    timer = Stopwatch.StartNew();
    int sum = 0;
    foreach (var value in items)
    {
        sum += value;
    }
    GC.KeepAlive(sum);
    Report("IterateVector", timer);

    int[] buffer = new int[count];
    timer = Stopwatch.StartNew();
    items.CopyTo(buffer, 0);
    Report("GetMany", timer);

    var map = o.Map(count);
    timer = Stopwatch.StartNew();
    int msum = 0;
    foreach (var pair in map)
    {
        msum += pair.Value;
    }
    GC.KeepAlive(msum);
    Report("Map", timer);
}

timer = Stopwatch.StartNew();
for (ulong i = 0; i < iterations; i++)
{
    _ = o.Operation().GetAwaiter().GetResult();
}
Report("Async", timer);

timer = Stopwatch.StartNew();
for (ulong i = 0; i < iterations; i++)
{
    _ = o.Reference()!.Value;
}
Report("Reference", timer);

timer = Stopwatch.StartNew();
for (ulong i = 0; i < iterations; i++)
{
    try
    {
        _ = o.Next();
    }
    catch (Exception)
    {
    }
}
Report("Error", timer);

static void Report(string label, Stopwatch timer)
{
    timer.Stop();
    Console.WriteLine($"{label}: {timer.ElapsedMilliseconds} ms");
}

static ulong ParseIterations(string[] args)
{
    for (int i = 0; i + 1 < args.Length; i++)
    {
        if (args[i] == "--iterations")
        {
            return ulong.Parse(args[i + 1]);
        }
    }

    var env = Environment.GetEnvironmentVariable("LANG_PERF_ITER");
    return ulong.TryParse(env, out var value) ? value : 1_000;
}
