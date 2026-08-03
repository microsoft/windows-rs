using System;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Threading;
using Windows.Foundation.Numerics;
using Windows.System;
using Windows.UI.Composition;
using Windows.UI.Composition.Desktop;
using Windows.Win32;
using static Windows.Win32.Apis;

namespace CSharpMinesweeper;

internal static unsafe class Program
{
    [STAThread]
    private static int Main(string[] args)
    {
        bool visibleSmoke = Array.IndexOf(args, "--smoke-visible") >= 0;
        bool smoke = visibleSmoke || Array.IndexOf(args, "--smoke") >= 0;
        long allocationStart = GC.GetAllocatedBytesForCurrentThread();
        long start = Stopwatch.GetTimestamp();

        try
        {
            using MinesweeperApplication application = new(smoke, !smoke || visibleSmoke);
            double startupMs = Stopwatch.GetElapsedTime(start).TotalMilliseconds;
            long startupBytes = GC.GetAllocatedBytesForCurrentThread() - allocationStart;
            application.Run();

            if (smoke)
            {
                Console.WriteLine(
                    $"MINESWEEPER_SMOKE startupMs={startupMs:F3} " +
                    $"startupBytes={startupBytes} " +
                    $"idlePumpBytes={application.IdlePumpBytes} " +
                    $"workingSet={application.IdleWorkingSet}");
            }
            return 0;
        }
        catch (Exception error)
        {
            Console.Error.WriteLine(error);
            return error.HResult;
        }
    }
}

internal sealed unsafe class MinesweeperApplication : IDisposable
{
    private const string WindowClass = "windows-csharp-minesweeper";
    private const uint WmClose = 0x0010;
    private const uint WmDestroy = 0x0002;
    private const uint WmSize = 0x0005;
    private const uint WmNull = 0x0000;
    private const uint WmMouseMove = 0x0200;
    private const uint WmLeftButtonDown = 0x0201;
    private const uint WmRightButtonDown = 0x0204;
    private const uint WmSmokeReady = 0x8001;

    private static MinesweeperApplication? s_current;

    private readonly IDispatcherQueueController _dispatcher;
    private readonly Compositor _compositor;
    private readonly ContainerVisual _root;
    private readonly IDesktopWindowTarget _desktopTarget;
    private readonly ICompositionTarget _compositionTarget;
    private readonly MinesweeperGame _game;
    private readonly bool _visibleSmoke;
    private HWND _window;
    private Exception? _messageError;
    private Timer? _smokeTimer;
    private bool _disposed;
    private long _idleAllocationStart;

    internal long IdlePumpBytes { get; private set; }
    internal long IdleWorkingSet { get; private set; }

    internal MinesweeperApplication(bool smoke, bool visible)
    {
        if (s_current is not null)
        {
            throw new InvalidOperationException("Only one sample window may exist.");
        }
        s_current = this;
        _visibleSmoke = smoke && visible;

        try
        {
            _dispatcher = CreateDispatcherQueueController(new DispatcherQueueOptions
            {
                dwSize = (uint)sizeof(DispatcherQueueOptions),
                threadType = DISPATCHERQUEUE_THREAD_TYPE.DQTYPE_THREAD_CURRENT,
                apartmentType = DISPATCHERQUEUE_THREAD_APARTMENTTYPE.DQTAT_COM_STA,
            });
            _compositor = new Compositor();
            _root = _compositor.CreateContainerVisual()!;
            _root.RelativeSizeAdjustment = new Vector2 { X = 1, Y = 1 };

            HINSTANCE instance = GetModuleHandleW(null);
            RegisterWindowClass(instance);
            uint style = unchecked((uint)WS_OVERLAPPEDWINDOW);
            if (visible)
            {
                style |= unchecked((uint)WS_VISIBLE);
            }

            _window = CreateWindowExW(
                0,
                WindowClass,
                "Minesweeper - windows-csharp",
                style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                800,
                600,
                default,
                default,
                instance,
                null);
            if (_window == default)
            {
                throw new InvalidOperationException("CreateWindowExW failed.");
            }

            using (ICompositorDesktopInterop interop =
                _compositor.As<ICompositorDesktopInterop>())
            {
                _desktopTarget = interop.CreateDesktopWindowTarget(_window, false);
            }
            _compositionTarget = _desktopTarget.As<ICompositionTarget>();
            using (Visual visual = _root.As<Visual>())
            {
                _compositionTarget.Root = visual;
            }

            Vector2 size = ClientSize();
            _game = new MinesweeperGame(_compositor, _root, size);

            if (smoke)
            {
                BeginSmoke();
            }
        }
        catch
        {
            Dispose();
            throw;
        }
    }

    internal void Run()
    {
        while (GetMessageW(out MSG message, default, 0, 0))
        {
            _ = TranslateMessage(&message);
            _ = DispatchMessageW(&message);
        }

        if (_messageError is not null)
        {
            throw new InvalidOperationException(
                "The window procedure failed.",
                _messageError);
        }
    }

    private void BeginSmoke()
    {
        _idleAllocationStart = GC.GetAllocatedBytesForCurrentThread();
        for (int i = 0; i < 32; i++)
        {
            Post(WmNull, 0);
        }
        Post(WmSmokeReady, 0);
    }

    private void CompleteSmoke()
    {
        IdlePumpBytes = GC.GetAllocatedBytesForCurrentThread() - _idleAllocationStart;
        IdleWorkingSet = Process.GetCurrentProcess().WorkingSet64;

        RECT rect;
        if (!GetClientRect(_window, out rect))
        {
            throw new InvalidOperationException("GetClientRect failed.");
        }
        int x = Math.Max(1, (rect.right - rect.left) / 2);
        int y = Math.Max(1, (rect.bottom - rect.top) / 2);
        nint point = PackPoint(x, y);
        Post(WmMouseMove, point);
        Post(WmLeftButtonDown, point);
        Post(WmMouseMove, PackPoint(x + 28, y));
        Post(WmRightButtonDown, PackPoint(x + 28, y));
        if (_visibleSmoke)
        {
            _smokeTimer = new Timer(
                static state =>
                {
                    MinesweeperApplication application =
                        (MinesweeperApplication)state!;
                    if (!PostMessageW(application._window, WmClose, 0, 0))
                    {
                        application._messageError =
                            new InvalidOperationException("PostMessageW failed for WM_CLOSE.");
                    }
                },
                this,
                500,
                Timeout.Infinite);
        }
        else
        {
            Post(WmClose, 0);
        }
    }

    private void Post(uint message, nint lparam)
    {
        if (!PostMessageW(_window, message, 0, lparam))
        {
            throw new InvalidOperationException($"PostMessageW failed for 0x{message:X}.");
        }
    }

    private Vector2 ClientSize()
    {
        if (!GetClientRect(_window, out RECT rect))
        {
            throw new InvalidOperationException("GetClientRect failed.");
        }
        return new Vector2
        {
            X = Math.Max(0, rect.right - rect.left),
            Y = Math.Max(0, rect.bottom - rect.top),
        };
    }

    private void RegisterWindowClass(HINSTANCE instance)
    {
        fixed (char* className = WindowClass)
        {
            WNDCLASSW windowClass = new()
            {
                lpfnWndProc = &WindowProc,
                hInstance = instance,
                lpszClassName = (ushort*)className,
            };
            if (RegisterClassW(&windowClass) == 0)
            {
                throw new InvalidOperationException("RegisterClassW failed.");
            }
        }
    }

    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvStdcall)])]
    private static nint WindowProc(HWND window, uint message, nuint wparam, nint lparam)
    {
        MinesweeperApplication? current = s_current;
        try
        {
            return current?.HandleMessage(window, message, wparam, lparam) ??
                DefWindowProcW(window, message, wparam, lparam);
        }
        catch (Exception error)
        {
            if (current is not null)
            {
                current._messageError = error;
            }
            PostQuitMessage(error.HResult);
            return 0;
        }
    }

    private nint HandleMessage(HWND window, uint message, nuint wparam, nint lparam)
    {
        switch (message)
        {
            case WmMouseMove:
                _game?.OnPointerMoved(PointFromLparam(lparam));
                return 0;
            case WmLeftButtonDown:
                _game?.OnPointerPressed(false, false);
                return 0;
            case WmRightButtonDown:
                _game?.OnPointerPressed(true, false);
                return 0;
            case WmSize:
                _game?.OnParentSizeChanged(new Vector2
                {
                    X = (ushort)(nuint)lparam,
                    Y = (ushort)((nuint)lparam >> 16),
                });
                return 0;
            case WmSmokeReady:
                CompleteSmoke();
                return 0;
            case WmClose:
                _ = DestroyWindow(window);
                return 0;
            case WmDestroy:
                _window = default;
                PostQuitMessage(0);
                return 0;
            default:
                return DefWindowProcW(window, message, wparam, lparam);
        }
    }

    private static Vector2 PointFromLparam(nint lparam) => new()
    {
        X = (short)(nuint)lparam,
        Y = (short)((nuint)lparam >> 16),
    };

    private static nint PackPoint(int x, int y) =>
        (nint)((uint)(ushort)x | ((uint)(ushort)y << 16));

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }
        _disposed = true;
        _smokeTimer?.Dispose();

        if (_window != default)
        {
            _ = DestroyWindow(_window);
            _window = default;
        }

        if (_compositionTarget is not null)
        {
            _compositionTarget.Root = null;
        }
        _game?.Dispose();
        _compositionTarget?.Dispose();
        _desktopTarget?.Dispose();
        _root?.Dispose();
        _compositor?.Dispose();
        _dispatcher?.Dispose();
        s_current = null;
    }
}
