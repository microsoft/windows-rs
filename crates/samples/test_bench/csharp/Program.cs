// windows-csharp consumer of the Bench.Widget component. Every type used here -- the `Widget`
// owner, the `INonDefault` cast target, and the runtime activation support -- comes from the
// generated `Bench.cs` produced by the windows-csharp generator (see build.rs). This is the
// dogfooding column: the same safe direct-vtable projection the generator emits for any winmd.

using System;
using System.Diagnostics;
using System.Runtime.InteropServices;
using Bench;
using GeneratedComBench;

long iterations = 1000;
bool profileGeneratedCom = Array.IndexOf(args, "--generated-com") >= 0;
for (int i = 0; i + 1 < args.Length; i++)
{
    if (args[i] == "--iterations")
    {
        iterations = long.Parse(args[i + 1]);
    }
}

using Widget widget = new();
Console.WriteLine($"# windows-csharp consumer -> Bench component - {iterations} iterations");

Stopwatch sw = Stopwatch.StartNew();
for (long i = 0; i < iterations; i++)
{
    using Widget scratch = new();
}
Report("Create", sw);

widget.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        borrowed.Int32Property = 123;
        _ = borrowed.Int32Property;
    }
    Report("Int32", timer);
});

widget.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        borrowed.StringProperty = "value";
        _ = borrowed.StringProperty;
    }
    Report("String", timer);
});

int sum = 0;
widget.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        sum += borrowed.Add((int)i, 1);
    }
    Report("Add", timer);
});
GC.KeepAlive(sum);

// Cast: the borrowed runtime class forwards a non-default-interface method through QI, the vtable
// call, and Release. The outer borrow protects the long-lived source for the loop; each temporary
// interface pointer is released immediately without a managed owner or cache.
sum = 0;
widget.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        sum += borrowed.Value();
    }
    Report("Cast", timer);
});
GC.KeepAlive(sum);

// CastOwned: the escapable form. Each QI result is wrapped in an independently owned, finalizable
// managed object and disposed deterministically.
sum = 0;
widget.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        using INonDefault nd = borrowed.As<INonDefault>();
        sum += nd.Value();
    }
    Report("CastOwned", timer);
});
GC.KeepAlive(sum);

// Interface: acquire the non-default interface once, then measure steady calls through that owner.
using (INonDefault nd = widget.As<INonDefault>())
{
    sum = 0;
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        sum += nd.Value();
    }
    Report("Interface", timer);
    GC.KeepAlive(sum);
}

// Object: set and get metadata Object (IInspectable). The setter borrows the input and each getter
// result is one small owning wrapper disposed deterministically.
widget.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        borrowed.ObjectProperty = widget;
        using WindowsCsharp.ComObject value = borrowed.ObjectProperty!;
    }
    Report("Object", timer);
});

// Event: subscribe one handler, then raise the event N times. Each Signal calls from the
// component back into this consumer's delegate through the reverse vtable the projection builds
// (native Invoke -> managed callback). The handler is created once; raising it allocates nothing.
using ChangedHandler handler = ChangedHandler.Create((_, _) => { });
long changedToken = widget.AddChanged(handler);
widget.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        borrowed.Signal((int)i);
    }
    Report("Event", timer);
});
widget.RemoveChanged(changedToken);

// AddRemove: subscribe and unsubscribe the same handler N times, measuring event-registration
// churn. The token is a plain long, so add and remove add no per-object bookkeeping or allocation.
widget.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        long token = borrowed.AddChanged(handler);
        borrowed.RemoveChanged(token);
    }
    Report("AddRemove", timer);
});

// Vector: read elements from a projected generic collection `IVector<int>`. The vector is built
// once, then GetAt reads one element per iteration - the per-element cost of a generic collection
// call across the ABI. The borrowed view holds one call lease around the full loop, so each read is
// a direct vtable call over `int` with no boxing or per-element allocation.
const uint vectorLen = 1024;
using Windows.Foundation.Collections.IVector<int> vector = widget.Items(vectorLen)!;
sum = 0;
vector.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        sum += borrowed.GetAt((uint)(i % vectorLen));
    }
    Report("Vector", timer);
});
GC.KeepAlive(sum);

// IterateVector: a `foreach` over the whole collection, repeated a bounded number of passes. The
// projection's GetEnumerator returns a struct enumerator that batches through GetMany into a
// stack `[InlineArray]` buffer, so the loop allocates nothing and makes one vtable call per block
// instead of one per element (unlike an RCW-backed IEnumerable<int> that boxes an enumerator per
// pass and marshals each element).
long iteratePasses = Math.Min(iterations, 100_000);
sw = Stopwatch.StartNew();
sum = 0;
for (long i = 0; i < iteratePasses; i++)
{
    foreach (int v in vector)
    {
        sum += v;
    }
}
GC.KeepAlive(sum);
Report("IterateVector", sw);

// GetMany: copy a vector sized to the requested iteration count into one caller-owned buffer.
// Construction and allocation happen before the timer so this isolates the bulk ABI call.
uint bulkCount = (uint)Math.Min(iterations, int.MaxValue);
using Windows.Foundation.Collections.IVector<int> bulkVector = widget.Items(bulkCount)!;
int[] buffer = new int[bulkCount];
sw = Stopwatch.StartNew();
uint actual = bulkVector.GetMany(0, buffer);
GC.KeepAlive((actual, buffer));
Report("GetMany", sw);

const uint mapLen = 1024;

// Map: enumerate an IMap<string,int>. Repeat a bounded 1024-entry map enough times to keep total
// entry visits near the requested count.
using Windows.Foundation.Collections.IMap<string, int> stringMap = widget.StringMap(mapLen)!;
long mapPasses = Math.Max(iterations / mapLen, 1);
sw = Stopwatch.StartNew();
sum = 0;
for (long i = 0; i < mapPasses; i++)
{
    foreach (var pair in stringMap)
    {
        sum += pair.Value;
    }
}
GC.KeepAlive(sum);
Report("Map", sw);

// Lookup: read values from a projected generic dictionary IMap<int,int> by key. The map is built
// once, then Lookup reads one value per iteration. The borrowed view holds one call lease around
// the loop, so each lookup is a direct vtable call with no boxing or per-lookup allocation.
using Windows.Foundation.Collections.IMap<int, int> map = widget.Map(mapLen)!;
sum = 0;
map.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        sum += borrowed.Lookup((int)(i % mapLen));
    }
    Report("Lookup", timer);
});
GC.KeepAlive(sum);

// VectorView: read elements from the read-only view IVectorView<int>. Same per-element vtable cost
// as Vector - a direct generic call over `int` - confirming the view projects like the collection.
using Windows.Foundation.Collections.IVectorView<int> vectorView = widget.ItemsView(vectorLen)!;
sum = 0;
vectorView.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        sum += borrowed.GetAt((uint)(i % vectorLen));
    }
    Report("VectorView", timer);
});
GC.KeepAlive(sum);

// MapView: read values from the read-only view IMapView<int,int> by key. Same per-lookup vtable
// cost as Lookup.
using Windows.Foundation.Collections.IMapView<int, int> mapView = widget.MapView(mapLen)!;
sum = 0;
mapView.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        sum += borrowed.Lookup((int)(i % mapLen));
    }
    Report("MapView", timer);
});
GC.KeepAlive(sum);

// Reference: box a nullable int input as IReference<int> and unbox the returned value.
widget.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        borrowed.ReferenceProperty = 0;
        _ = borrowed.ReferenceProperty!.Value;
    }
    Report("Reference", timer);
});

// Async: obtain an already-completed IAsyncOperation<int> and synchronously read its result.
widget.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        using Windows.Foundation.IAsyncOperation<int> operation = borrowed.Operation()!;
        _ = operation.GetAwaiter().GetResult();
    }
    Report("Async", timer);
});

// Error: call a method that always returns a failing HRESULT. Com.Check throws, and the caller
// catches it -- the idiomatic C# error model. Throwing and catching a managed exception costs
// orders of magnitude more than the scalar calls above, so this loop runs a reduced count.
long failIterations = Math.Min(iterations, 1_000_000);
long errors = 0;
widget.Borrow(borrowed =>
{
    Stopwatch timer = Stopwatch.StartNew();
    for (long i = 0; i < failIterations; i++)
    {
        try { borrowed.Fail(); } catch (Exception) { errors++; }
    }
    Report("Error", timer);
});
GC.KeepAlive(errors);

// Leak check: activate, cast, and dispose N objects, then confirm the component's live instance
// count returns to the baseline. Every AddRef the projection issues is matched by idempotent
// Dispose/finalizer ownership.
int baseline = widget.LiveCount();
for (long i = 0; i < iterations; i++)
{
    using Widget scratch = new();
    using INonDefault nd = scratch.As<INonDefault>();
    _ = nd.Value();
}
Console.WriteLine($"Leak: {widget.LiveCount() - baseline}");

// Scalability: retain N live owners and report the managed heap cost per object.
int live = (int)Math.Min(iterations, 1_000_000);
long before = GC.GetTotalAllocatedBytes(precise: true);
Widget[] widgets = new Widget[live];
for (int i = 0; i < live; i++)
{
    widgets[i] = new Widget();
}

long after = GC.GetTotalAllocatedBytes(precise: true);
long bytes = after - before;
Console.WriteLine($"Live-{live}: {bytes} bytes ({(double)bytes / live:F1} bytes/object)");

for (int i = 0; i < live; i++)
{
    widgets[i].Dispose();
}

if (profileGeneratedCom)
{
    int generatedBaseline = RunGeneratedComProfile(widget, iterations, live);
    GC.Collect();
    GC.WaitForPendingFinalizers();
    GC.Collect();
    Console.WriteLine($"GeneratedComLeak: {widget.LiveCount() - generatedBaseline}");
}

static int RunGeneratedComProfile(Widget widget, long iterations, int live)
{
    int baseline = widget.LiveCount();
    IWidgetAbi generatedWidget = GeneratedWidget.Create();
    IWidgetPreserveSigAbi generatedRawWidget = GeneratedWidget.CreatePreserveSig();

    generatedWidget.GetIids(out uint iidCount, out nint iids);
    try
    {
        if (iidCount != 0 && iids == 0)
        {
            throw new InvalidOperationException("generated IInspectable GetIids returned null");
        }
    }
    finally
    {
        Marshal.FreeCoTaskMem(iids);
    }
    string className = WindowsCsharp.Interop.FromHstring(generatedWidget.GetRuntimeClassName());
    int trustLevel = generatedWidget.GetTrustLevel();
    if (className != "Bench.IWidget" || trustLevel != 0)
    {
        throw new InvalidOperationException(
            $"generated IInspectable base slots are incorrect: {className}, {trustLevel}");
    }

    widget.Int32Property = 123;
    _ = widget.Int32Property;
    Stopwatch sw = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        widget.Int32Property = 123;
        _ = widget.Int32Property;
    }
    Report("OwnerInt32", sw);

    widget.StringProperty = "value";
    _ = widget.StringProperty;
    sw = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        widget.StringProperty = "value";
        _ = widget.StringProperty;
    }
    Report("OwnerString", sw);

    int sum = widget.Add(1, 1);
    sw = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        sum += widget.Add((int)i, 1);
    }
    Report("OwnerAdd", sw);
    GC.KeepAlive(sum);

    generatedWidget.SetInt32Property(123);
    _ = generatedWidget.GetInt32Property();
    sw = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        generatedWidget.SetInt32Property(123);
        _ = generatedWidget.GetInt32Property();
    }
    Report("GeneratedComInt32", sw);

    generatedWidget.SetStringProperty("value");
    _ = generatedWidget.GetStringProperty();
    sw = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        generatedWidget.SetStringProperty("value");
        _ = generatedWidget.GetStringProperty();
    }
    Report("GeneratedComString", sw);

    sum = generatedWidget.Add(1, 1);
    sw = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        sum += generatedWidget.Add((int)i, 1);
    }
    Report("GeneratedComAdd", sw);
    GC.KeepAlive(sum);

    WindowsCsharp.Com.Check(generatedRawWidget.SetInt32Property(123));
    WindowsCsharp.Com.Check(generatedRawWidget.GetInt32Property(out _));
    sw = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        WindowsCsharp.Com.Check(generatedRawWidget.SetInt32Property(123));
        WindowsCsharp.Com.Check(generatedRawWidget.GetInt32Property(out _));
    }
    Report("GeneratedComRawInt32", sw);

    WindowsCsharp.Com.Check(generatedRawWidget.Add(1, 1, out sum));
    sw = Stopwatch.StartNew();
    for (long i = 0; i < iterations; i++)
    {
        WindowsCsharp.Com.Check(generatedRawWidget.Add((int)i, 1, out int value));
        sum += value;
    }
    Report("GeneratedComRawAdd", sw);
    GC.KeepAlive(sum);

    GC.Collect();
    GC.WaitForPendingFinalizers();
    GC.Collect();
    long before = GC.GetTotalAllocatedBytes(precise: true);
    IWidgetAbi[] generatedWidgets = new IWidgetAbi[live];
    for (int i = 0; i < live; i++)
    {
        generatedWidgets[i] = GeneratedWidget.Create();
    }
    long after = GC.GetTotalAllocatedBytes(precise: true);
    long bytes = after - before;
    Console.WriteLine(
        $"GeneratedComLive-{live}: {bytes} bytes ({(double)bytes / live:F1} bytes/object)");
    return baseline;
}

static void Report(string label, Stopwatch sw) =>
    Console.WriteLine($"{label}: {sw.ElapsedMilliseconds} ms");
