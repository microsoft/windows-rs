fn main() {
    let reference = "../../../libs/bindgen/default";

    // Generate a minimal slice of the `Windows.UI.Xaml` API surface — only the
    // types this sample actually uses. Bindgen will skip methods on the
    // included types that depend on Xaml types that fall outside the filter
    // (those skipped methods are reported as warnings and printed via
    // `cargo:warning`); the sample itself never calls them, so the resulting
    // bindings still compile and exercise the round-trip we want.
    _ = windows_bindgen::builder()
        .input(reference)
        .output("src/bindings.rs")
        .filter("Windows.UI.Xaml.Application")
        .filter("Windows.UI.Xaml.IApplicationOverrides")
        .filter("Windows.UI.Xaml.ApplicationInitializationCallback")
        .filter("Windows.UI.Xaml.Window")
        .filter("Windows.UI.Xaml.Controls.TextBox")
        .filter("Windows.UI.Xaml.WindowCreatedEventArgs")
        // The Xaml namespaces are no longer in the `windows` crate (PR #1836),
        // so the transitive Xaml dependencies that are kept must be emitted
        // inline (default behaviour). The non-Xaml namespaces that Xaml types
        // reference are still in the `windows` crate and are routed there via
        // narrow references rather than the catch-all `.reference("windows")`
        // (which would also try to route the Xaml types themselves into a
        // `windows::UI::Xaml` module that no longer exists).
        .reference("windows,skip-root,Windows.Foundation")
        .reference("windows,skip-root,Windows.ApplicationModel")
        .reference("windows,skip-root,Windows.UI.Composition")
        .reference("windows,skip-root,Windows.UI.Core")
        .reference("windows,skip-root,Windows.UI.Input")
        .reference("windows,skip-root,Windows.UI.Text")
        .reference("windows,skip-root,Windows.Win32")
        .flat()
        .implement()
        .write();
}
