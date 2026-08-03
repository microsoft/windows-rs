using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Media;
using Windows.Foundation.Collections;
using Windows.UI;

internal static unsafe partial class Program
{
    private static int s_iterations = 100_000;
    private static int s_createIterations = 1_000;
    private static int s_treeIterations = 20;
    private static int s_treeSize = 100;
    private static int s_stressIterations = 100;
    private static int s_sustainedSeconds = 3;
    private static int s_sustainedPercent = 10;
    private static int s_sustainedChurn;
    private static bool s_headless;
    private static uint s_settleMs = 750;
    private static double s_mainMs;
    private static double s_hostStartMs;
    private static string? s_resultPrefix;
    private static long s_sustainedStart;
    private static long s_sustainedUpdateNs;
    private static long s_sustainedAllocationStart;
    private static int s_sustainedTicks;
    private static int s_sustainedFrames;
    private static bool s_sustainedAllocationStarted;
    private static Exception? s_sustainedError;
    private static Window? s_visibleWindow;
    private static StressFixture? s_visibleStress;
    private static UIElement? s_visibleStressContent;

    [LibraryImport("microsoft.windowsappruntime.bootstrap.dll")]
    private static partial int MddBootstrapInitialize2(
        uint majorMinorVersion,
        char* versionTag,
        ulong minVersion,
        int options);

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
        Check(MddBootstrapInitialize2(0x0002_0000, null, 0x0002_0000_0001_0000, 24));
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
            using TextBlock text = new() { Text = "windows-csharp" };
            using TreeFixture retained = TreeFixture.Create(s_treeSize);
            using UIElement content = retained.Root.As<UIElement>();
            Window window = new()
            {
                Title = "windows-csharp WinUI benchmark",
                Content = content,
            };
            if (!s_headless)
            {
                window.Activate();
            }
            double windowMs = ElapsedMs();

            Metric create = Measure(s_createIterations, static _ =>
            {
                using TextBlock item = new() { Text = "item" };
            });

            Metric update = MeasureUpdate(text, s_iterations);
            Metric cast = MeasureCast(text, s_createIterations);
            Metric tree = MeasureTree(s_treeIterations, s_treeSize);
            Metric batchUpdate = MeasureBatchUpdate(retained, s_iterations);
            Metric churn = MeasureChurn(retained, s_createIterations);
            Metric teardown = MeasureTeardown(s_treeIterations, s_treeSize);
            using Button button = new();
            using ButtonBase buttonBase = button.As<ButtonBase>();
            using Control control = button.As<Control>();
            using RoutedEventHandler handler = RoutedEventHandler.Create(static (_, _) => { });
            Metric eventAddRemove = MeasureEvent(buttonBase, handler, s_createIterations);
            Metric boolean = MeasureBoolean(control, s_iterations);

            VerifyTree(retained.Root, s_treeSize);
            DrainFinalizers();
            long workingSet = Process.GetCurrentProcess().WorkingSet64;
            using (StressFixture warm = StressFixture.Create(1))
            {
            }
            long stressBefore = GC.GetAllocatedBytesForCurrentThread();
            long stressStart = Stopwatch.GetTimestamp();
            StressFixture stress = StressFixture.Create(4_900);
            Metric stressBuild = new(
                Stopwatch.GetElapsedTime(stressStart).TotalNanoseconds,
                GC.GetAllocatedBytesForCurrentThread() - stressBefore);
            UIElement stressContent = stress.Root.As<UIElement>();
            window.Content = stressContent;
            Metric stress0 = MeasureStress(stress, 0, s_stressIterations);
            Metric stress10 = MeasureStress(stress, 10, s_stressIterations);
            Metric stress50 = MeasureStress(stress, 50, s_stressIterations);
            Metric stress100 = MeasureStress(stress, 100, s_stressIterations);
            long stressWorkingSet = Process.GetCurrentProcess().WorkingSet64;
            StressFixture sustainedStress = StressFixture.Create(4_900);
            UIElement sustainedContent = sustainedStress.Root.As<UIElement>();
            window.Content = sustainedContent;
            stressContent.Dispose();
            stress.Dispose();
            s_resultPrefix =
                $"WINUI_BENCH_JSON {{\"consumer\":\"windows-csharp\",\"mainMs\":{s_mainMs:F3}," +
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
            s_visibleWindow = window;
            s_visibleStress = sustainedStress;
            s_visibleStressContent = sustainedContent;
            s_sustainedStart = Stopwatch.GetTimestamp();
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
            if (!s_sustainedAllocationStarted)
            {
                s_sustainedAllocationStart = GC.GetAllocatedBytesForCurrentThread();
                s_sustainedAllocationStarted = true;
            }
            StressFixture stress = s_visibleStress!;
            int count = Math.Max(1, stress.Text.Count * s_sustainedPercent / 100);
            long start = Stopwatch.GetTimestamp();
            stress.UpdateSustained(s_sustainedTicks, count);
            stress.ReattachTail(s_sustainedChurn);
            s_sustainedUpdateNs +=
                (long)Stopwatch.GetElapsedTime(start).TotalNanoseconds;
            s_sustainedTicks++;
        }
        catch (Exception error)
        {
            s_sustainedError = error;
        }
    }

    [UnmanagedCallersOnly]
    private static void OnFrame()
    {
        s_sustainedFrames++;
    }

    [UnmanagedCallersOnly]
    private static void OnFinished()
    {
        try
        {
            if (s_sustainedError is not null)
            {
                Console.Error.WriteLine(s_sustainedError);
                return;
            }
            double seconds = Stopwatch.GetElapsedTime(s_sustainedStart).TotalSeconds;
            double updateMs = s_sustainedTicks == 0
                ? 0
                : s_sustainedUpdateNs / 1_000_000.0 / s_sustainedTicks;
            double updateBytes = s_sustainedTicks == 0 || !s_sustainedAllocationStarted
                ? 0
                : (double)(GC.GetAllocatedBytesForCurrentThread() -
                    s_sustainedAllocationStart) / s_sustainedTicks;
            double fps = seconds == 0 ? 0 : s_sustainedFrames / seconds;
            long workingSet = Process.GetCurrentProcess().WorkingSet64;
            Console.WriteLine(
                $"{s_resultPrefix},\"sustainedTicks\":{s_sustainedTicks}," +
                $"\"sustainedUpdateMs\":{updateMs:F3}," +
                $"\"sustainedUpdateBytes\":{updateBytes:F3}," +
                $"\"sustainedFrames\":{s_sustainedFrames}," +
                $"\"sustainedFps\":{fps:F3}," +
                $"\"sustainedChurn\":{s_sustainedChurn}," +
                $"\"sustainedWorkingSet\":{workingSet}}}");
            if (s_headless)
            {
                s_visibleWindow?.Dispose();
                s_visibleStressContent?.Dispose();
                s_visibleStress?.Dispose();
                s_visibleWindow = null;
                s_visibleStressContent = null;
                s_visibleStress = null;
            }
        }
        catch (Exception error)
        {
            Console.Error.WriteLine(error);
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

    private static Metric MeasureUpdate(TextBlock text, int iterations)
    {
        Metric result = default;
        text.Borrow(borrowed =>
        {
            for (int i = 0; i < Math.Min(iterations, 100); i++)
            {
                borrowed.Text = (i & 1) == 0 ? "even" : "odd";
            }

            DrainFinalizers();
            long before = GC.GetAllocatedBytesForCurrentThread();
            long start = Stopwatch.GetTimestamp();
            for (int i = 0; i < iterations; i++)
            {
                borrowed.Text = (i & 1) == 0 ? "even" : "odd";
            }
            TimeSpan elapsed = Stopwatch.GetElapsedTime(start);
            long bytes = GC.GetAllocatedBytesForCurrentThread() - before;
            result = new Metric(
                elapsed.TotalNanoseconds / iterations,
                (double)bytes / iterations);
        });
        return result;
    }

    private static Metric MeasureCast(TextBlock text, int iterations)
    {
        Metric result = default;
        text.Borrow(borrowed =>
        {
            for (int i = 0; i < Math.Min(iterations, 100); i++)
            {
                using UIElement item = borrowed.As<UIElement>();
            }

            DrainFinalizers();
            long before = GC.GetAllocatedBytesForCurrentThread();
            long start = Stopwatch.GetTimestamp();
            for (int i = 0; i < iterations; i++)
            {
                using UIElement item = borrowed.As<UIElement>();
            }
            TimeSpan elapsed = Stopwatch.GetElapsedTime(start);
            long bytes = GC.GetAllocatedBytesForCurrentThread() - before;
            result = new Metric(
                elapsed.TotalNanoseconds / iterations,
                (double)bytes / iterations);
        });
        return result;
    }

    private static Metric MeasureBatchUpdate(TreeFixture tree, int targetUpdates)
    {
        int iterations = Math.Max(1, targetUpdates / tree.Text.Count);
        for (int i = 0; i < Math.Min(iterations, 10); i++)
        {
            tree.Update((i & 1) == 0 ? "even" : "odd");
        }

        DrainFinalizers();
        long before = GC.GetAllocatedBytesForCurrentThread();
        long start = Stopwatch.GetTimestamp();
        for (int i = 0; i < iterations; i++)
        {
            tree.Update((i & 1) == 0 ? "even" : "odd");
        }
        TimeSpan elapsed = Stopwatch.GetElapsedTime(start);
        double operations = (double)iterations * tree.Text.Count;
        return new Metric(
            elapsed.TotalNanoseconds / operations,
            (GC.GetAllocatedBytesForCurrentThread() - before) / operations);
    }

    private static Metric MeasureChurn(TreeFixture tree, int targetOperations)
    {
        int iterations = Math.Max(1, targetOperations / tree.Children.Count);
        for (int i = 0; i < Math.Min(iterations, 10); i++)
        {
            tree.Reattach();
        }

        DrainFinalizers();
        long before = GC.GetAllocatedBytesForCurrentThread();
        long start = Stopwatch.GetTimestamp();
        for (int i = 0; i < iterations; i++)
        {
            tree.Reattach();
        }
        TimeSpan elapsed = Stopwatch.GetElapsedTime(start);
        double operations = (double)iterations * tree.Children.Count;
        return new Metric(
            elapsed.TotalNanoseconds / operations,
            (GC.GetAllocatedBytesForCurrentThread() - before) / operations);
    }

    private static Metric MeasureTree(int iterations, int size)
    {
        for (int i = 0; i < Math.Min(iterations, 10); i++)
        {
            using TreeFixture tree = TreeFixture.Create(size);
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
        foreach (TreeFixture tree in trees)
        {
            tree.Dispose();
        }
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
            tree.Dispose();
        }
        TimeSpan elapsed = Stopwatch.GetElapsedTime(start);
        return new Metric(
            elapsed.TotalNanoseconds / iterations,
            (double)(GC.GetAllocatedBytesForCurrentThread() - before) / iterations);
    }

    private static Metric MeasureEvent(
        ButtonBase button,
        RoutedEventHandler handler,
        int iterations)
    {
        Metric result = default;
        button.Borrow(borrowed =>
        {
            for (int i = 0; i < Math.Min(iterations, 100); i++)
            {
                long token = borrowed.AddClick(handler);
                borrowed.RemoveClick(token);
            }

            DrainFinalizers();
            long before = GC.GetAllocatedBytesForCurrentThread();
            long start = Stopwatch.GetTimestamp();
            for (int i = 0; i < iterations; i++)
            {
                long token = borrowed.AddClick(handler);
                borrowed.RemoveClick(token);
            }
            TimeSpan elapsed = Stopwatch.GetElapsedTime(start);
            result = new Metric(
                elapsed.TotalNanoseconds / iterations,
                (double)(GC.GetAllocatedBytesForCurrentThread() - before) / iterations);
        });
        return result;
    }

    private static Metric MeasureBoolean(Control control, int iterations)
    {
        Metric result = default;
        control.Borrow(borrowed =>
        {
            for (int i = 0; i < Math.Min(iterations, 100); i++)
            {
                borrowed.IsEnabled = (i & 1) == 0;
            }

            DrainFinalizers();
            long before = GC.GetAllocatedBytesForCurrentThread();
            long start = Stopwatch.GetTimestamp();
            for (int i = 0; i < iterations; i++)
            {
                borrowed.IsEnabled = (i & 1) == 0;
            }
            TimeSpan elapsed = Stopwatch.GetElapsedTime(start);
            result = new Metric(
                elapsed.TotalNanoseconds / iterations,
                (double)(GC.GetAllocatedBytesForCurrentThread() - before) / iterations);
        });
        return result;
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

    private static void VerifyTree(StackPanel panel, int count)
    {
        using Panel parent = panel.As<Panel>();
        using UIElementCollection children = parent.Children!;
        using IVector<UIElement?> vector = children.As<IVector<UIElement?>>();
        if (vector.Count != count)
        {
            throw new InvalidOperationException("The projected child vector has the wrong size.");
        }

        using UIElement first = vector.GetAt(0)!;
        UIElement?[] batch = new UIElement?[Math.Min(count, 4)];
        uint actual = vector.GetMany(0, batch);
        for (int i = 0; i < actual; i++)
        {
            batch[i]!.Dispose();
        }

        int enumerated = 0;
        foreach (UIElement? child in vector)
        {
            child!.Dispose();
            enumerated++;
        }
        if (enumerated != count)
        {
            throw new InvalidOperationException("The projected child vector enumerated incorrectly.");
        }
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

    private static void DrainFinalizers()
    {
        GC.Collect();
        GC.WaitForPendingFinalizers();
        GC.Collect();
    }

    private static void Check(int hr)
    {
        if (hr < 0)
        {
            Marshal.ThrowExceptionForHR(hr);
        }
    }

    private readonly record struct Metric(double Ns, double Bytes);

    private sealed class StressFixture : IDisposable
    {
        private const int Columns = 70;
        private const double CellWidth = 64;
        private const double CellHeight = 18;

        internal ScrollViewer Root { get; }
        internal List<TextBlock> Text { get; }
        private readonly Canvas _canvas;
        private readonly ContentControl _content;
        private readonly Panel _panel;
        private readonly UIElementCollection _collection;
        private readonly IVector<UIElement?> _vector;
        private readonly Brush _red;
        private readonly Brush _green;
        private bool _disposed;

        private StressFixture(
            ScrollViewer root,
            Canvas canvas,
            ContentControl content,
            Panel panel,
            UIElementCollection collection,
            IVector<UIElement?> vector,
            Brush red,
            Brush green,
            List<TextBlock> text)
        {
            Root = root;
            _canvas = canvas;
            _content = content;
            _panel = panel;
            _collection = collection;
            _vector = vector;
            _red = red;
            _green = green;
            Text = text;
        }

        internal static StressFixture Create(int count)
        {
            ScrollViewer root = new();
            Canvas canvas = new();
            ContentControl content = root.As<ContentControl>();
            Panel panel = canvas.As<Panel>();
            UIElementCollection collection = panel.Children!;
            IVector<UIElement?> vector = collection.As<IVector<UIElement?>>();
            List<TextBlock> text = new(count);
            Brush? redBrush = null;
            Brush? greenBrush = null;
            try
            {
                using (FrameworkElement canvasElement = canvas.As<FrameworkElement>())
                {
                    canvasElement.Width = Columns * CellWidth;
                    canvasElement.Height = ((count + Columns - 1) / Columns) * CellHeight;
                }
                using SolidColorBrush red = new()
                {
                    Color = new Color { A = 255, R = 220, G = 60, B = 60 },
                };
                using SolidColorBrush green = new()
                {
                    Color = new Color { A = 255, R = 70, G = 210, B = 100 },
                };
                redBrush = red.As<Brush>();
                greenBrush = green.As<Brush>();
                for (int i = 0; i < count; i++)
                {
                    TextBlock item = new()
                    {
                        Text = $"Item {i}",
                        FontSize = 12,
                        Foreground = (i & 1) == 0 ? greenBrush : redBrush,
                    };
                    text.Add(item);
                    item.BorrowAs(static (FrameworkElement.Borrowed element) =>
                    {
                        element.Width = CellWidth;
                        element.Height = CellHeight;
                    });
                    Canvas.SetLeft(item, (i % Columns) * CellWidth);
                    Canvas.SetTop(item, (i / Columns) * CellHeight);
                    vector.Append(item);
                }
                content.Content = canvas;
                return new StressFixture(
                    root,
                    canvas,
                    content,
                    panel,
                    collection,
                    vector,
                    redBrush,
                    greenBrush,
                    text);
            }
            catch
            {
                foreach (TextBlock item in text)
                {
                    item.Dispose();
                }
                vector.Dispose();
                collection.Dispose();
                panel.Dispose();
                content.Dispose();
                redBrush?.Dispose();
                greenBrush?.Dispose();
                canvas.Dispose();
                root.Dispose();
                throw;
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

        internal void UpdateSustained(int render, int count)
        {
            string value = (render & 1) == 0 ? "even" : "odd";
            int start = render * 97 % Text.Count;
            for (int i = 0; i < count; i++)
            {
                TextBlock item = Text[(start + i * 17) % Text.Count];
                item.Text = value;
                item.Foreground = ((render + i) & 1) == 0 ? _green : _red;
            }
        }

        internal void ReattachTail(int count)
        {
            count = Math.Min(count, Text.Count);
            for (int i = 0; i < count; i++)
            {
                _vector.RemoveAtEnd();
            }
            for (int i = Text.Count - count; i < Text.Count; i++)
            {
                _vector.Append(Text[i]);
            }
        }

        public void Dispose()
        {
            if (_disposed)
            {
                return;
            }
            _disposed = true;
            _content.Content = null;
            _vector.Clear();
            foreach (TextBlock item in Text)
            {
                item.Dispose();
            }
            _vector.Dispose();
            _collection.Dispose();
            _panel.Dispose();
            _content.Dispose();
            _red.Dispose();
            _green.Dispose();
            _canvas.Dispose();
            Root.Dispose();
        }
    }

    private sealed class TreeFixture : IDisposable
    {
        internal StackPanel Root { get; }
        internal List<TextBlock> Text { get; }
        internal List<TextBlock> Children => Text;
        private readonly Panel _panel;
        private readonly UIElementCollection _collection;
        private readonly IVector<UIElement?> _vector;
        private bool _disposed;

        private TreeFixture(
            StackPanel root,
            List<TextBlock> text,
            Panel panel,
            UIElementCollection collection,
            IVector<UIElement?> vector)
        {
            Root = root;
            Text = text;
            _panel = panel;
            _collection = collection;
            _vector = vector;
        }

        internal static TreeFixture Create(int count)
        {
            StackPanel root = new();
            Panel panel = root.As<Panel>();
            UIElementCollection collection = panel.Children!;
            IVector<UIElement?> vector = collection.As<IVector<UIElement?>>();
            List<TextBlock> text = new(count);
            try
            {
                for (int i = 0; i < count; i++)
                {
                    TextBlock item = new() { Text = $"Item {i}" };
                    text.Add(item);
                    vector.Append(item);
                }
                return new TreeFixture(root, text, panel, collection, vector);
            }
            catch
            {
                foreach (TextBlock item in text)
                {
                    item.Dispose();
                }
                vector.Dispose();
                collection.Dispose();
                panel.Dispose();
                root.Dispose();
                throw;
            }
        }

        internal void Update(string value)
        {
            foreach (TextBlock item in Text)
            {
                item.Text = value;
            }
        }

        internal void Reattach()
        {
            _vector.Clear();
            foreach (TextBlock child in Children)
            {
                _vector.Append(child);
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

        public void Dispose()
        {
            if (_disposed)
            {
                return;
            }
            _disposed = true;
            _vector.Clear();
            foreach (TextBlock item in Text)
            {
                item.Dispose();
            }
            _vector.Dispose();
            _collection.Dispose();
            _panel.Dispose();
            Root.Dispose();
        }
    }
}
