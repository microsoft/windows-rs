fn main() {
    windows_bindgen::builder()
        .input("../../../tools/reactor-metadata/winmd")
        .input_default()
        .filters([
            "Microsoft.UI.Dispatching.DispatcherQueue.GetForCurrentThread",
            "Microsoft.UI.Dispatching.DispatcherQueueHandler",
            "Microsoft.UI.Dispatching.DispatcherQueuePriority.Low",
            "Microsoft.UI.Dispatching.IDispatcherQueue.TryEnqueueWithPriority",
            "Microsoft.UI.Xaml.Media.CompositionTarget.Rendering",
        ])
        .write();
}
