use bindings::*;
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};
use windows_core::*;

#[allow(dead_code, nonstandard_style, clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "\\native_pointer_bindings.rs"));
}

implement_decl! {
    #[compose]
    impl NativeApplication as NativeApplication_Impl:
        [IApplicationOverrides, IXamlMetadataProvider]
}

#[derive(Default)]
struct NativeApplication {
    provider: RefCell<Option<XamlControlsXamlMetaDataProvider>>,
    window: RefCell<Option<NativeWindow>>,
}

impl NativeApplication {
    fn provider(&self) -> Result<XamlControlsXamlMetaDataProvider> {
        if let Some(provider) = self.provider.borrow().as_ref() {
            return Ok(provider.clone());
        }
        let provider = XamlControlsXamlMetaDataProvider::new()?;
        *self.provider.borrow_mut() = Some(provider.clone());
        Ok(provider)
    }
}

impl IApplicationOverrides_Impl for NativeApplication_Impl {
    fn OnLaunched(&self, _: Ref<LaunchActivatedEventArgs>) -> Result<()> {
        let resources = XamlControlsResources::new()?.cast::<ResourceDictionary>()?;
        Application::Current()?
            .Resources()?
            .MergedDictionaries()?
            .Append(&resources)?;
        *self.window.borrow_mut() = Some(NativeWindow::new()?);
        Ok(())
    }
}

impl IXamlMetadataProvider_Impl for NativeApplication_Impl {
    fn GetXamlType(&self, value: &TypeName) -> Result<IXamlType> {
        self.provider()?.GetXamlType(value)
    }

    fn GetXamlTypeByFullName(&self, name: &HSTRING) -> Result<IXamlType> {
        self.provider()?
            .GetXamlTypeByFullName(&name.to_string_lossy())
    }

    fn GetXmlnsDefinitions(&self) -> Result<Array<XmlnsDefinition>> {
        self.provider()?.GetXmlnsDefinitions()
    }
}

struct NativeWindow {
    _window: Window,
    _events: Vec<EventRevoker>,
}

impl NativeWindow {
    fn new() -> Result<Self> {
        let root = XamlReader::Load(
            r##"<Border xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
                       xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
                       Background="{ThemeResource SystemControlBackgroundAltHighBrush}">
                <Grid>
                    <Grid.RowDefinitions>
                        <RowDefinition Height="Auto"/>
                        <RowDefinition Height="*"/>
                    </Grid.RowDefinitions>
                    <TitleBar x:Name="chrome" IsTabStop="False"
                              Title="Reactor UI fixture: pointer-wheel-native"/>
                    <Border Grid.Row="1">
                        <StackPanel Margin="20" Spacing="12">
                            <Button Content="Change mode"/>
                            <Border>
                                <Border x:Name="target" Width="200" Height="100"
                                        Background="#FFF0F0F0">
                                    <TextBlock AutomationProperties.AutomationId="routing-target"
                                               Foreground="Black" Text="Pointer target"/>
                                </Border>
                            </Border>
                            <TextBlock Text="Native WinUI wheel input"/>
                            <TextBlock x:Name="calls" AutomationProperties.AutomationId="calls"/>
                        </StackPanel>
                    </Border>
                </Grid>
            </Border>"##,
        )?
        .cast::<FrameworkElement>()?;
        let target = root.FindName("target")?.cast::<UIElement>()?;
        let calls = root.FindName("calls")?.cast::<TextBlock>()?;
        let chrome = root.FindName("chrome")?.cast::<TitleBar>()?;
        chrome
            .cast::<ITitleBar2>()?
            .SetAutoRefreshDragRegions(true)?;
        let window = Window::new()?;
        window.SetTitle("Reactor UI fixture: pointer-wheel-native")?;
        window.SetContent(&root)?;
        window.SetExtendsContentIntoTitleBar(true)?;
        window.SetTitleBar(&chrome)?;
        window
            .cast::<IWindow2>()?
            .AppWindow()?
            .TitleBar()?
            .cast::<IAppWindowTitleBar2>()?
            .SetPreferredHeightOption(TitleBarHeightOption::Standard)?;
        let mut events = Vec::new();
        let trace = |name| {
            move |sender: Ref<IInspectable>, args: Ref<PointerRoutedEventArgs>| {
                let element = sender.unwrap().cast::<UIElement>().unwrap();
                let point = args.unwrap().GetCurrentPoint(&element).unwrap();
                eprintln!("native XAML {name}: {:?}", point.Position().unwrap());
            }
        };
        events.push(target.PointerEntered(trace("entered"))?);
        events.push(target.PointerMoved(trace("moved"))?);
        events.push(target.PointerWheelChanged(move |sender, args| {
            let element = sender.unwrap().cast::<UIElement>().unwrap();
            let args = args.unwrap();
            let properties = args
                .GetCurrentPoint(&element)
                .unwrap()
                .Properties()
                .unwrap();
            let modifiers = args.KeyModifiers().unwrap();
            let report = format!(
                "delta {} horizontal {} shift {} control {}",
                properties.MouseWheelDelta().unwrap(),
                properties.IsHorizontalMouseWheel().unwrap(),
                modifiers.contains(VirtualKeyModifiers::Shift),
                modifiers.contains(VirtualKeyModifiers::Control),
            );
            eprintln!("native XAML wheel: {report}");
            calls.SetText(&report).unwrap();
        })?);
        events.push(window.Closed(|_, _| {
            Application::Current().unwrap().Exit().unwrap();
        })?);
        window.Activate()?;
        events.extend(super::input_trace::subscribe()?);
        Ok(Self {
            _window: window,
            _events: events,
        })
    }
}

pub fn run() -> Result<()> {
    unsafe {
        _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED as u32).ok()?;
    }
    let application = Arc::new(Mutex::new(None));
    let initialized = application.clone();
    let result = Application::Start(&ApplicationInitializationCallback::new(move |_| {
        *initialized.lock().unwrap() =
            Some(Application::compose(NativeApplication::default()).unwrap());
    }));
    drop(application.lock().unwrap().take());
    result
}
