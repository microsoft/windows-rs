using System.Collections.Generic;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Windows.UI;

internal static unsafe partial class Program
{
    private static int s_iterations = 100_000;
    private static int s_createIterations = 1_000;
    private static int s_treeIterations = 20;
    private static int s_treeSize = 100;
    private static int s_stressIterations = 100;
    private static bool s_headless;
    private static uint s_settleMs = 750;
    private static int s_sustainedSeconds = 3;
    private static int s_sustainedPercent = 10;
    private static int s_sustainedChurn;
    private static double s_mainMs;
    private static double s_hostStartMs;
    private static Window? s_window;
    private static UIElement? s_treeContent;
    private static StressFixture? s_stress;
    private static Window? s_visibleWindow;
    private static StressFixture? s_visibleStress;
    private static string s_jsonPrefix = "";
    private static long s_sustainedStart;
    private static long s_sustainedTicks;
    private static long s_sustainedFrames;
    private static double s_sustainedTotalNs;
    private static long? s_sustainedAllocBaseline;
    private static Exception? s_sustainedError;

    [LibraryImport("test_winui_bench_host.dll")]
    private static partial int run(
        delegate* unmanaged<int> launch,
        delegate* unmanaged<void> tick,
        delegate* unmanaged<void> frame,
        delegate* unmanaged<void> finish,
        uint durationMs,
        uint settleMs);

    [STAThread]
    private static void Main(string[] args)
    {
        s_mainMs = ElapsedMs();
        Parse(args);
        s_hostStartMs = ElapsedMs();
        Check(run(
            &OnLaunched,
            &OnTick,
            &OnFrame,
            &OnFinished,
            checked((uint)s_sustainedSeconds * 1_000),
            s_headless ? 0 : s_settleMs));
    }

    [UnmanagedCallersOnly]
    private static int OnLaunched()
    {
        try
        {
            double startupMs = ElapsedMs();
            TextBlock text = new() { Text = "CsWinRT" };
            TreeFixture retained = TreeFixture.Create(s_treeSize);
            UIElement content = retained.Root;
            Window window = new()
            {
                Title = "CsWinRT WinUI benchmark",
                Content = content,
            };
            if (!s_headless)
            {
                window.Activate();
            }
            double windowMs = ElapsedMs();

            Metric create = Measure(s_createIterations, static _ =>
            {
                TextBlock item = new() { Text = "item" };
                GC.KeepAlive(item);
            });
            DrainFinalizers();

            Metric update = Measure(
                s_iterations,
                i => text.Text = (i & 1) == 0 ? "even" : "odd");

            Metric cast = Measure(s_createIterations, _ =>
            {
                UIElement item = text;
                GC.KeepAlive(item);
            });
            Metric tree = MeasureTree(s_treeIterations, s_treeSize);
            Metric batchUpdate = MeasureBatchUpdate(retained, s_iterations);
            Metric churn = MeasureChurn(retained, s_createIterations);
            Metric teardown = MeasureTeardown(s_treeIterations, s_treeSize);
            Button button = new();
            RoutedEventHandler handler = static (_, _) => { };
            Metric eventAddRemove = Measure(s_createIterations, _ =>
            {
                button.Click += handler;
                button.Click -= handler;
            });
            Metric boolean = Measure(
                s_iterations,
                i => button.IsEnabled = (i & 1) == 0);

            DrainFinalizers();
            long workingSet = Process.GetCurrentProcess().WorkingSet64;
            StressFixture warm = StressFixture.Create(1);
            warm.Clear();
            warm = null!;
            DrainFinalizers();
            long stressBefore = GC.GetAllocatedBytesForCurrentThread();
            long stressStart = Stopwatch.GetTimestamp();
            StressFixture stress = StressFixture.Create(4_900);
            Metric stressBuild = new(
                Stopwatch.GetElapsedTime(stressStart).TotalNanoseconds,
                GC.GetAllocatedBytesForCurrentThread() - stressBefore);
            window.Content = stress.Root;
            Metric stress0 = MeasureStress(stress, 0, s_stressIterations);
            Metric stress10 = MeasureStress(stress, 10, s_stressIterations);
            Metric stress50 = MeasureStress(stress, 50, s_stressIterations);
            Metric stress100 = MeasureStress(stress, 100, s_stressIterations);
            long stressWorkingSet = Process.GetCurrentProcess().WorkingSet64;
            stress.Clear();
            stress = null!;
            DrainFinalizers();
            StressFixture sustainedStress = StressFixture.Create(4_900);
            window.Content = sustainedStress.Root;
            s_jsonPrefix =
                $"WINUI_BENCH_JSON {{\"consumer\":\"cswinrt\",\"mainMs\":{s_mainMs:F3}," +
                $"\"hostStartMs\":{s_hostStartMs:F3},\"startupMs\":{startupMs:F3}," +
                $"\"windowMs\":{windowMs:F3},\"workingSet\":{workingSet}," +
                $"\"createNs\":{create.Ns:F3},\"createBytes\":{create.Bytes:F3}," +
                $"\"updateNs\":{update.Ns:F3},\"updateBytes\":{update.Bytes:F3}," +
                $"\"castNs\":{cast.Ns:F3},\"castBytes\":{cast.Bytes:F3}," +
                $"\"treeNs\":{tree.Ns:F3},\"treeBytes\":{tree.Bytes:F3}," +
                $"\"batchUpdateNs\":{batchUpdate.Ns:F3}," +
                $"\"batchUpdateBytes\":{batchUpdate.Bytes:F3}," +
                $"\"churnNs\":{churn.Ns:F3},\"churnBytes\":{churn.Bytes:F3}," +
                $"\"teardownNs\":{teardown.Ns:F3},\"teardownBytes\":{teardown.Bytes:F3}," +
                $"\"eventNs\":{eventAddRemove.Ns:F3}," +
                $"\"eventBytes\":{eventAddRemove.Bytes:F3}," +
                $"\"booleanNs\":{boolean.Ns:F3},\"booleanBytes\":{boolean.Bytes:F3}," +
                $"\"stressBuildNs\":{stressBuild.Ns:F3}," +
                $"\"stressBuildBytes\":{stressBuild.Bytes:F3}," +
                $"\"stress0Ms\":{stress0.Ns / 1_000_000:F3}," +
                $"\"stress0Bytes\":{stress0.Bytes:F3}," +
                $"\"stress10Ms\":{stress10.Ns / 1_000_000:F3}," +
                $"\"stress10Bytes\":{stress10.Bytes:F3}," +
                $"\"stress50Ms\":{stress50.Ns / 1_000_000:F3}," +
                $"\"stress50Bytes\":{stress50.Bytes:F3}," +
                $"\"stress100Ms\":{stress100.Ns / 1_000_000:F3}," +
                $"\"stress100Bytes\":{stress100.Bytes:F3}," +
                $"\"stressWorkingSet\":{stressWorkingSet}";
            s_window = window;
            s_treeContent = content;
            s_stress = sustainedStress;
            s_sustainedStart = Stopwatch.GetTimestamp();
            s_sustainedTicks = 0;
            s_sustainedFrames = 0;
            s_sustainedTotalNs = 0;
            s_sustainedAllocBaseline = null;
            GC.KeepAlive(window);
            return 0;
        }
        catch (Exception error)
        {
            Console.Error.WriteLine(error);
            return error.HResult;
        }
    }

    [UnmanagedCallersOnly]
    private static void OnTick()
    {
        if (s_sustainedError is not null)
        {
            return;
        }
        try
        {
            if (s_sustainedAllocBaseline is null)
            {
                s_sustainedAllocBaseline = GC.GetAllocatedBytesForCurrentThread();
            }
            int count = Math.Max(1, s_stress!.Text.Count * s_sustainedPercent / 100);
            long start = Stopwatch.GetTimestamp();
            s_stress.UpdateSustained((int)s_sustainedTicks, count);
            s_stress.ReattachTail(s_sustainedChurn);
            s_sustainedTotalNs += Stopwatch.GetElapsedTime(start).TotalNanoseconds;
            s_sustainedTicks++;
        }
        catch (Exception ex)
        {
            s_sustainedError ??= ex;
        }
    }

    [UnmanagedCallersOnly]
    private static void OnFrame()
    {
        try
        {
            s_sustainedFrames++;
        }
        catch (Exception ex)
        {
            s_sustainedError ??= ex;
        }
    }

    [UnmanagedCallersOnly]
    private static void OnFinished()
    {
        try
        {
            if (s_sustainedError != null)
            {
                Console.Error.WriteLine(s_sustainedError);
                return;
            }
            double elapsedSec = Stopwatch.GetElapsedTime(s_sustainedStart).TotalSeconds;
            double avgUpdateMs = s_sustainedTicks > 0
                ? s_sustainedTotalNs / s_sustainedTicks / 1_000_000.0
                : 0.0;
            long allocEnd = GC.GetAllocatedBytesForCurrentThread();
            long allocBaseline = s_sustainedAllocBaseline ?? allocEnd;
            double bytesPerTick = s_sustainedTicks > 0
                ? (double)(allocEnd - allocBaseline) / s_sustainedTicks
                : 0.0;
            double fps = elapsedSec > 0 ? s_sustainedFrames / elapsedSec : 0.0;
            long sustainedWorkingSet = Process.GetCurrentProcess().WorkingSet64;
            Console.WriteLine(
                s_jsonPrefix +
                $",\"sustainedTicks\":{s_sustainedTicks}" +
                $",\"sustainedUpdateMs\":{avgUpdateMs:F3}" +
                $",\"sustainedUpdateBytes\":{bytesPerTick:F3}" +
                $",\"sustainedFrames\":{s_sustainedFrames}" +
                $",\"sustainedFps\":{fps:F3}" +
                $",\"sustainedChurn\":{s_sustainedChurn}" +
                $",\"sustainedWorkingSet\":{sustainedWorkingSet}}}");
            if (s_headless)
            {
                s_window!.Content = s_treeContent;
                s_stress!.Clear();
            }
            else
            {
                s_visibleWindow = s_window;
                s_visibleStress = s_stress;
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine(ex);
        }
    }

    private static Metric Measure(int iterations, Action<int> action)
    {
        for (int i = 0; i < Math.Min(iterations, 100); i++)
        {
            action(i);
        }

        DrainFinalizers();
        long before = GC.GetAllocatedBytesForCurrentThread();
        long start = Stopwatch.GetTimestamp();
        for (int i = 0; i < iterations; i++)
        {
            action(i);
        }
        TimeSpan elapsed = Stopwatch.GetElapsedTime(start);
        long bytes = GC.GetAllocatedBytesForCurrentThread() - before;
        return new Metric(
            elapsed.TotalNanoseconds / iterations,
            (double)bytes / iterations);
    }

    private static void DrainFinalizers()
    {
        GC.Collect();
        GC.WaitForPendingFinalizers();
        GC.Collect();
    }

    private static Metric MeasureBatchUpdate(TreeFixture tree, int targetUpdates)
    {
        int iterations = Math.Max(1, targetUpdates / tree.Text.Count);
        return MeasureOperations(iterations, tree.Text.Count, i =>
        {
            string value = (i & 1) == 0 ? "even" : "odd";
            foreach (TextBlock item in tree.Text)
            {
                item.Text = value;
            }
        });
    }

    private static Metric MeasureChurn(TreeFixture tree, int targetOperations)
    {
        int iterations = Math.Max(1, targetOperations / tree.Children.Count);
        return MeasureOperations(iterations, tree.Children.Count, _ => tree.Reattach());
    }

    private static Metric MeasureOperations(
        int iterations,
        int operationsPerIteration,
        Action<int> action)
    {
        for (int i = 0; i < Math.Min(iterations, 10); i++)
        {
            action(i);
        }

        DrainFinalizers();
        long before = GC.GetAllocatedBytesForCurrentThread();
        long start = Stopwatch.GetTimestamp();
        for (int i = 0; i < iterations; i++)
        {
            action(i);
        }
        TimeSpan elapsed = Stopwatch.GetElapsedTime(start);
        double operations = (double)iterations * operationsPerIteration;
        return new Metric(
            elapsed.TotalNanoseconds / operations,
            (GC.GetAllocatedBytesForCurrentThread() - before) / operations);
    }

    private static Metric MeasureStress(StressFixture tree, int percent, int iterations)
    {
        int count = Math.Max(1, tree.Text.Count * percent / 100);
        for (int render = 0; render < Math.Min(iterations, 3); render++)
        {
            tree.UpdateIndices(render, count);
        }

        DrainFinalizers();
        long before = GC.GetAllocatedBytesForCurrentThread();
        long start = Stopwatch.GetTimestamp();
        for (int render = 0; render < iterations; render++)
        {
            tree.UpdateIndices(render, count);
        }
        return new Metric(
            Stopwatch.GetElapsedTime(start).TotalNanoseconds / iterations,
            (double)(GC.GetAllocatedBytesForCurrentThread() - before) / iterations);
    }

    private static Metric MeasureTree(int iterations, int size)
    {
        for (int i = 0; i < Math.Min(iterations, 10); i++)
        {
            GC.KeepAlive(TreeFixture.Create(size));
        }

        DrainFinalizers();
        List<TreeFixture> trees = new(iterations);
        long before = GC.GetAllocatedBytesForCurrentThread();
        long start = Stopwatch.GetTimestamp();
        for (int i = 0; i < iterations; i++)
        {
            trees.Add(TreeFixture.Create(size));
        }
        TimeSpan elapsed = Stopwatch.GetElapsedTime(start);
        long bytes = GC.GetAllocatedBytesForCurrentThread() - before;
        trees.Clear();
        DrainFinalizers();
        return new Metric(
            elapsed.TotalNanoseconds / iterations,
            (double)bytes / iterations);
    }

    private static Metric MeasureTeardown(int iterations, int size)
    {
        List<TreeFixture> trees = new(iterations);
        for (int i = 0; i < iterations; i++)
        {
            trees.Add(TreeFixture.Create(size));
        }

        DrainFinalizers();
        long before = GC.GetAllocatedBytesForCurrentThread();
        long start = Stopwatch.GetTimestamp();
        foreach (TreeFixture tree in trees)
        {
            tree.Clear();
        }
        trees.Clear();
        DrainFinalizers();
        TimeSpan elapsed = Stopwatch.GetElapsedTime(start);
        return new Metric(
            elapsed.TotalNanoseconds / iterations,
            (double)(GC.GetAllocatedBytesForCurrentThread() - before) / iterations);
    }

    private static void Parse(string[] args)
    {
        s_headless = Array.IndexOf(args, "--headless") >= 0;
        for (int i = 0; i + 1 < args.Length; i++)
        {
            if (args[i] == "--iterations")
            {
                s_iterations = int.Parse(args[++i]);
            }
            else if (args[i] == "--create-iterations")
            {
                s_createIterations = int.Parse(args[++i]);
            }
            else if (args[i] == "--settle-ms")
            {
                s_settleMs = uint.Parse(args[++i]);
            }
            else if (args[i] == "--tree-iterations")
            {
                s_treeIterations = int.Parse(args[++i]);
            }
            else if (args[i] == "--tree-size")
            {
                s_treeSize = int.Parse(args[++i]);
            }
            else if (args[i] == "--stress-iterations")
            {
                s_stressIterations = Math.Max(1, int.Parse(args[++i]));
            }
            else if (args[i] == "--sustained-seconds")
            {
                s_sustainedSeconds = Math.Max(1, int.Parse(args[++i]));
            }
            else if (args[i] == "--sustained-percent")
            {
                s_sustainedPercent = Math.Clamp(int.Parse(args[++i]), 0, 100);
            }
            else if (args[i] == "--sustained-churn")
            {
                s_sustainedChurn = Math.Clamp(int.Parse(args[++i]), 0, 4_900);
            }
        }
    }

    private static double ElapsedMs() =>
        (DateTime.UtcNow - Process.GetCurrentProcess().StartTime.ToUniversalTime()).TotalMilliseconds;

    private static void Check(int hr)
    {
        if (hr < 0)
        {
            Marshal.ThrowExceptionForHR(hr);
        }
    }

    private readonly record struct Metric(double Ns, double Bytes);

    private sealed class StressFixture
    {
        private const int Columns = 70;
        private const double CellWidth = 64;
        private const double CellHeight = 18;

        internal ScrollViewer Root { get; }
        internal List<TextBlock> Text { get; }
        private readonly Canvas _canvas;
        private readonly IList<UIElement> _children;
        private readonly SolidColorBrush _brushRed;
        private readonly SolidColorBrush _brushGreen;

        private StressFixture(
            ScrollViewer root,
            Canvas canvas,
            List<TextBlock> text,
            IList<UIElement> children,
            SolidColorBrush brushRed,
            SolidColorBrush brushGreen)
        {
            Root = root;
            _canvas = canvas;
            Text = text;
            _children = children;
            _brushRed = brushRed;
            _brushGreen = brushGreen;
        }

        internal static StressFixture Create(int count)
        {
            Canvas canvas = new()
            {
                Width = Columns * CellWidth,
                Height = ((count + Columns - 1) / Columns) * CellHeight,
            };
            IList<UIElement> children = canvas.Children;
            List<TextBlock> text = new(count);
            SolidColorBrush red = new()
            {
                Color = Color.FromArgb(255, 220, 60, 60),
            };
            SolidColorBrush green = new()
            {
                Color = Color.FromArgb(255, 70, 210, 100),
            };
            for (int i = 0; i < count; i++)
            {
                TextBlock item = new()
                {
                    Text = $"Item {i}",
                    Width = CellWidth,
                    Height = CellHeight,
                    FontSize = 12,
                    Foreground = (i & 1) == 0 ? green : red,
                };
                Canvas.SetLeft(item, (i % Columns) * CellWidth);
                Canvas.SetTop(item, (i / Columns) * CellHeight);
                children.Add(item);
                text.Add(item);
            }
            return new StressFixture(
                new ScrollViewer { Content = canvas },
                canvas,
                text,
                children,
                red,
                green);
        }

        internal void UpdateIndices(int render, int count)
        {
            string value = (render & 1) == 0 ? "even" : "odd";
            int start = render * 97 % Text.Count;
            for (int i = 0; i < count; i++)
            {
                Text[(start + i * 17) % Text.Count].Text = value;
            }
        }

        internal void UpdateSustained(int render, int count)
        {
            string value = (render & 1) == 0 ? "even" : "odd";
            int start = render * 97 % Text.Count;
            for (int i = 0; i < count; i++)
            {
                TextBlock item = Text[(start + i * 17) % Text.Count];
                item.Text = value;
                item.Foreground = ((render + i) & 1) == 0 ? _brushGreen : _brushRed;
            }
        }

        internal void ReattachTail(int count)
        {
            count = Math.Min(count, Text.Count);
            for (int i = 0; i < count; i++)
            {
                _children.RemoveAt(_children.Count - 1);
            }
            for (int i = Text.Count - count; i < Text.Count; i++)
            {
                _children.Add(Text[i]);
            }
        }

        internal void Clear()
        {
            Root.Content = null;
            _children.Clear();
            Text.Clear();
            GC.KeepAlive(_canvas);
        }
    }

    private sealed class TreeFixture
    {
        internal StackPanel Root { get; }
        internal List<TextBlock> Text { get; }
        internal IList<UIElement> Children { get; }

        private TreeFixture(StackPanel root, List<TextBlock> text, IList<UIElement> children)
        {
            Root = root;
            Text = text;
            Children = children;
        }

        internal static TreeFixture Create(int count)
        {
            StackPanel root = new();
            IList<UIElement> children = root.Children;
            List<TextBlock> text = new(count);
            for (int i = 0; i < count; i++)
            {
                TextBlock item = new() { Text = $"Item {i}" };
                children.Add(item);
                text.Add(item);
            }
            return new TreeFixture(root, text, children);
        }

        internal void Reattach()
        {
            Children.Clear();
            foreach (TextBlock item in Text)
            {
                Children.Add(item);
            }
        }

        internal void UpdateIndices(int render, int count)
        {
            string value = (render & 1) == 0 ? "even" : "odd";
            int start = render * 97 % Text.Count;
            for (int i = 0; i < count; i++)
            {
                Text[(start + i * 17) % Text.Count].Text = value;
            }
        }

        internal void Clear()
        {
            Children.Clear();
            Text.Clear();
        }
    }
}
