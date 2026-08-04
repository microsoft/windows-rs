using System;
using System.Threading;
using Microsoft.UI.Xaml;
using Windows.Win32;
using BootstrapApis = extras.Apis;
using Win32Apis = Windows.Win32.Apis;

namespace CSharpTicTacToe;

internal static unsafe class Program
{
    [STAThread]
    private static int Main(string[] args)
    {
        bool smoke = Array.IndexOf(args, "--smoke") >= 0;
        bool comInitialized = false;
        bool bootstrapInitialized = false;
        Application? application = null;
        TicTacToeApplication? sample = null;
        ApplicationInitializationCallback? initialization = null;
        Timer? watchdog = null;

        try
        {
            _ = Win32Apis.SetProcessDpiAwarenessContext(
                (DPI_AWARENESS_CONTEXT)(nint)
                    Win32Apis.DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
            Win32Apis.CoInitializeEx(null, (uint)COINIT.COINIT_APARTMENTTHREADED);
            comInitialized = true;

            PACKAGE_VERSION minimumVersion = new()
            {
                Anonymous = new PACKAGE_VERSION.PACKAGE_VERSION_0
                {
                    Version = BootstrapApis.WINDOWSAPPSDK_RUNTIME_VERSION_UINT64,
                },
            };
            BootstrapApis.MddBootstrapInitialize2(
                (uint)BootstrapApis.WINDOWSAPPSDK_RELEASE_MAJORMINOR,
                null,
                minimumVersion,
                extras.MddBootstrapInitializeOptions
                    .MddBootstrapInitializeOptions_OnNoMatch_ShowUI |
                extras.MddBootstrapInitializeOptions
                    .MddBootstrapInitializeOptions_OnPackageIdentity_NOOP);
            bootstrapInitialized = true;

            if (smoke)
            {
                watchdog = new Timer(
                    static _ => Environment.FailFast("Tic-Tac-Toe smoke test timed out."),
                    null,
                    TimeSpan.FromSeconds(10),
                    Timeout.InfiniteTimeSpan);
            }

            initialization = ApplicationInitializationCallback.Create(_ =>
            {
                if (application is not null)
                {
                    throw new InvalidOperationException(
                        "The WinUI initialization callback ran more than once.");
                }
                application = new Application();
                sample = new TicTacToeApplication(smoke);
            });

            Application.Start(initialization);
            watchdog?.Change(Timeout.Infinite, Timeout.Infinite);
            sample?.ThrowIfCallbackFailed();
            if (smoke)
            {
                Console.WriteLine("TICTACTOE_SMOKE ok");
            }
            return 0;
        }
        catch (Exception error)
        {
            Console.Error.WriteLine(error);
            return error.HResult == 0 ? 1 : error.HResult;
        }
        finally
        {
            watchdog?.Dispose();
            try
            {
                initialization?.Dispose();
                sample?.Dispose();
                application?.Dispose();
            }
            finally
            {
                try
                {
                    if (bootstrapInitialized)
                    {
                        BootstrapApis.MddBootstrapShutdown();
                    }
                }
                finally
                {
                    if (comInitialized)
                    {
                        Win32Apis.CoUninitialize();
                    }
                }
            }
        }
    }
}
