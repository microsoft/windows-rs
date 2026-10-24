impl WinUiAdapter {
    fn set_resource_overrides(
        &mut self,
        object: ObjectId,
        resources: &ResourceOverrides,
    ) -> Result<(), WinUiError> {
        let dictionary = self
            .ui_element(object)?
            .cast::<native::IFrameworkElement>()?
            .Resources()?;
        let map = dictionary.cast::<IMap<IInspectable, IInspectable>>()?;
        let desired = resources
            .values()
            .map(|(key, _)| key.to_string())
            .collect::<HashSet<_>>();
        if let Some(previous) = self.resource_override_keys.get(&object) {
            for key in previous.difference(&desired) {
                let key = windows_reference::IReference::from(key.as_str());
                if map.HasKey(&key)? {
                    map.Remove(&key)?;
                }
            }
        }
        for (key, value) in resources.values() {
            let key = windows_reference::IReference::from(key);
            let value: IInspectable = match value {
                ResourceValue::Color(value) => solid_color_brush(*value)?.into(),
                ResourceValue::Thickness(value) => {
                    windows_reference::IReference::from(native::Thickness {
                        left: value.left,
                        top: value.top,
                        right: value.right,
                        bottom: value.bottom,
                    })
                    .into()
                }
                ResourceValue::CornerRadius(value) => {
                    windows_reference::IReference::from(native::CornerRadius {
                        top_left: value.top_left,
                        top_right: value.top_right,
                        bottom_right: value.bottom_right,
                        bottom_left: value.bottom_left,
                    })
                    .into()
                }
            };
            map.Insert(&key, &value)?;
        }
        if desired.is_empty() {
            self.resource_override_keys.remove(&object);
        } else {
            self.resource_override_keys.insert(object, desired);
        }
        Ok(())
    }

    fn set_key_accelerators(
        &self,
        object: ObjectId,
        accelerators: &KeyAccelerators,
    ) -> Result<(), WinUiError> {
        let element = self.ui_element(object)?;
        let values = element.KeyboardAccelerators()?;
        values.Clear()?;
        element.SetKeyboardAcceleratorPlacementMode(
            native::KeyboardAcceleratorPlacementMode::Hidden,
        )?;
        for accelerator in accelerators.values() {
            let value = native::KeyboardAccelerator::new()?;
            value.SetKey(match accelerator.key {
                AcceleratorKey::Left => native::VirtualKey::Left,
                AcceleratorKey::Up => native::VirtualKey::Up,
                AcceleratorKey::Right => native::VirtualKey::Right,
                AcceleratorKey::Down => native::VirtualKey::Down,
                AcceleratorKey::Space => native::VirtualKey::Space,
                AcceleratorKey::N => native::VirtualKey::N,
                AcceleratorKey::P => native::VirtualKey::P,
                AcceleratorKey::R => native::VirtualKey::R,
                AcceleratorKey::NumberPad0 => native::VirtualKey::NumberPad0,
                AcceleratorKey::NumberPad1 => native::VirtualKey::NumberPad1,
                AcceleratorKey::NumberPad2 => native::VirtualKey::NumberPad2,
                AcceleratorKey::NumberPad3 => native::VirtualKey::NumberPad3,
                AcceleratorKey::NumberPad4 => native::VirtualKey::NumberPad4,
                AcceleratorKey::NumberPad5 => native::VirtualKey::NumberPad5,
                AcceleratorKey::NumberPad6 => native::VirtualKey::NumberPad6,
                AcceleratorKey::NumberPad7 => native::VirtualKey::NumberPad7,
                AcceleratorKey::NumberPad8 => native::VirtualKey::NumberPad8,
                AcceleratorKey::NumberPad9 => native::VirtualKey::NumberPad9,
                AcceleratorKey::Divide => native::VirtualKey::Divide,
                AcceleratorKey::Multiply => native::VirtualKey::Multiply,
                AcceleratorKey::Subtract => native::VirtualKey::Subtract,
                AcceleratorKey::Add => native::VirtualKey::Add,
                AcceleratorKey::Decimal => native::VirtualKey::Decimal,
                AcceleratorKey::Enter => native::VirtualKey::Enter,
            })?;
            value.SetModifiers(match accelerator.modifiers {
                AcceleratorModifiers::None => native::VirtualKeyModifiers::None,
                AcceleratorModifiers::Control => native::VirtualKeyModifiers::Control,
            })?;
            let callback = accelerator.callback.clone();
            value
                .Invoked(move |_, args| {
                    if let Some(args) = args.as_ref() {
                        _ = args.SetHandled(true);
                    }
                    callback.call(());
                })?
                .into_token();
            values.Append(&value)?;
        }
        Ok(())
    }

    fn set_button_style(
        &mut self,
        object: ObjectId,
        value: Option<ButtonStyle>,
    ) -> Result<(), WinUiError> {
        let base = match value {
            None | Some(ButtonStyle::Default) => None,
            Some(value) => Some(value),
        };
        // State exists only while a style is applied, so clearing without state is a no-op.
        if base.is_none() && !self.style_states.contains_key(&object) {
            return Ok(());
        }
        self.style_states.entry(object).or_default().base = base;
        self.apply_style_state(object)
    }

    fn set_theme_brush(
        &mut self,
        object: ObjectId,
        property: PropertyId,
        value: Option<ThemeBrush>,
    ) -> Result<(), WinUiError> {
        if value.is_none() && !self.style_states.contains_key(&object) {
            return Ok(());
        }
        let state = self.style_states.entry(object).or_default();
        if let Some(value) = value {
            state.brushes.insert(property, value);
        } else {
            state.brushes.remove(&property);
        }
        self.apply_style_state(object)
    }

    fn apply_style_state(&mut self, object: ObjectId) -> Result<(), WinUiError> {
        let kind = self.kind(object)?;
        let state = self.style_states.get(&object).cloned().unwrap_or_default();
        let element = self
            .ui_element(object)?
            .cast::<native::IFrameworkElement>()?;
        if state.base.is_none() && state.brushes.is_empty() {
            self.style_states.remove(&object);
            element.SetStyle(None::<&native::Style>)?;
            return Ok(());
        }
        if state.brushes.is_empty() {
            let style = self.lookup_button_style(state.base.unwrap())?;
            element.SetStyle(&style)?;
            return Ok(());
        }
        let key = NativeStyleKey {
            kind,
            base: state.base,
            brushes: state
                .brushes
                .iter()
                .map(|(property, brush)| (*property, *brush))
                .collect(),
        };
        let style = if let Some(style) = self.style_cache.get(&key) {
            style.clone()
        } else {
            let (target, properties) =
                theme_style_info(kind).ok_or(WinUiError::InvalidObject(object))?;
            let based_on = state
                .base
                .map(button_style_resource)
                .map(|resource| format!(" BasedOn='{{StaticResource {resource}}}'"))
                .unwrap_or_default();
            let mut xaml = format!(
                "<Style xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' \
                 TargetType='{target}'{based_on}>"
            );
            for (property, native_name) in properties {
                if let Some(brush) = state.brushes.get(property) {
                    xaml.push_str(&format!(
                        "<Setter Property='{native_name}' Value='{{ThemeResource {}}}'/>",
                        brush.resource_key()
                    ));
                }
            }
            xaml.push_str("</Style>");
            let style = native::XamlReader::Load(&xaml)?.cast::<native::Style>()?;
            self.style_cache.insert(key, style.clone());
            style
        };
        element.SetStyle(&style)?;
        Ok(())
    }

    fn lookup_button_style(&self, style: ButtonStyle) -> Result<native::Style, WinUiError> {
        let resources = native::Application::Current()?.Resources()?;
        let resources =
            resources.cast::<IMap<IInspectable, IInspectable>>()?;
        let key = windows_reference::IReference::from(button_style_resource(style));
        Ok(resources.Lookup(&key)?.cast()?)
    }
    pub fn set_event_waker(&mut self, waker: impl Fn() + 'static) {
        *self.event_queue.waker.borrow_mut() = Some(Rc::new(waker));
        if !self.event_queue.events.borrow().is_empty() {
            Self::schedule_event_wake(&self.event_queue);
        }
    }

    pub fn set_realizer(&mut self, realizer: impl Fn() + 'static) {
        *self.event_queue.realizer.borrow_mut() = Some(Rc::new(realizer));
    }

    pub fn create_window_with_policy(
        &self,
        root_object: ObjectId,
        policy: &WindowPolicy,
    ) -> Result<NativeWindow, WinUiError> {
        if self
            .live_windows()
            .iter()
            .any(|window| window.root == root_object)
        {
            return Err(WinUiError::DuplicateWindowRoot(root_object));
        }
        let window = native::Window::new()?;
        let root = self.ui_element(root_object)?;
        // The opaque XAML island follows the app theme, not the requested window theme.
        let surface = native::Border::new()?;
        let background_style = native::XamlReader::Load(
            "<Style xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' \
             TargetType='Border'><Setter Property='Background' \
             Value='{ThemeResource SystemControlBackgroundAltHighBrush}'/></Style>",
        )?
        .cast::<native::Style>()?;
        surface
            .cast::<native::FrameworkElement>()?
            .SetStyle(&background_style)?;
        surface.SetChild(&root)?;
        window.SetContent(&surface)?;
        let state = Rc::new(NativeWindowState {
            window,
            root: root_object,
            root_element: root.cast()?,
            surface,
            background_style,
            title_bar: Cell::new(None),
            shown: Cell::new(false),
            closed: Cell::new(false),
            maximize_on_first_show: Cell::new(false),
            placement_callback: RefCell::new(None),
            last_placement: Cell::new(None),
        });
        if let Some((object, height)) = self.window_title_bar
            && self.owns(state.root, object)
        {
            self.apply_title_bar_to_window(&state, object, height)?;
        }
        self.apply_window_policy(&state.window, &state.surface.cast()?, policy)?;
        self.windows.borrow_mut().push(Rc::downgrade(&state));
        Ok(NativeWindow {
            state,
            actual_theme_changed: None,
            closed: None,
            published_title: None,
            published_visuals: false,
            size_changed: None,
            placement_changed: None,
            visuals: WindowVisuals {
                client_size: policy.client_size,
                constraints: policy.minimum_client_size.map(|(width, height)| {
                    WindowConstraints {
                        min_width: Some(width),
                        min_height: Some(height),
                        ..Default::default()
                    }
                }),
                theme: policy.theme,
                ..Default::default()
            },
        })
    }

    fn owns(&self, root: ObjectId, mut object: ObjectId) -> bool {
        loop {
            if object == root {
                return true;
            }
            let Some((parent, _)) = self.owners.get(&object) else {
                return false;
            };
            object = *parent;
        }
    }

    fn live_windows(&self) -> Vec<Rc<NativeWindowState>> {
        let mut windows = self.windows.borrow_mut();
        let live = windows.iter().filter_map(Weak::upgrade).collect::<Vec<_>>();
        windows.retain(|window| window.strong_count() != 0);
        live
    }

    fn apply_title_bar_to_window(
        &self,
        state: &NativeWindowState,
        object: ObjectId,
        height: WindowTitleBarHeight,
    ) -> Result<(), WinUiError> {
        if self.kind(object)? != ObjectType::TitleBar {
            return Err(WinUiError::InvalidObject(object));
        }
        let element = self.ui_element(object)?;
        element.SetIsTabStop(false)?;
        state.window.SetExtendsContentIntoTitleBar(true)?;
        state.window.SetTitleBar(&element)?;
        state
            .window
            .cast::<native::IWindow2>()?
            .AppWindow()?
            .TitleBar()?
            .cast::<native::IAppWindowTitleBar2>()?
            .SetPreferredHeightOption(match height {
                WindowTitleBarHeight::Standard => native::TitleBarHeightOption::Standard,
                WindowTitleBarHeight::Tall => native::TitleBarHeightOption::Tall,
                WindowTitleBarHeight::Collapsed => native::TitleBarHeightOption::Collapsed,
            })?;
        state.title_bar.set(Some((object, height)));
        Ok(())
    }

    fn clear_title_bar_from_window(
        state: &NativeWindowState,
        object: ObjectId,
    ) -> Result<(), WinUiError> {
        if !state
            .title_bar
            .get()
            .is_some_and(|(current, _)| current == object)
        {
            return Ok(());
        }
        state
            .window
            .cast::<native::IWindow2>()?
            .AppWindow()?
            .TitleBar()?
            .cast::<native::IAppWindowTitleBar2>()?
            .SetPreferredHeightOption(native::TitleBarHeightOption::Standard)?;
        state.window.SetTitleBar(None::<&native::UIElement>)?;
        state.window.SetExtendsContentIntoTitleBar(false)?;
        state.title_bar.set(None);
        Ok(())
    }

    fn set_window_title_bar(
        &mut self,
        object: ObjectId,
        height: WindowTitleBarHeight,
    ) -> Result<(), WinUiError> {
        if self.kind(object)? != ObjectType::TitleBar {
            return Err(WinUiError::InvalidObject(object));
        }
        self.window_title_bar = Some((object, height));
        Ok(())
    }

    fn clear_window_title_bar(&mut self, object: ObjectId) -> Result<(), WinUiError> {
        for window in self.live_windows() {
            Self::clear_title_bar_from_window(&window, object)?;
        }
        if self
            .window_title_bar
            .is_some_and(|(current, _)| current == object)
        {
            self.window_title_bar = None;
        }
        Ok(())
    }

    fn set_tooltip(
        &mut self,
        target: ObjectId,
        tooltip: Option<ObjectId>,
        placement: TooltipPlacement,
    ) -> Result<(), WinUiError> {
        if self.tooltip_owners.contains_key(&target) {
            return Err(WinUiError::StillOwned(target));
        }
        let target_element = self
            .ui_element(target)?
            .cast::<native::DependencyObject>()?;
        let tooltip_element = tooltip
            .map(|tooltip| {
                if tooltip == target || self.kind(tooltip)? != ObjectType::ToolTip {
                    return Err(WinUiError::InvalidObject(tooltip));
                }
                if self.owners.contains_key(&tooltip)
                    || self
                        .tooltip_owners
                        .get(&tooltip)
                        .is_some_and(|owner| *owner != target)
                {
                    return Err(WinUiError::StillOwned(tooltip));
                }
                self.ui_element(tooltip)
            })
            .transpose()?;
        if let Some(tooltip) = &tooltip_element {
            native::ToolTipService::SetToolTip(&target_element, tooltip)?;
        } else {
            native::ToolTipService::SetToolTip(&target_element, None::<&IInspectable>)?;
        }
        native::ToolTipService::SetPlacement(
            &target_element,
            match placement {
                TooltipPlacement::Top => native::PlacementMode::Top,
                TooltipPlacement::Bottom => native::PlacementMode::Bottom,
                TooltipPlacement::Left => native::PlacementMode::Left,
                TooltipPlacement::Right => native::PlacementMode::Right,
                TooltipPlacement::Mouse => native::PlacementMode::Mouse,
            },
        )?;
        if let Some((previous, _)) = self.tooltips.remove(&target) {
            self.tooltip_owners.remove(&previous);
        }
        if let Some(tooltip) = tooltip {
            self.tooltips.insert(target, (tooltip, placement));
            self.tooltip_owners.insert(tooltip, target);
        }
        Ok(())
    }

    fn set_flyout(
        &mut self,
        target: ObjectId,
        content: Option<ObjectId>,
        placement: FlyoutPlacement,
    ) -> Result<(), WinUiError> {
        if self.flyout_owners.contains_key(&target) {
            return Err(WinUiError::StillOwned(target));
        }
        let content_element = content
            .map(|content| {
                if content == target
                    || self.owners.contains_key(&content)
                    || self.tooltip_owners.contains_key(&content)
                    || self
                        .flyout_owners
                        .get(&content)
                        .is_some_and(|owner| *owner != target)
                    || self.content_dialog_owners.contains_key(&content)
                {
                    return Err(WinUiError::StillOwned(content));
                }
                self.ui_element(content)
            })
            .transpose()?;
        let handle = self
            .handles
            .get(&target)
            .ok_or(WinUiError::InvalidObject(target))?;
        if let Some(content) = content {
            let flyout = if let Some((flyout, previous, _)) = self.flyouts.get(&target) {
                if *previous != content {
                    flyout.SetContent(None::<&native::UIElement>)?;
                }
                flyout.clone()
            } else {
                let flyout = native::Flyout::new()?;
                match handle {
                    Handle::Generated(GeneratedHandle::Button(control)) => {
                        control.value.SetFlyout(&flyout)?;
                    }
                    Handle::Generated(GeneratedHandle::SplitButton(control)) => {
                        control.value.SetFlyout(&flyout)?;
                    }
                    _ => return Err(WinUiError::InvalidObject(target)),
                }
                flyout
            };
            flyout.SetContent(content_element.as_ref().unwrap())?;
            flyout
                .cast::<native::IFlyoutBase>()?
                .SetPlacement(match placement {
                    FlyoutPlacement::Top => native::FlyoutPlacementMode::Top,
                    FlyoutPlacement::Bottom => native::FlyoutPlacementMode::Bottom,
                    FlyoutPlacement::Left => native::FlyoutPlacementMode::Left,
                    FlyoutPlacement::Right => native::FlyoutPlacementMode::Right,
                    FlyoutPlacement::Full => native::FlyoutPlacementMode::Full,
                    FlyoutPlacement::TopEdgeAlignedLeft => {
                        native::FlyoutPlacementMode::TopEdgeAlignedLeft
                    }
                    FlyoutPlacement::TopEdgeAlignedRight => {
                        native::FlyoutPlacementMode::TopEdgeAlignedRight
                    }
                    FlyoutPlacement::BottomEdgeAlignedLeft => {
                        native::FlyoutPlacementMode::BottomEdgeAlignedLeft
                    }
                    FlyoutPlacement::BottomEdgeAlignedRight => {
                        native::FlyoutPlacementMode::BottomEdgeAlignedRight
                    }
                    FlyoutPlacement::LeftEdgeAlignedTop => {
                        native::FlyoutPlacementMode::LeftEdgeAlignedTop
                    }
                    FlyoutPlacement::LeftEdgeAlignedBottom => {
                        native::FlyoutPlacementMode::LeftEdgeAlignedBottom
                    }
                    FlyoutPlacement::RightEdgeAlignedTop => {
                        native::FlyoutPlacementMode::RightEdgeAlignedTop
                    }
                    FlyoutPlacement::RightEdgeAlignedBottom => {
                        native::FlyoutPlacementMode::RightEdgeAlignedBottom
                    }
                    FlyoutPlacement::Auto => native::FlyoutPlacementMode::Auto,
                })?;
            if let Some((_, previous, _)) =
                self.flyouts.insert(target, (flyout, content, placement))
            {
                self.flyout_owners.remove(&previous);
            }
            self.flyout_owners.insert(content, target);
        } else {
            if let Some((flyout, previous, _)) = self.flyouts.remove(&target) {
                flyout.SetContent(None::<&native::UIElement>)?;
                self.flyout_owners.remove(&previous);
            }
            match handle {
                Handle::Generated(GeneratedHandle::Button(control)) => {
                    control.value.SetFlyout(None::<&native::FlyoutBase>)?;
                }
                Handle::Generated(GeneratedHandle::SplitButton(control)) => {
                    control.value.SetFlyout(None::<&native::FlyoutBase>)?;
                }
                _ => return Err(WinUiError::InvalidObject(target)),
            }
        }
        Ok(())
    }

    fn set_menu(
        &mut self,
        target: ObjectId,
        menu: Option<&Menu>,
        revision: u64,
    ) -> Result<(), WinUiError> {
        let kind = self.kind(target)?;
        if let Some(menu) = menu
            && let Some(current) = self.menus.get_mut(&target)
            && current.revision == revision
        {
            current.menu.clone_from(menu);
            return Ok(());
        }
        if let Some(previous) = self.menus.remove(&target) {
            if let Some(flyout) = &previous.flyout {
                flyout.cast::<native::IFlyoutBase>()?.Hide()?;
            }
            match kind {
                ObjectType::Button | ObjectType::DropDownButton => self
                    .ui_element(target)?
                    .cast::<native::IButton>()?
                    .SetFlyout(None::<&native::FlyoutBase>)?,
                ObjectType::MenuBarItem => {
                    let Some(Handle::Generated(GeneratedHandle::MenuBarItem(item))) =
                        self.handles.get(&target)
                    else {
                        return Err(WinUiError::InvalidObject(target));
                    };
                    let peer = native::FrameworkElementAutomationPeer::CreatePeerForElement(item)?;
                    let provider = peer
                        .GetPattern(native::PatternInterface::ExpandCollapse)?
                        .cast::<native::IExpandCollapseProvider>()?;
                    if provider.ExpandCollapseState()? == native::ExpandCollapseState::Expanded {
                        provider.Collapse()?;
                    }
                    // WinUI's MenuBarItem mirrors item removals, but ignores vector resets.
                    let items = item.Items()?;
                    while items.Size()? != 0 {
                        items.RemoveAtEnd()?;
                    }
                }
                _ => return Err(WinUiError::InvalidObject(target)),
            }
            drop(previous);
        }
        let Some(menu) = menu else {
            return Ok(());
        };
        let mut revokers = Vec::new();
        let flyout = match kind {
            ObjectType::Button | ObjectType::DropDownButton => {
                let flyout = native::MenuFlyout::new()?;
                build_menu_items(
                    &menu.items,
                    &flyout.Items()?,
                    &mut revokers,
                    &self.event_queue,
                    target,
                    revision,
                )?;
                self.ui_element(target)?
                    .cast::<native::IButton>()?
                    .SetFlyout(&flyout)?;
                Some(flyout)
            }
            ObjectType::MenuBarItem => {
                let Some(Handle::Generated(GeneratedHandle::MenuBarItem(item))) =
                    self.handles.get(&target)
                else {
                    return Err(WinUiError::InvalidObject(target));
                };
                build_menu_items(
                    &menu.items,
                    &item.Items()?,
                    &mut revokers,
                    &self.event_queue,
                    target,
                    revision,
                )?;
                None
            }
            _ => return Err(WinUiError::InvalidObject(target)),
        };
        self.menus.insert(
            target,
            NativeMenu {
                menu: menu.clone(),
                revision,
                flyout,
                _revokers: revokers,
            },
        );
        Ok(())
    }

    fn set_command_bar_flyout(
        &mut self,
        target: ObjectId,
        flyout: Option<&CommandBarFlyout>,
        revision: u64,
    ) -> Result<(), WinUiError> {
        if self.kind(target)? != ObjectType::Button {
            return Err(WinUiError::InvalidObject(target));
        }
        if let Some(flyout) = flyout
            && let Some(current) = self.command_bar_flyouts.get_mut(&target)
            && current.revision == revision
        {
            current.flyout.clone_from(flyout);
            return Ok(());
        }
        if let Some(previous) = self.command_bar_flyouts.remove(&target) {
            previous.native.cast::<native::IFlyoutBase>()?.Hide()?;
            self.ui_element(target)?
                .cast::<native::IButton>()?
                .SetFlyout(None::<&native::FlyoutBase>)?;
            self.released_images.extend(previous.images);
        }
        let Some(flyout) = flyout else {
            return Ok(());
        };
        let native = native::CommandBarFlyout::new()?;
        let primary = native.PrimaryCommands()?;
        let secondary = native.SecondaryCommands()?;
        let mut revokers = Vec::new();
        let mut images = Vec::new();
        for command in flyout.primary.iter() {
            primary.Append(&self.build_command_bar_element(
                command,
                &mut revokers,
                &mut images,
                target,
                revision,
            )?)?;
        }
        for command in flyout.secondary.iter() {
            secondary.Append(&self.build_command_bar_element(
                command,
                &mut revokers,
                &mut images,
                target,
                revision,
            )?)?;
        }
        self.ui_element(target)?
            .cast::<native::IButton>()?
            .SetFlyout(&native)?;
        self.command_bar_flyouts.insert(
            target,
            NativeCommandBarFlyout {
                flyout: flyout.clone(),
                revision,
                native,
                _revokers: revokers,
                images,
            },
        );
        Ok(())
    }

    fn build_command_bar_element(
        &mut self,
        command: &CommandBarCommand,
        revokers: &mut Vec<windows_core::EventRevoker>,
        images: &mut Vec<Rc<NativeImageSource>>,
        target: ObjectId,
        revision: u64,
    ) -> Result<native::ICommandBarElement, WinUiError> {
        match command {
            CommandBarCommand::Button {
                key,
                label,
                icon,
                enabled,
            } => {
                let button = native::AppBarButton::new()?;
                button.SetLabel(label)?;
                button.cast::<native::IControl>()?.SetIsEnabled(*enabled)?;
                if let Some(icon) = icon {
                    let image = self.shared_icon_image(icon)?;
                    button.SetIcon(&icon_element(icon, image.as_ref().map(|image| &image.value))?)?;
                    images.extend(image);
                }
                let clicked_key = key.clone();
                let clicked_queue = Rc::clone(&self.event_queue);
                revokers.push(button.cast::<native::IButtonBase>()?.Click(move |_, _| {
                    clicked_queue.queue(
                        None,
                        Some(QueuedEvent {
                            object: target,
                            event: EventId::CommandInvoked,
                            handler: QueuedHandler::Presentation(revision),
                            payload: EventPayload::Key(clicked_key.clone()),
                        }),
                    );
                    Self::schedule_event_wake(&clicked_queue);
                })?);
                Ok(button.cast()?)
            }
            CommandBarCommand::Separator { .. } => Ok(native::AppBarSeparator::new()?.cast()?),
        }
    }

    fn set_content_dialog(
        &mut self,
        owner: ObjectId,
        dialog: Option<ObjectId>,
    ) -> Result<(), WinUiError> {
        if self.content_dialog_owners.contains_key(&owner) {
            return Err(WinUiError::StillOwned(owner));
        }
        if let Some(dialog) = dialog {
            if owner == dialog || self.kind(dialog)? != ObjectType::ContentDialog {
                return Err(WinUiError::InvalidObject(dialog));
            }
            if self.owners.contains_key(&dialog)
                || self
                    .content_dialog_owners
                    .get(&dialog)
                    .is_some_and(|current| *current != owner)
            {
                return Err(WinUiError::StillOwned(dialog));
            }
            let owner_element = self.ui_element(owner)?;
            let framework = owner_element.cast::<native::IFrameworkElement>()?;
            let value = self
                .event_queue
                .content_dialogs
                .borrow()
                .dialogs
                .get(&dialog)
                .ok_or(WinUiError::MissingObject(dialog))?
                .value
                .clone();
            let dialog_element = value.cast::<native::IFrameworkElement>()?;
            let themed_dialog = dialog_element.clone();
            let theme_changed = framework.ActualThemeChanged(move |sender, _| {
                let Some(sender) = sender.as_ref() else {
                    return;
                };
                themed_dialog
                    .SetRequestedTheme(sender.ActualTheme().unwrap())
                    .unwrap();
            })?;
            dialog_element.SetRequestedTheme(framework.ActualTheme()?)?;
            {
                let mut scheduler = self.event_queue.content_dialogs.borrow_mut();
                let state = scheduler
                    .dialogs
                    .get_mut(&dialog)
                    .ok_or(WinUiError::MissingObject(dialog))?;
                state.owner = Some(owner);
                state.theme_changed = Some(theme_changed);
            }
            match owner_element.XamlRoot() {
                Ok(root) => {
                    let mut scheduler = self.event_queue.content_dialogs.borrow_mut();
                    scheduler.set_root(dialog, root)?;
                    scheduler.start_next_for_dialog(dialog)?;
                }
                Err(error) if error.code().is_ok() => {
                    let event_queue = Rc::clone(&self.event_queue);
                    let loaded_element = owner_element;
                    let loaded = framework.Loaded(move |_, _| {
                        let root = loaded_element.XamlRoot().unwrap();
                        let mut scheduler = event_queue.content_dialogs.borrow_mut();
                        scheduler.set_root(dialog, root).unwrap();
                        scheduler.start_next_for_dialog(dialog).unwrap();
                    })?;
                    self.event_queue
                        .content_dialogs
                        .borrow_mut()
                        .dialogs
                        .get_mut(&dialog)
                        .unwrap()
                        .loaded = Some(loaded);
                }
                Err(error) => return Err(error.into()),
            }
        }
        if dialog.is_none()
            && let Some(previous) = self.content_dialogs.get(&owner)
            && self
                .event_queue
                .content_dialogs
                .borrow()
                .dialogs
                .get(previous)
                .is_some_and(|state| state.desired_open)
        {
            return Err(WinUiError::StillOwned(*previous));
        }
        if let Some(previous) = self.content_dialogs.remove(&owner) {
            self.content_dialog_owners.remove(&previous);
            let mut scheduler = self.event_queue.content_dialogs.borrow_mut();
            if let Some(state) = scheduler.dialogs.get_mut(&previous) {
                state.owner = None;
                state.loaded = None;
                state.theme_changed = None;
                if state.pending_generation.is_none() {
                    state.root = None;
                }
            }
        }
        if let Some(dialog) = dialog {
            self.content_dialogs.insert(owner, dialog);
            self.content_dialog_owners.insert(dialog, owner);
        }
        Ok(())
    }

    fn set_content_dialog_open(&mut self, dialog: ObjectId, open: bool) -> Result<(), WinUiError> {
        if !self.content_dialog_owners.contains_key(&dialog) {
            return Err(WinUiError::InvalidObject(dialog));
        }
        let hide = self
            .event_queue
            .content_dialogs
            .borrow_mut()
            .set_open(dialog, open)?;
        if let Some(value) = hide {
            value.Hide()?;
        }
        Ok(())
    }

    fn content_dialog_closed(
        event_queue: &Rc<NativeEventQueue>,
        dialog: ObjectId,
    ) -> Result<bool, WinUiError> {
        let generation = event_queue
            .content_dialogs
            .borrow()
            .dialogs
            .get(&dialog)
            .and_then(|state| state.pending_generation);
        let Some(generation) = generation else {
            return Ok(false);
        };
        let (dispatch, root) = event_queue
            .content_dialogs
            .borrow_mut()
            .closed(dialog, generation)?;
        let queue = native::DispatcherQueue::GetForCurrentThread()?;
        let deferred_queue = Rc::clone(event_queue);
        let handler = native::DispatcherQueueHandler::new(move || {
            let mut scheduler = deferred_queue.content_dialogs.borrow_mut();
            scheduler.cleanup_retired();
            if let Some(root) = &root {
                scheduler.start_next(root).unwrap();
            }
        });
        // A dispatcher that is shutting down rejects the continuation; no dialog will show again.
        queue.TryEnqueueWithPriority(native::DispatcherQueuePriority::Normal, &handler)?;
        Ok(dispatch)
    }

    fn sync_window_title_bars(&self) -> Result<(), WinUiError> {
        for window in self.live_windows() {
            let desired = self
                .window_title_bar
                .filter(|(object, _)| self.owns(window.root, *object));
            if window.title_bar.get() == desired {
                continue;
            }
            if let Some((object, _)) = window.title_bar.get() {
                Self::clear_title_bar_from_window(&window, object)?;
            }
            if let Some((object, height)) = desired {
                self.apply_title_bar_to_window(&window, object, height)?;
            }
        }
        Ok(())
    }

    fn apply_window_policy(
        &self,
        window: &native::Window,
        root: &native::UIElement,
        policy: &WindowPolicy,
    ) -> Result<(), WinUiError> {
        if let Some(title) = &policy.title {
            window.SetTitle(title)?;
        }

        let window_2 = window.cast::<native::IWindow2>()?;
        let app_window = window_2.AppWindow()?;
        let title_bar = app_window.TitleBar()?;
        title_bar
            .cast::<native::IAppWindowTitleBar3>()?
            .SetPreferredTheme(match policy.theme {
                WindowTheme::System => native::TitleBarTheme::UseDefaultAppMode,
                WindowTheme::Light => native::TitleBarTheme::Light,
                WindowTheme::Dark => native::TitleBarTheme::Dark,
            })?;
        root.cast::<native::FrameworkElement>()?
            .SetRequestedTheme(match policy.theme {
                WindowTheme::System => native::ElementTheme::Default,
                WindowTheme::Light => native::ElementTheme::Light,
                WindowTheme::Dark => native::ElementTheme::Dark,
            })?;

        let needs_metrics = policy.client_size.is_some() || policy.minimum_client_size.is_some();
        if needs_metrics {
            let mut hwnd = std::ptr::null_mut();
            unsafe {
                window
                    .cast::<native::IWindowNative>()?
                    .WindowHandle(&mut hwnd)
                    .ok()?;
            }
            let dpi = unsafe { native::GetDpiForWindow(hwnd.cast()) }.max(96);
            let pixels = |dips: f64| (dips * f64::from(dpi) / 96.0).round() as i32;
            let client_window = app_window.cast::<native::IAppWindow2>()?;

            if let Some((width, height)) = policy.minimum_client_size {
                let outer = app_window.Size()?;
                let inner = client_window.ClientSize()?;
                let non_client_width = outer.width.saturating_sub(inner.width);
                let non_client_height = outer.height.saturating_sub(inner.height);
                let presenter = app_window
                    .Presenter()?
                    .cast::<native::IOverlappedPresenter3>()?;
                presenter.SetPreferredMinimumWidth(Some(
                    pixels(width).saturating_add(non_client_width),
                ))?;
                presenter.SetPreferredMinimumHeight(Some(
                    pixels(height).saturating_add(non_client_height),
                ))?;
            }

            if let Some((width, height)) = policy.client_size {
                resize_window_client(
                    &app_window,
                    native::SizeInt32 {
                        width: pixels(width),
                        height: pixels(height),
                    },
                )?;
            }
        }
        Ok(())
    }
}
