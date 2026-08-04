use windows_csharp::Architecture;

fn main() {
    const FOUNDATION: &str = r"C:\Windows\System32\WinMetadata\Windows.Foundation.winmd";
    const DEFAULT: &str = "../../../libs/bindgen/default";
    const WINDOWS: &str = "../../../libs/bindgen/default/Windows.winmd";
    const WIN32: &str = "../../../libs/bindgen/default/Windows.Win32.winmd";

    println!("cargo:rerun-if-changed={FOUNDATION}");
    println!("cargo:rerun-if-changed={WINDOWS}");
    println!("cargo:rerun-if-changed={WIN32}");

    windows_csharp::builder()
        .input(DEFAULT)
        .input(FOUNDATION)
        .architecture(Architecture::X64)
        .member("Windows.UI.Composition.Compositor", "CreateContainerVisual")
        .member("Windows.UI.Composition.Compositor", "CreateSpriteVisual")
        .member("Windows.UI.Composition.Compositor", "CreateColorBrush")
        .member("Windows.UI.Composition.Compositor", "CreateNineGridBrush")
        .member("Windows.UI.Composition.Compositor", "CreateShapeVisual")
        .member("Windows.UI.Composition.Compositor", "CreateSpriteShape")
        .member("Windows.UI.Composition.Compositor", "CreateContainerShape")
        .member("Windows.UI.Composition.Compositor", "CreateEllipseGeometry")
        .member("Windows.UI.Composition.Compositor", "CreateScopedBatch")
        .member(
            "Windows.UI.Composition.Compositor",
            "CreateVector3KeyFrameAnimation",
        )
        .member("Windows.UI.Composition.ContainerVisual", "Children")
        .member(
            "Windows.UI.Composition.ContainerVisual",
            "RelativeSizeAdjustment",
        )
        .member(
            "Windows.UI.Composition.ContainerVisual",
            "RelativeOffsetAdjustment",
        )
        .member("Windows.UI.Composition.ContainerVisual", "AnchorPoint")
        .member("Windows.UI.Composition.ContainerVisual", "Scale")
        .member("Windows.UI.Composition.ContainerVisual", "Size")
        .member("Windows.UI.Composition.SpriteVisual", "Brush")
        .member(
            "Windows.UI.Composition.SpriteVisual",
            "RelativeSizeAdjustment",
        )
        .member("Windows.UI.Composition.SpriteVisual", "BorderMode")
        .member("Windows.UI.Composition.SpriteVisual", "Size")
        .member("Windows.UI.Composition.SpriteVisual", "CenterPoint")
        .member("Windows.UI.Composition.SpriteVisual", "Offset")
        .member("Windows.UI.Composition.SpriteVisual", "IsVisible")
        .member("Windows.UI.Composition.SpriteVisual", "ParentForTransform")
        .member("Windows.UI.Composition.SpriteVisual", "Children")
        .member("Windows.UI.Composition.SpriteVisual", "Parent")
        .member("Windows.UI.Composition.SpriteVisual", "Scale")
        .member("Windows.UI.Composition.SpriteVisual", "StartAnimation")
        .member(
            "Windows.UI.Composition.ShapeVisual",
            "RelativeSizeAdjustment",
        )
        .member("Windows.UI.Composition.ShapeVisual", "BorderMode")
        .member("Windows.UI.Composition.ShapeVisual", "Shapes")
        .member("Windows.UI.Composition.VisualCollection", "InsertAtTop")
        .member("Windows.UI.Composition.VisualCollection", "Remove")
        .member("Windows.UI.Composition.VisualCollection", "RemoveAll")
        .member(
            "Windows.UI.Composition.CompositionNineGridBrush",
            "SetInsets",
        )
        .member(
            "Windows.UI.Composition.CompositionNineGridBrush",
            "IsCenterHollow",
        )
        .member("Windows.UI.Composition.CompositionNineGridBrush", "Source")
        .member("Windows.UI.Composition.CompositionSpriteShape", "FillBrush")
        .member("Windows.UI.Composition.CompositionSpriteShape", "Offset")
        .member("Windows.UI.Composition.CompositionContainerShape", "Shapes")
        .member(
            "Windows.UI.Composition.CompositionEllipseGeometry",
            "Radius",
        )
        .member(
            "Windows.UI.Composition.Vector3KeyFrameAnimation",
            "InsertKeyFrame",
        )
        .member(
            "Windows.UI.Composition.Vector3KeyFrameAnimation",
            "Duration",
        )
        .member(
            "Windows.UI.Composition.Vector3KeyFrameAnimation",
            "DelayTime",
        )
        .member(
            "Windows.UI.Composition.Vector3KeyFrameAnimation",
            "IterationCount",
        )
        .member("Windows.UI.Composition.CompositionScopedBatch", "End")
        .member("Windows.UI.Composition.ICompositionTarget", "Root")
        .member(
            "Windows.Win32.ICompositorDesktopInterop",
            "CreateDesktopWindowTarget",
        )
        .function("Windows.Win32.CreateDispatcherQueueController")
        .function("Windows.Win32.GetModuleHandleW")
        .function("Windows.Win32.RegisterClassW")
        .function("Windows.Win32.CreateWindowExW")
        .function("Windows.Win32.DefWindowProcW")
        .function("Windows.Win32.GetClientRect")
        .function("Windows.Win32.GetMessageW")
        .function("Windows.Win32.TranslateMessage")
        .function("Windows.Win32.DispatchMessageW")
        .function("Windows.Win32.PostQuitMessage")
        .function("Windows.Win32.DestroyWindow")
        .function("Windows.Win32.PostMessageW")
        .constant("Windows.Win32.CW_USEDEFAULT")
        .constant("Windows.Win32.WS_OVERLAPPEDWINDOW")
        .constant("Windows.Win32.WS_VISIBLE")
        .output("Windows.cs")
        .write()
        .unwrap();
}
