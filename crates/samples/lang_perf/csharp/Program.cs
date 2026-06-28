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
