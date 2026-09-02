fn main() {
    let bindings = format!("{}/bindings.rs", std::env::var("OUT_DIR").unwrap());
    windows_bindgen::builder()
        .input("../../../tools/reactor/winmd")
        .input_default()
        .output(bindings)
        .flat()
        .minimal()
        .filters([
            "Microsoft.UI.Dispatching.DispatcherQueue.GetForCurrentThread",
            "Microsoft.UI.Dispatching.DispatcherQueueHandler",
            "Microsoft.UI.Dispatching.DispatcherQueuePriority.Low",
            "Microsoft.UI.Dispatching.IDispatcherQueue.TryEnqueueWithPriority",
            "Microsoft.UI.Xaml.Media.CompositionTarget.Rendering",
        ])
        .write();
}
