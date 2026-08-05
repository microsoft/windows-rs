// Standard C#/WinRT (CsWinRT) consumer of the Bench.Widget component, generated from the same
// bench.winmd the other consumers use. This is the idiomatic projection: a projected `Widget`
// class backed by an RCW. It measures what the conventional projection model costs relative to
// Rust and C++/WinRT, including the per-object memory of the RCW.

using System;
using System.Diagnostics;
using Bench;
using WinRT;

long iterations = 1000;
for (int i = 0; i + 1 < args.Length; i++)
{
    if (args[i] == "--iterations")
    {
        iterations = long.Parse(args[i + 1]);
    }
}

var widget = new Widget();
Console.WriteLine($"# C#/WinRT 2.x consumer -> Bench component - {iterations} iterations");

var sw = Stopwatch.StartNew();
for (long i = 0; i < iterations; i++)
{
    _ = new Widget();
}
Report("Create", sw);
DrainFinalizers();

sw = Stopwatch.StartNew();
for (long i = 0; i < iterations; i++)
{
    widget.Int32Property = 123;
    _ = widget.Int32Property;
}
Report("Int32", sw);

sw = Stopwatch.StartNew();
for (long i = 0; i < iterations; i++)
{
    widget.StringProperty = "value";
    _ = widget.StringProperty;
}
Report("String", sw);

sw = Stopwatch.StartNew();
int sum = 0;
for (long i = 0; i < iterations; i++)
{
    sum += widget.Add((int)i, 1);
}
GC.KeepAlive(sum);
Report("Add", sw);

// Interface: acquire the projected interface once, then measure steady calls.
INonDefault nonDefault = widget.As<INonDefault>();
sw = Stopwatch.StartNew();
sum = 0;
for (long i = 0; i < iterations; i++)
{
    sum += nonDefault.Value();
}
GC.KeepAlive(sum);
Report("Interface", sw);

// Object: set and get metadata Object (IInspectable). CsWinRT resolves each returned pointer to an
// RCW and keeps its native reference until GC finalization.
sw = Stopwatch.StartNew();
for (long i = 0; i < iterations; i++)
{
    widget.ObjectProperty = widget;
    _ = widget.ObjectProperty;
}
Report("Object", sw);
DrainFinalizers();

// Event: subscribe one handler and raise it N times. CsWinRT projects the WinRT event as an
// idiomatic C# event, so `Signal` calls back into the delegate through CsWinRT's event plumbing.
ChangedHandler handler = (sender, value) => { };
widget.Changed += handler;
sw = Stopwatch.StartNew();
for (long i = 0; i < iterations; i++)
{
    widget.Signal((int)i);
}
Report("Event", sw);
widget.Changed -= handler;

// AddRemove: subscribe and unsubscribe the same handler N times. CsWinRT keeps a per-object event
// table so `+=` / `-=` look up and mutate managed state on every call.
sw = Stopwatch.StartNew();
for (long i = 0; i < iterations; i++)
{
    widget.Changed += handler;
    widget.Changed -= handler;
}
Report("AddRemove", sw);

// Vector: read elements from a projected generic collection IVector<int>. The vector is built
// once, then the indexer reads one element per iteration. CsWinRT projects IVector<int> as
// IList<int>; each element read crosses the ABI through the RCW, so this exposes the per-element
// cost the struct-over-pointer consumers avoid.
const uint vectorLen = 1024;
var vector = widget.Items(vectorLen);
sw = Stopwatch.StartNew();
sum = 0;
for (long i = 0; i < iterations; i++)
{
    sum += vector[(int)(i % vectorLen)];
}
GC.KeepAlive(sum);
Report("Vector", sw);

// IterateVector: a foreach over the whole collection, repeated a bounded number of passes. CsWinRT
// projects IVector<int> as IList<int>; foreach obtains an IEnumerator<int> RCW per pass and
// marshals each element across the ABI, so this exposes the per-pass enumerator allocation and
// per-element marshalling the direct-vtable consumers avoid.
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
DrainFinalizers();

// GetMany: copy a vector sized to the requested iteration count into one caller-owned buffer.
// Construction and allocation happen before the timer so this isolates the bulk ABI call.
uint bulkCount = (uint)Math.Min(iterations, int.MaxValue);
var bulkVector = widget.Items(bulkCount);
int[] buffer = new int[bulkCount];
sw = Stopwatch.StartNew();
bulkVector.CopyTo(buffer, 0);
GC.KeepAlive(buffer);
Report("GetMany", sw);

const uint mapLen = 1024;

// Map: enumerate an IMap<string,int>. Repeat a bounded 1024-entry map enough times to keep total
// entry visits near the requested count.
var stringMap = widget.StringMap(mapLen);
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
DrainFinalizers();

// Lookup: read values from a projected generic dictionary IMap<int,int> by key. CsWinRT projects
// IMap<int,int> as IDictionary<int,int>; each lookup crosses the ABI through the RCW, so this
// exposes the per-lookup cost the struct-over-pointer consumers avoid.
var map = widget.Map(mapLen);
sw = Stopwatch.StartNew();
sum = 0;
for (long i = 0; i < iterations; i++)
{
    sum += map[(int)(i % mapLen)];
}
GC.KeepAlive(sum);
Report("Lookup", sw);

// VectorView: read elements from the read-only view. CsWinRT projects IVectorView<int> as
// IReadOnlyList<int>; each element read crosses the ABI through the RCW.
var vectorView = widget.ItemsView(vectorLen);
sw = Stopwatch.StartNew();
sum = 0;
for (long i = 0; i < iterations; i++)
{
    sum += vectorView[(int)(i % vectorLen)];
}
GC.KeepAlive(sum);
Report("VectorView", sw);

// MapView: read values from the read-only view by key. CsWinRT projects IMapView<int,int> as
// IReadOnlyDictionary<int,int>; each lookup crosses the ABI through the RCW.
var mapView = widget.MapView(mapLen);
sw = Stopwatch.StartNew();
sum = 0;
for (long i = 0; i < iterations; i++)
{
    sum += mapView[(int)(i % mapLen)];
}
GC.KeepAlive(sum);
Report("MapView", sw);

// Reference: box a nullable int input as IReference<int> and unbox the returned value.
sw = Stopwatch.StartNew();
for (long i = 0; i < iterations; i++)
{
    widget.ReferenceProperty = 0;
    _ = widget.ReferenceProperty!.Value;
}
Report("Reference", sw);
DrainFinalizers();

// Async: obtain an already-completed IAsyncOperation<int> and synchronously read its result.
sw = Stopwatch.StartNew();
for (long i = 0; i < iterations; i++)
{
    _ = widget.Operation().GetAwaiter().GetResult();
}
Report("Async", sw);
DrainFinalizers();

// Error: call a method that always returns a failing HRESULT. CsWinRT throws on the failed check,
// and the caller catches it. Throwing and catching a managed exception costs orders of magnitude
// more than the scalar calls above, so this loop runs a reduced count.
long failIterations = Math.Min(iterations, 1_000_000);
sw = Stopwatch.StartNew();
long errors = 0;
for (long i = 0; i < failIterations; i++)
{
    try { widget.Fail(); } catch (Exception) { errors++; }
}
GC.KeepAlive(errors);
Report("Error", sw);

// Leak check: activate and use N objects, then confirm the component's live instance count
// returns to the baseline. Unlike Rust and C++/WinRT, which release deterministically at scope
// exit, CsWinRT's RCWs hold their native reference until the GC finalizes them, so the count only
// returns to baseline after a forced collection. This is not a leak, but it does mean native
// objects outlive their last managed use by an unbounded interval.
// The baseline is read after a forced collection so earlier dead-but-unfinalized RCWs (from the
// Create/String/Cast loops above) do not inflate it.
GC.Collect();
GC.WaitForPendingFinalizers();
GC.Collect();
int baseline = widget.LiveCount();
ChurnWidgets(iterations);
GC.Collect();
GC.WaitForPendingFinalizers();
GC.Collect();
Console.WriteLine($"Leak: {widget.LiveCount() - baseline}");

// The churn loop lives in its own method so its last-iteration local is out of scope (and

// Scalability: retain N live objects and report the managed heap cost per object. Each
// projected Widget is an RCW-backed managed object, so this is where the projection model's
// per-object overhead shows up against the single pointer held by Rust and C++/WinRT.
int live = (int)Math.Min(iterations, 1_000_000);
long before = GC.GetTotalAllocatedBytes(precise: true);
var widgets = new Widget[live];
for (int i = 0; i < live; i++)
{
    widgets[i] = new Widget();
}

long after = GC.GetTotalAllocatedBytes(precise: true);
long bytes = after - before;
Console.WriteLine($"Live-{live}: {bytes} bytes ({(double)bytes / live:F1} bytes/object)");
GC.KeepAlive(widgets);

static void Report(string label, Stopwatch sw) =>
    Console.WriteLine($"{label}: {sw.ElapsedMilliseconds} ms");

// RCWs release native references through finalization. Drain any remaining work after metrics that
// create transient projected objects so their backlog cannot contaminate the next timed region.
static void DrainFinalizers()
{
    GC.Collect();
    GC.WaitForPendingFinalizers();
    GC.Collect();
}

// The churn loop lives in its own method so its last-iteration local is out of scope (and
// therefore collectable) before the forced GC in the leak check -- under a Debug JIT locals are
// otherwise rooted to the end of the enclosing method, which would leave one object uncollected.
static void ChurnWidgets(long iterations)
{
    for (long i = 0; i < iterations; i++)
    {
        var scratch = new Widget();
        _ = scratch.As<INonDefault>().Value();
    }
}
