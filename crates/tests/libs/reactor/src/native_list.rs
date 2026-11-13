use std::{cell::RefCell, collections::HashMap, rc::Rc};
use windows_collections::IVector;
use windows_core::{EventRevoker, HSTRING, IInspectable, Interface, Result};
use windows_reactor::test::{native_test_handler, native_test_list_source, run_native_fixture};
use windows_reference::IReference;

#[allow(
    clippy::upper_case_acronyms,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
mod native {
    include!(concat!(env!("OUT_DIR"), "/native_bindings.rs"));
}

struct Row {
    button: native::Button,
    host: Option<native::IContentControl>,
    _click: EventRevoker,
}

fn label(button: &native::Button, key: u64, clicks: usize) -> Result<()> {
    let text = format!("Row {key}; value 0; clicks {clicks}");
    let value: IInspectable = IReference::from(HSTRING::from(&text)).into();
    button
        .cast::<native::IContentControl>()?
        .SetContent(&value)?;
    native::AutomationProperties::SetName(button, &text)
}

pub(super) fn run() -> Result<()> {
    run_native_fixture(|| {
        let root = native::XamlReader::Load(
            "<Grid xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' \
             xmlns:x='http://schemas.microsoft.com/winfx/2006/xaml'>\
             <Grid.RowDefinitions><RowDefinition Height='Auto'/><RowDefinition/></Grid.RowDefinitions>\
             <StackPanel>\
             <TextBlock x:Name='Revision' AutomationProperties.AutomationId='revision' Text='Revision 0'/>\
             <TextBlock Text='[]'/>\
             <StackPanel Orientation='Horizontal'><Button Content='Reverse'/><Button Content='Select'/>\
             <Button Content='Keep 3'/><Button Content='Append'/><Button Content='Set'/>\
             <Button Content='Clear'/><Button Content='Release'/></StackPanel>\
             <TextBlock Text='Clicked row 0 drops 0'/><Button Content='0:0:0:0'/>\
             <StackPanel Orientation='Horizontal'><Button Content='Rerender'/><Button Content='Change row 0'/>\
             <Button Content='Toggle row 0'/><Button Content='Toggle empty'/><Button Content='Edit row 0'/>\
             </StackPanel></StackPanel>\
             <ListView x:Name='Rows' Grid.Row='1' AutomationProperties.AutomationId='rows' \
             SelectionMode='Single' CanDragItems='True' CanReorderItems='True' AllowDrop='True' \
             ShowsScrollingPlaceholders='False'>\
             <ListView.ItemContainerTransitions><TransitionCollection/></ListView.ItemContainerTransitions>\
             <ListView.ItemTemplate><DataTemplate>\
             <ContentControl IsTabStop='False' HorizontalContentAlignment='Stretch' \
             AutomationProperties.Name='{Binding Content.(AutomationProperties.Name), RelativeSource={RelativeSource Self}}'/>\
             </DataTemplate></ListView.ItemTemplate>\
             <ListView.ItemContainerStyle><Style TargetType='ListViewItem'>\
             <Setter Property='HorizontalContentAlignment' Value='Stretch'/>\
             <Setter Property='MinHeight' Value='0'/>\
             </Style></ListView.ItemContainerStyle>\
             <ListView.ItemsPanel><ItemsPanelTemplate>\
             <ItemsStackPanel ItemsUpdatingScrollMode='KeepScrollOffset' CacheLength='1'/>\
             </ItemsPanelTemplate></ListView.ItemsPanel>\
             </ListView></Grid>",
        )?
        .cast::<native::IFrameworkElement>()?;
        let list = root.FindName("Rows")?;
        let revision = root.FindName("Revision")?.cast::<native::ITextBlock>()?;
        let source = native_test_list_source((0..200).collect())?;
        let rows = Rc::new(RefCell::new(HashMap::<u64, Row>::new()));
        let changing = list
            .cast::<native::IListViewBase>()?
            .ContainerContentChanging(native_test_handler::<
                _,
                native::ContainerContentChangingEventArgs,
                _,
            >({
                let rows = Rc::clone(&rows);
                move |_, args| {
                    let args = args.unwrap();
                    args.SetHandled(true).unwrap();
                    if args.Phase().unwrap() != 0 || args.InRecycleQueue().unwrap() {
                        return;
                    }
                    let key = args
                        .Item()
                        .unwrap()
                        .cast::<IReference<u64>>()
                        .unwrap()
                        .Value()
                        .unwrap();
                    let host = args
                        .ItemContainer()
                        .unwrap()
                        .cast::<native::IContentControl>()
                        .unwrap()
                        .ContentTemplateRoot()
                        .unwrap()
                        .cast::<native::IContentControl>()
                        .unwrap();
                    let mut rows = rows.borrow_mut();
                    if let Some(row) = rows.get(&key)
                        && row.host.as_ref() == Some(&host)
                    {
                        return;
                    }
                    for row in rows.values_mut() {
                        if row.host.as_ref() == Some(&host) {
                            row.host = None;
                        }
                    }
                    let row = rows.entry(key).or_insert_with(|| {
                        let button = native::Button::new().unwrap();
                        button
                            .cast::<native::IFrameworkElement>()
                            .unwrap()
                            .SetHeight(32.0)
                            .unwrap();
                        button
                            .cast::<native::IFrameworkElement>()
                            .unwrap()
                            .SetHorizontalAlignment(native::HorizontalAlignment::Stretch)
                            .unwrap();
                        native::AutomationProperties::SetAutomationId(
                            &button,
                            &format!("row-{key}"),
                        )
                        .unwrap();
                        label(&button, key, 0).unwrap();
                        let clicks = std::cell::Cell::new(0);
                        let click = button
                            .cast::<native::IButtonBase>()
                            .unwrap()
                            .Click(native_test_handler({
                                let button = button.clone();
                                move |_, _| {
                                    clicks.set(clicks.get() + 1);
                                    label(&button, key, clicks.get()).unwrap();
                                }
                            }))
                            .unwrap();
                        Row {
                            button,
                            host: None,
                            _click: click,
                        }
                    });
                    if let Some(previous) = row.host.take() {
                        previous.SetContent(None::<&IInspectable>).unwrap();
                    }
                    host.SetContent(&row.button).unwrap();
                    row.host = Some(host);
                }
            }))?;
        let completed =
            list.cast::<native::IListViewBase>()?
                .DragItemsCompleted(native_test_handler::<
                    _,
                    native::DragItemsCompletedEventArgs,
                    _,
                >({
                    let source = source.cast::<IVector<IInspectable>>()?;
                    move |_, args| {
                        let order: Vec<_> = (0..8)
                            .map(|index| {
                                source
                                    .GetAt(index)
                                    .unwrap()
                                    .cast::<IReference<u64>>()
                                    .unwrap()
                                    .Value()
                                    .unwrap()
                            })
                            .collect();
                        eprintln!(
                            "native drag {:?}: {order:?}",
                            args.unwrap().DropResult().unwrap()
                        );
                        assert_eq!(order, [1, 2, 3, 0, 4, 5, 6, 7]);
                        revision.SetText("Revision 1").unwrap();
                    }
                }))?;
        list.cast::<native::IItemsControl>()?
            .SetItemsSource(&source)?;
        let window = native::Window::new()?;
        window.SetTitle("Reactor UI fixture: native_list")?;
        window.SetContent(&root.cast::<native::UIElement>()?)?;
        let closed = window.Closed(native_test_handler(|_, _| {
            native::Application::Current().unwrap().Exit().unwrap();
        }))?;
        window.Activate()?;
        unsafe {
            let handle = window.cast::<native::IWindowNative>()?.WindowHandle()?;
            let mut client = native::RECT::default();
            let mut bounds = native::RECT::default();
            native::GetClientRect(handle, &mut client).ok()?;
            native::GetWindowRect(handle, &mut bounds).ok()?;
            let dpi = native::GetDpiForWindow(handle);
            native::SetWindowPos(
                handle,
                std::ptr::null_mut(),
                100,
                100,
                (500 * dpi / 96) as i32 + bounds.right - bounds.left - client.right,
                (360 * dpi / 96) as i32 + bounds.bottom - bounds.top - client.bottom,
                (native::SWP_NOACTIVATE | native::SWP_NOZORDER) as u32,
            )
            .ok()?;
        }
        Ok(((changing, completed, closed), rows, window))
    })
}
