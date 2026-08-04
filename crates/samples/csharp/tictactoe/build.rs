use windows_csharp::Architecture;

fn main() {
    const REACTOR: &str = "../../../tools/reactor/winmd";

    println!("cargo:rerun-if-changed={REACTOR}");
    windows_reactor_setup::as_framework_dependent();

    windows_csharp::builder()
        .input(REACTOR)
        .input_default()
        .architecture(Architecture::X64)
        .member("Microsoft.UI.Xaml.Application", "Start")
        .member(
            "Microsoft.UI.Dispatching.DispatcherQueue",
            "GetForCurrentThread",
        )
        .member("Microsoft.UI.Dispatching.DispatcherQueue", "TryEnqueue")
        .member("Microsoft.UI.Xaml.Window", "Title")
        .member("Microsoft.UI.Xaml.Window", "Content")
        .member("Microsoft.UI.Xaml.Window", "Activate")
        .member("Microsoft.UI.Xaml.Window", "Close")
        .member("Microsoft.UI.Xaml.Controls.StackPanel", "Children")
        .member("Microsoft.UI.Xaml.Controls.StackPanel", "Spacing")
        .member("Microsoft.UI.Xaml.Controls.Grid", "Children")
        .member("Microsoft.UI.Xaml.Controls.Grid", "RowDefinitions")
        .member("Microsoft.UI.Xaml.Controls.Grid", "ColumnDefinitions")
        .member("Microsoft.UI.Xaml.Controls.Grid", "RowSpacing")
        .member("Microsoft.UI.Xaml.Controls.Grid", "ColumnSpacing")
        .member("Microsoft.UI.Xaml.Controls.Grid", "SetRow")
        .member("Microsoft.UI.Xaml.Controls.Grid", "SetColumn")
        .member("Microsoft.UI.Xaml.Controls.RowDefinition", "Height")
        .member("Microsoft.UI.Xaml.Controls.ColumnDefinition", "Width")
        .member("Microsoft.UI.Xaml.Controls.Button", "Content")
        .member("Microsoft.UI.Xaml.Controls.Primitives.ButtonBase", "Click")
        .member("Microsoft.UI.Xaml.Controls.Control", "IsEnabled")
        .member("Microsoft.UI.Xaml.Controls.TextBlock", "Text")
        .member("Microsoft.UI.Xaml.Controls.TextBlock", "FontSize")
        .member("Microsoft.UI.Xaml.Controls.TextBlock", "FontWeight")
        .member("Microsoft.UI.Xaml.FrameworkElement", "HorizontalAlignment")
        .member("Microsoft.UI.Xaml.FrameworkElement", "VerticalAlignment")
        .member("Microsoft.UI.Xaml.FrameworkElement", "Margin")
        .member("Microsoft.UI.Xaml.FrameworkElement", "Width")
        .member("Microsoft.UI.Xaml.FrameworkElement", "Height")
        .function("extras.MddBootstrapInitialize2")
        .function("extras.MddBootstrapShutdown")
        .constant("extras.WINDOWSAPPSDK_RELEASE_MAJORMINOR")
        .constant("extras.WINDOWSAPPSDK_RUNTIME_VERSION_UINT64")
        .function("Windows.Win32.CoInitializeEx")
        .function("Windows.Win32.CoUninitialize")
        .function("Windows.Win32.SetProcessDpiAwarenessContext")
        .function("Windows.Win32.PostQuitMessage")
        .select("Windows.Win32.COINIT")
        .constant("Windows.Win32.DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2")
        .output("Windows.cs")
        .write()
        .unwrap();
}
