use super::*;

pub(super) struct ScalarCallbackState<T: Copy> {
    expected: Cell<T>,
    suppressing: Cell<bool>,
}

pub(super) struct ListCallbackState<T> {
    expected: RefCell<T>,
    suppressing: Cell<bool>,
}

fn selected_date(picker: &bindings::IDatePicker) -> WindowsResult<Option<DateTime>> {
    unsafe {
        let mut result = std::ptr::null_mut();
        (Interface::vtable(picker).SelectedDate)(Interface::as_raw(picker), &mut result).ok()?;
        if result.is_null() {
            return Ok(None);
        }
        let reference: windows_reference::IReference<DateTime> =
            windows_core::Type::from_abi(result)?;
        Ok(Some(reference.Value()?))
    }
}

fn calendar_date(picker: &bindings::ICalendarDatePicker) -> WindowsResult<Option<DateTime>> {
    unsafe {
        let mut result = std::ptr::null_mut();
        (Interface::vtable(picker).Date)(Interface::as_raw(picker), &mut result).ok()?;
        if result.is_null() {
            return Ok(None);
        }
        let reference: windows_reference::IReference<DateTime> =
            windows_core::Type::from_abi(result)?;
        Ok(Some(reference.Value()?))
    }
}

fn selected_time(picker: &bindings::ITimePicker) -> WindowsResult<Option<TimeSpan>> {
    unsafe {
        let mut result = std::ptr::null_mut();
        (Interface::vtable(picker).SelectedTime)(Interface::as_raw(picker), &mut result).ok()?;
        if result.is_null() {
            return Ok(None);
        }
        let reference: windows_reference::IReference<TimeSpan> =
            windows_core::Type::from_abi(result)?;
        Ok(Some(reference.Value()?))
    }
}

fn selected_dates(view: &bindings::ICalendarView) -> WindowsResult<Vec<DateTime>> {
    let dates = view.SelectedDates()?;
    let mut values = Vec::with_capacity(dates.Size()? as usize);
    for index in 0..dates.Size()? {
        values.push(dates.GetAt(index)?);
    }
    values.sort_unstable();
    values.dedup();
    Ok(values)
}

fn optional_number(value: f64) -> Option<f64> {
    (!value.is_nan()).then_some(value)
}

fn optional_rating(value: f64) -> Option<f64> {
    (value != -1.0).then_some(value)
}

fn native_color(value: Color) -> bindings::Color {
    bindings::Color {
        a: value.a,
        r: value.r,
        g: value.g,
        b: value.b,
    }
}

fn public_color(value: bindings::Color) -> Color {
    Color {
        a: value.a,
        r: value.r,
        g: value.g,
        b: value.b,
    }
}

pub(super) fn inspectable_text(value: &str) -> windows_core::IInspectable {
    windows_reference::IReference::<windows_core::HSTRING>::from(windows_core::HSTRING::from(value))
        .into()
}

impl WinUiRuntime {
    pub(super) fn apply_control_chrome(
        &self,
        id: NodeId,
        update: &ControlChromeUpdate,
    ) -> WindowsResult<()> {
        let handle = &self.node(id)?.handle;
        let control = handle.control()?;
        let dependency = handle.dependency_object()?;

        if let Some(value) = &update.background {
            control.SetBackground(&native_brush(value)?)?;
        } else {
            dependency.ClearValue(&bindings::Control::BackgroundProperty()?)?;
        }
        if let Some(value) = &update.border_brush {
            control.SetBorderBrush(&native_brush(value)?)?;
        } else {
            dependency.ClearValue(&bindings::Control::BorderBrushProperty()?)?;
        }
        if let Some(value) = update.border_thickness {
            control.SetBorderThickness(native_thickness(value))?;
        } else {
            dependency.ClearValue(&bindings::Control::BorderThicknessProperty()?)?;
        }
        Ok(())
    }

    pub(super) fn create_check_box(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::CheckBox::new()?;
        let expected = Rc::new(Cell::new(false));
        let toggle: bindings::IToggleButton = value.cast()?;
        let revokers = subscribe_toggle(
            &toggle,
            id,
            &expected,
            Rc::clone(&self.events),
            Rc::clone(&self.waker),
        )?;
        Ok(Handle::CheckBox {
            _revokers: revokers,
            expected,
            value,
        })
    }

    pub(super) fn create_radio_button(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::RadioButton::new()?;
        let expected = Rc::new(Cell::new(false));
        let toggle: bindings::IToggleButton = value.cast()?;
        let revokers = subscribe_toggle(
            &toggle,
            id,
            &expected,
            Rc::clone(&self.events),
            Rc::clone(&self.waker),
        )?;
        Ok(Handle::RadioButton {
            _revokers: revokers,
            expected,
            value,
        })
    }

    pub(super) fn create_toggle_button(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::ToggleButton::new()?;
        let expected = Rc::new(Cell::new(false));
        let toggle: bindings::IToggleButton = value.cast()?;
        let [checked, unchecked] = subscribe_toggle(
            &toggle,
            id,
            &expected,
            Rc::clone(&self.events),
            Rc::clone(&self.waker),
        )?;
        let button_base: bindings::IButtonBase = value.cast()?;
        let click = subscribe_click(
            &button_base,
            id,
            Rc::clone(&self.events),
            Rc::clone(&self.waker),
        )?;
        Ok(Handle::ToggleButton {
            _revokers: Box::new([click, checked, unchecked]),
            expected,
            value,
        })
    }

    pub(super) fn create_toggle_switch(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::ToggleSwitch::new()?;
        let expected = Rc::new(Cell::new(false));
        let toggle: bindings::IToggleSwitch = value.cast()?;
        let revoker = subscribe_toggle_switch(
            &toggle,
            id,
            &expected,
            Rc::clone(&self.events),
            Rc::clone(&self.waker),
        )?;
        Ok(Handle::ToggleSwitch {
            _revoker: revoker,
            expected,
            value,
        })
    }

    pub(super) fn create_slider(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::Slider::new()?;
        let range: bindings::IRangeBase = value.cast()?;
        let state = Rc::new(ScalarCallbackState {
            expected: Cell::new(0.0),
            suppressing: Cell::new(false),
        });
        let event_state = Rc::clone(&state);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = range.ValueChanged(move |_sender, args| {
            if event_state.suppressing.get() {
                return;
            }
            let value = args.as_ref().unwrap().NewValue().unwrap();
            if event_state.expected.get() == value {
                return;
            }
            event_state.expected.set(value);
            events
                .borrow_mut()
                .push_back(NativeEvent::ValueChanged { target: id, value });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::Slider {
            _revoker: revoker,
            state,
            value,
        })
    }

    pub(super) fn create_number_box(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::NumberBox::new()?;
        let number: bindings::INumberBox = value.cast()?;
        let state = Rc::new(ScalarCallbackState {
            expected: Cell::new(Some(0.0)),
            suppressing: Cell::new(false),
        });
        let event_state = Rc::clone(&state);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = number.ValueChanged(move |_sender, args| {
            if event_state.suppressing.get() {
                return;
            }
            let value = optional_number(args.as_ref().unwrap().NewValue().unwrap());
            if event_state.expected.get() == value {
                return;
            }
            event_state.expected.set(value);
            events
                .borrow_mut()
                .push_back(NativeEvent::OptionalValueChanged { target: id, value });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::NumberBox {
            _revoker: revoker,
            state,
            value,
        })
    }

    pub(super) fn create_rating_control(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::RatingControl::new()?;
        let rating: bindings::IRatingControl = value.cast()?;
        let state = Rc::new(ScalarCallbackState {
            expected: Cell::new(optional_rating(rating.Value()?)),
            suppressing: Cell::new(false),
        });
        let event_state = Rc::clone(&state);
        let event_rating = rating.clone();
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = rating.ValueChanged(move |_sender, _args| {
            if event_state.suppressing.get() {
                return;
            }
            let value = optional_rating(event_rating.Value().unwrap());
            if event_state.expected.get() == value {
                return;
            }
            event_state.expected.set(value);
            events
                .borrow_mut()
                .push_back(NativeEvent::OptionalValueChanged { target: id, value });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::RatingControl {
            _revoker: revoker,
            state,
            value,
        })
    }

    pub(super) fn create_color_picker(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::ColorPicker::new()?;
        let picker: bindings::IColorPicker = value.cast()?;
        let expected = Rc::new(Cell::new(public_color(picker.Color()?)));
        let event_expected = Rc::clone(&expected);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = picker.ColorChanged(move |_sender, args| {
            let value = public_color(args.as_ref().unwrap().NewColor().unwrap());
            if event_expected.replace(value) == value {
                return;
            }
            events
                .borrow_mut()
                .push_back(NativeEvent::ColorChanged { target: id, value });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::ColorPicker {
            _revoker: revoker,
            expected,
            value,
        })
    }

    pub(super) fn create_date_picker(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::DatePicker::new()?;
        let picker: bindings::IDatePicker = value.cast()?;
        let expected = Rc::new(Cell::new(selected_date(&picker)?));
        let event_expected = Rc::clone(&expected);
        let event_picker = picker.clone();
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = picker.SelectedDateChanged(move |_sender, _args| {
            let value = selected_date(&event_picker).unwrap();
            if event_expected.replace(value) == value {
                return;
            }

            events
                .borrow_mut()
                .push_back(NativeEvent::DateChanged { target: id, value });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::DatePicker {
            _revoker: revoker,
            expected,
            value,
        })
    }

    pub(super) fn create_calendar_date_picker(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::CalendarDatePicker::new()?;
        let picker: bindings::ICalendarDatePicker = value.cast()?;
        let expected = Rc::new(Cell::new(calendar_date(&picker)?));
        let event_expected = Rc::clone(&expected);
        let event_picker = picker.clone();
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = picker.DateChanged(move |_sender, _args| {
            let value = calendar_date(&event_picker).unwrap();
            if event_expected.replace(value) == value {
                return;
            }
            events
                .borrow_mut()
                .push_back(NativeEvent::DateChanged { target: id, value });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::CalendarDatePicker {
            _revoker: revoker,
            expected,
            value,
        })
    }

    pub(super) fn create_time_picker(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::TimePicker::new()?;
        let picker: bindings::ITimePicker = value.cast()?;
        let expected = Rc::new(Cell::new(selected_time(&picker)?));
        let event_expected = Rc::clone(&expected);
        let event_picker = picker.clone();
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = picker.SelectedTimeChanged(move |_sender, _args| {
            let value = selected_time(&event_picker).unwrap();
            if event_expected.replace(value) == value {
                return;
            }
            events
                .borrow_mut()
                .push_back(NativeEvent::TimeChanged { target: id, value });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::TimePicker {
            _revoker: revoker,
            expected,
            value,
        })
    }

    pub(super) fn create_calendar_view(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::CalendarView::new()?;
        let view: bindings::ICalendarView = value.cast()?;
        let state = Rc::new(ListCallbackState {
            expected: RefCell::new(selected_dates(&view)?),
            suppressing: Cell::new(false),
        });
        let event_state = Rc::clone(&state);
        let event_view = view.clone();
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = view.SelectedDatesChanged(move |_sender, _args| {
            if event_state.suppressing.get() {
                return;
            }
            let value = selected_dates(&event_view).unwrap();
            if *event_state.expected.borrow() == value {
                return;
            }
            event_state.expected.borrow_mut().clone_from(&value);
            events
                .borrow_mut()
                .push_back(NativeEvent::DatesChanged { target: id, value });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::CalendarView {
            _revoker: revoker,
            state,
            value,
        })
    }

    pub(super) fn create_text_box(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::TextBox::new()?;
        let expected = Rc::new(RefCell::new(String::new()));
        let control = value.clone();
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let event_expected = Rc::clone(&expected);
        let revoker = value.TextChanged(move |_sender, _args| {
            let value = control.Text().unwrap();
            if *event_expected.borrow() == value {
                return;
            }
            event_expected.borrow_mut().clone_from(&value);
            events
                .borrow_mut()
                .push_back(NativeEvent::TextChanged { target: id, value });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::TextBox {
            _revoker: revoker,
            expected,
            value,
        })
    }

    pub(super) fn create_password_box(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::PasswordBox::new()?;
        let expected = Rc::new(RefCell::new(String::new()));
        let control = value.clone();
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let event_expected = Rc::clone(&expected);
        let revoker = value.PasswordChanged(move |_sender, _args| {
            let value = control.Password().unwrap();
            if *event_expected.borrow() == value {
                return;
            }
            event_expected.borrow_mut().clone_from(&value);
            events
                .borrow_mut()
                .push_back(NativeEvent::PasswordChanged { target: id, value });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::PasswordBox {
            _revoker: revoker,
            expected,
            value,
        })
    }

    pub(super) fn apply_text_box_update(
        &self,
        id: NodeId,
        update: &TextBoxUpdate,
    ) -> WindowsResult<()> {
        match update {
            TextBoxUpdate::Text(value) => self.apply_text_box_text(id, value),
            TextBoxUpdate::Header(value) => self.apply_text_box_header(id, value),
            TextBoxUpdate::Placeholder(value) => self.apply_text_box_placeholder(id, value),
            TextBoxUpdate::AcceptsReturn(value) => self.apply_text_box_accepts_return(id, *value),
            TextBoxUpdate::Chrome(update) => self.apply_control_chrome(id, update),
        }
    }

    pub(super) fn apply_password_box_update(
        &self,
        id: NodeId,
        update: &PasswordBoxUpdate,
    ) -> WindowsResult<()> {
        match update {
            PasswordBoxUpdate::Password(value) => self.apply_password_box_password(id, value),
            PasswordBoxUpdate::Header(value) => self.apply_password_box_header(id, value),
            PasswordBoxUpdate::Placeholder(value) => self.apply_password_box_placeholder(id, value),
            PasswordBoxUpdate::RevealMode(value) => self.apply_password_box_reveal_mode(id, *value),
        }
    }

    pub(super) fn apply_text_box_text(&self, id: NodeId, text: &str) -> WindowsResult<()> {
        let Handle::TextBox {
            value, expected, ..
        } = &self.node(id)?.handle
        else {
            panic!("text target is not a TextBox");
        };
        let previous = expected.replace(text.to_owned());
        let result = value.SetText(text);
        if result.is_err() {
            expected.replace(previous);
        }
        result
    }

    pub(super) fn apply_text_box_header(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        let Handle::TextBox { value: text, .. } = &self.node(id)?.handle else {
            panic!("header update target is not a TextBox");
        };
        let header = value.as_deref().map(inspectable_text);
        text.SetHeader(header.as_ref())
    }

    pub(super) fn apply_text_box_placeholder(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        let Handle::TextBox { value: text, .. } = &self.node(id)?.handle else {
            panic!("placeholder update target is not a TextBox");
        };
        text.SetPlaceholderText(value.as_deref().unwrap_or_default())
    }

    pub(super) fn apply_text_box_accepts_return(
        &self,
        id: NodeId,
        value: bool,
    ) -> WindowsResult<()> {
        let Handle::TextBox { value: text, .. } = &self.node(id)?.handle else {
            panic!("accepts-return update target is not a TextBox");
        };
        text.SetAcceptsReturn(value)
    }

    pub(super) fn apply_password_box_password(
        &self,
        id: NodeId,
        password: &str,
    ) -> WindowsResult<()> {
        let Handle::PasswordBox {
            value, expected, ..
        } = &self.node(id)?.handle
        else {
            panic!("password target is not a PasswordBox");
        };
        let previous = expected.replace(password.to_owned());
        let result = value.SetPassword(password);
        if result.is_err() {
            expected.replace(previous);
        }
        result
    }

    pub(super) fn apply_password_box_header(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        let Handle::PasswordBox {
            value: password, ..
        } = &self.node(id)?.handle
        else {
            panic!("header update target is not a PasswordBox");
        };
        let header = value.as_deref().map(inspectable_text);
        password.SetHeader(header.as_ref())
    }

    pub(super) fn apply_password_box_placeholder(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        let Handle::PasswordBox {
            value: password, ..
        } = &self.node(id)?.handle
        else {
            panic!("placeholder update target is not a PasswordBox");
        };
        password.SetPlaceholderText(value.as_deref().unwrap_or_default())
    }

    pub(super) fn apply_password_box_reveal_mode(
        &self,
        id: NodeId,
        value: PasswordRevealMode,
    ) -> WindowsResult<()> {
        let Handle::PasswordBox {
            value: password, ..
        } = &self.node(id)?.handle
        else {
            panic!("reveal-mode update target is not a PasswordBox");
        };
        password.SetPasswordRevealMode(native_password_reveal_mode(value))
    }

    pub(super) fn apply_toggle_checked(&self, id: NodeId, value: bool) -> WindowsResult<()> {
        let (toggle, expected) = match &self.node(id)?.handle {
            Handle::CheckBox {
                value, expected, ..
            } => (value.cast::<bindings::IToggleButton>(), expected),
            Handle::RadioButton {
                value, expected, ..
            } => (value.cast::<bindings::IToggleButton>(), expected),
            Handle::ToggleButton {
                value, expected, ..
            } => (value.cast::<bindings::IToggleButton>(), expected),
            _ => {
                panic!("checked target does not implement ToggleButton");
            }
        };
        let previous = expected.replace(value);
        let result = toggle.and_then(|toggle| toggle.SetIsChecked(Some(value)));
        if result.is_err() {
            expected.set(previous);
        }
        result
    }

    pub(super) fn apply_toggle_switch_update(
        &self,
        id: NodeId,
        update: &ToggleSwitchUpdate,
    ) -> WindowsResult<()> {
        match update {
            ToggleSwitchUpdate::On(value) => self.apply_toggle_switch_on(id, *value),
            ToggleSwitchUpdate::Content(update) => self.apply_toggle_switch_content(id, update),
        }
    }

    pub(super) fn apply_toggle_switch_on(&self, id: NodeId, on: bool) -> WindowsResult<()> {
        let Handle::ToggleSwitch {
            value, expected, ..
        } = &self.node(id)?.handle
        else {
            panic!("set on target is not a ToggleSwitch");
        };
        let previous = expected.replace(on);
        let result = value
            .cast::<bindings::IToggleSwitch>()
            .and_then(|toggle| toggle.SetIsOn(on));
        if result.is_err() {
            expected.set(previous);
        }
        result
    }

    pub(super) fn apply_toggle_switch_content(
        &self,
        id: NodeId,
        update: &ToggleSwitchContentUpdate,
    ) -> WindowsResult<()> {
        let Handle::ToggleSwitch { value, .. } = &self.node(id)?.handle else {
            panic!("content target is not a ToggleSwitch");
        };
        let control: bindings::IToggleSwitch = value.cast()?;
        let header = update.header.as_deref().map(inspectable_text);
        control.SetHeader(header.as_ref())?;
        let dependency: bindings::IDependencyObject = value.cast()?;
        if let Some(content) = update.on_content.as_deref() {
            control.SetOnContent(&inspectable_text(content))?;
        } else {
            dependency.ClearValue(&bindings::ToggleSwitch::OnContentProperty()?)?;
        }
        if let Some(content) = update.off_content.as_deref() {
            control.SetOffContent(&inspectable_text(content))?;
        } else {
            dependency.ClearValue(&bindings::ToggleSwitch::OffContentProperty()?)?;
        }
        Ok(())
    }

    pub(super) fn apply_slider_update(
        &self,
        id: NodeId,
        update: &SliderUpdate,
    ) -> WindowsResult<()> {
        match update {
            SliderUpdate::Range(value) => self.apply_slider_range(id, *value),
            SliderUpdate::Orientation(value) => self.apply_slider_orientation(id, *value),
            SliderUpdate::Step(value) => self.apply_slider_step(id, *value),
            SliderUpdate::Header(value) => self.apply_slider_header(id, value),
        }
    }

    pub(super) fn apply_slider_range(&self, id: NodeId, range: RangeState) -> WindowsResult<()> {
        let Handle::Slider { value, state, .. } = &self.node(id)?.handle else {
            panic!("range update target is not a Slider");
        };
        let previous = state.expected.replace(range.value);
        state.suppressing.set(true);
        let result = (|| {
            let native: bindings::IRangeBase = value.cast()?;
            let current_maximum = native.Maximum()?;
            if range.minimum > current_maximum {
                native.SetMaximum(range.maximum)?;
                native.SetMinimum(range.minimum)?;
            } else {
                native.SetMinimum(range.minimum)?;
                native.SetMaximum(range.maximum)?;
            }
            native.SetValue(range.value)
        })();
        state.suppressing.set(false);
        if result.is_err() {
            state.expected.set(previous);
        }
        result
    }

    pub(super) fn apply_slider_orientation(
        &self,
        id: NodeId,
        value: Orientation,
    ) -> WindowsResult<()> {
        let Handle::Slider { value: slider, .. } = &self.node(id)?.handle else {
            panic!("orientation update target is not a Slider");
        };
        slider
            .cast::<bindings::ISlider>()
            .and_then(|slider| slider.SetOrientation(native_orientation(value)))
    }

    pub(super) fn apply_slider_step(&self, id: NodeId, value: f64) -> WindowsResult<()> {
        let Handle::Slider { value: slider, .. } = &self.node(id)?.handle else {
            panic!("step update target is not a Slider");
        };
        slider.cast::<bindings::ISlider>()?.SetStepFrequency(value)
    }

    pub(super) fn apply_slider_header(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        let Handle::Slider { value: slider, .. } = &self.node(id)?.handle else {
            panic!("header update target is not a Slider");
        };
        let header = value.as_deref().map(inspectable_text);
        slider
            .cast::<bindings::ISlider>()?
            .SetHeader(header.as_ref())
    }

    pub(super) fn apply_number_box_update(
        &self,
        id: NodeId,
        update: &NumberBoxUpdate,
    ) -> WindowsResult<()> {
        match update {
            NumberBoxUpdate::Bounds { minimum, maximum } => {
                self.apply_number_box_bounds(id, *minimum, *maximum)
            }
            NumberBoxUpdate::Value(value) => self.apply_number_box_value(id, *value),
            NumberBoxUpdate::Header(value) => self.apply_number_box_header(id, value),
        }
    }

    pub(super) fn apply_number_box_bounds(
        &self,
        id: NodeId,
        minimum: f64,
        maximum: f64,
    ) -> WindowsResult<()> {
        let Handle::NumberBox { value, state, .. } = &self.node(id)?.handle else {
            panic!("bounds update target is not a NumberBox");
        };
        state.suppressing.set(true);
        let result = (|| {
            let number: bindings::INumberBox = value.cast()?;
            let current_maximum = number.Maximum()?;
            if minimum > current_maximum {
                number.SetMaximum(maximum)?;
                number.SetMinimum(minimum)
            } else {
                number.SetMinimum(minimum)?;
                number.SetMaximum(maximum)
            }
        })();
        state.suppressing.set(false);
        result
    }

    pub(super) fn apply_number_box_header(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        let Handle::NumberBox { value: number, .. } = &self.node(id)?.handle else {
            panic!("header update target is not a NumberBox");
        };
        let header = value.as_deref().map(inspectable_text);
        number
            .cast::<bindings::INumberBox>()?
            .SetHeader(header.as_ref())
    }

    pub(super) fn apply_number_box_value(
        &self,
        id: NodeId,
        value: Option<f64>,
    ) -> WindowsResult<()> {
        let Handle::NumberBox {
            value: number,
            state,
            ..
        } = &self.node(id)?.handle
        else {
            panic!("value update target is not a NumberBox");
        };
        let previous = state.expected.replace(value);
        state.suppressing.set(true);
        let result = number
            .cast::<bindings::INumberBox>()
            .and_then(|number| number.SetValue(value.unwrap_or(f64::NAN)));
        state.suppressing.set(false);
        if result.is_err() {
            state.expected.set(previous);
        }
        result
    }

    pub(super) fn apply_rating_control_update(
        &self,
        id: NodeId,
        update: &RatingControlUpdate,
    ) -> WindowsResult<()> {
        match update {
            RatingControlUpdate::Max(value) => self.apply_rating_max(id, *value),
            RatingControlUpdate::Placeholder(value) => self.apply_rating_placeholder(id, *value),
            RatingControlUpdate::Caption(value) => self.apply_rating_caption(id, value),
            RatingControlUpdate::ReadOnly(value) => self.apply_rating_read_only(id, *value),
            RatingControlUpdate::Value(value) => self.apply_rating_value(id, *value),
        }
    }

    pub(super) fn apply_rating_max(&self, id: NodeId, value: i32) -> WindowsResult<()> {
        let Handle::RatingControl {
            value: rating,
            state,
            ..
        } = &self.node(id)?.handle
        else {
            panic!("maximum update target is not a RatingControl");
        };
        state.suppressing.set(true);
        let result = rating
            .cast::<bindings::IRatingControl>()
            .and_then(|rating| rating.SetMaxRating(value));
        state.suppressing.set(false);
        result
    }

    pub(super) fn apply_rating_placeholder(
        &self,
        id: NodeId,
        value: Option<f64>,
    ) -> WindowsResult<()> {
        let Handle::RatingControl {
            value: rating,
            state,
            ..
        } = &self.node(id)?.handle
        else {
            panic!("placeholder update target is not a RatingControl");
        };
        state.suppressing.set(true);
        let result = rating
            .cast::<bindings::IRatingControl>()
            .and_then(|rating| rating.SetPlaceholderValue(value.unwrap_or(-1.0)));
        state.suppressing.set(false);
        result
    }

    pub(super) fn apply_rating_caption(&self, id: NodeId, value: &str) -> WindowsResult<()> {
        let Handle::RatingControl { value: rating, .. } = &self.node(id)?.handle else {
            panic!("caption update target is not a RatingControl");
        };
        rating
            .cast::<bindings::IRatingControl>()
            .and_then(|rating| rating.SetCaption(value))
    }

    pub(super) fn apply_rating_read_only(&self, id: NodeId, value: bool) -> WindowsResult<()> {
        let Handle::RatingControl { value: rating, .. } = &self.node(id)?.handle else {
            panic!("read-only update target is not a RatingControl");
        };
        rating
            .cast::<bindings::IRatingControl>()
            .and_then(|rating| rating.SetIsReadOnly(value))
    }

    pub(super) fn apply_rating_value(&self, id: NodeId, value: Option<f64>) -> WindowsResult<()> {
        let Handle::RatingControl {
            value: rating,
            state,
            ..
        } = &self.node(id)?.handle
        else {
            panic!("value update target is not a RatingControl");
        };
        let previous = state.expected.replace(value);
        state.suppressing.set(true);
        let result = rating
            .cast::<bindings::IRatingControl>()
            .and_then(|rating| rating.SetValue(value.unwrap_or(-1.0)));
        state.suppressing.set(false);
        if result.is_err() {
            state.expected.set(previous);
        }
        result
    }

    pub(super) fn apply_color_picker_update(
        &self,
        id: NodeId,
        update: ColorPickerUpdate,
    ) -> WindowsResult<()> {
        match update {
            ColorPickerUpdate::Color(value) => self.apply_color_picker_color(id, value),
            ColorPickerUpdate::AlphaEnabled(value) => {
                self.apply_color_picker_alpha_enabled(id, value)
            }
            ColorPickerUpdate::HexInputVisible(value) => {
                self.apply_color_picker_hex_input_visible(id, value)
            }
            ColorPickerUpdate::SliderVisible(value) => {
                self.apply_color_picker_slider_visible(id, value)
            }
            ColorPickerUpdate::ChannelInputVisible(value) => {
                self.apply_color_picker_channel_input_visible(id, value)
            }
        }
    }

    pub(super) fn apply_color_picker_color(&self, id: NodeId, value: Color) -> WindowsResult<()> {
        let Handle::ColorPicker {
            value: picker,
            expected,
            ..
        } = &self.node(id)?.handle
        else {
            panic!("color update target is not a ColorPicker");
        };
        let previous = expected.replace(value);
        let result = picker
            .cast::<bindings::IColorPicker>()
            .and_then(|picker| picker.SetColor(native_color(value)));
        if result.is_err() {
            expected.set(previous);
        }
        result
    }

    pub(super) fn apply_color_picker_alpha_enabled(
        &self,
        id: NodeId,
        value: bool,
    ) -> WindowsResult<()> {
        self.color_picker(id)?.SetIsAlphaEnabled(value)
    }

    pub(super) fn apply_color_picker_hex_input_visible(
        &self,
        id: NodeId,
        value: bool,
    ) -> WindowsResult<()> {
        self.color_picker(id)?.SetIsHexInputVisible(value)
    }

    pub(super) fn apply_color_picker_slider_visible(
        &self,
        id: NodeId,
        value: bool,
    ) -> WindowsResult<()> {
        self.color_picker(id)?.SetIsColorSliderVisible(value)
    }

    pub(super) fn apply_color_picker_channel_input_visible(
        &self,
        id: NodeId,
        value: bool,
    ) -> WindowsResult<()> {
        self.color_picker(id)?
            .SetIsColorChannelTextInputVisible(value)
    }

    fn color_picker(&self, id: NodeId) -> WindowsResult<bindings::IColorPicker> {
        let Handle::ColorPicker { value, .. } = &self.node(id)?.handle else {
            panic!("property update target is not a ColorPicker");
        };
        value.cast()
    }

    pub(super) fn apply_date_picker_update(
        &self,
        id: NodeId,
        update: &DatePickerUpdate,
    ) -> WindowsResult<()> {
        match update {
            DatePickerUpdate::Date(value) => self.apply_date_picker_date(id, *value),
            DatePickerUpdate::Header(value) => self.apply_date_picker_header(id, value),
            DatePickerUpdate::DayVisible(value) => self.apply_date_picker_day_visible(id, *value),
            DatePickerUpdate::MonthVisible(value) => {
                self.apply_date_picker_month_visible(id, *value)
            }
            DatePickerUpdate::YearVisible(value) => self.apply_date_picker_year_visible(id, *value),
        }
    }

    pub(super) fn apply_date_picker_date(
        &self,
        id: NodeId,
        value: Option<DateTime>,
    ) -> WindowsResult<()> {
        let Handle::DatePicker {
            value: picker,
            expected,
            ..
        } = &self.node(id)?.handle
        else {
            panic!("date update target is not a DatePicker");
        };
        let previous = expected.replace(value);
        let result = picker
            .cast::<bindings::IDatePicker>()
            .and_then(|picker| picker.SetSelectedDate(value));
        if result.is_err() {
            expected.set(previous);
        }
        result
    }

    pub(super) fn apply_date_picker_header(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        let header = value.as_deref().map(inspectable_text);
        self.date_picker(id)?.SetHeader(header.as_ref())
    }

    pub(super) fn apply_date_picker_day_visible(
        &self,
        id: NodeId,
        value: bool,
    ) -> WindowsResult<()> {
        self.date_picker(id)?.SetDayVisible(value)
    }

    pub(super) fn apply_date_picker_month_visible(
        &self,
        id: NodeId,
        value: bool,
    ) -> WindowsResult<()> {
        self.date_picker(id)?.SetMonthVisible(value)
    }

    pub(super) fn apply_date_picker_year_visible(
        &self,
        id: NodeId,
        value: bool,
    ) -> WindowsResult<()> {
        self.date_picker(id)?.SetYearVisible(value)
    }

    fn date_picker(&self, id: NodeId) -> WindowsResult<bindings::IDatePicker> {
        let Handle::DatePicker { value, .. } = &self.node(id)?.handle else {
            panic!("property update target is not a DatePicker");
        };
        value.cast()
    }

    pub(super) fn apply_calendar_date_picker_update(
        &self,
        id: NodeId,
        update: &CalendarDatePickerUpdate,
    ) -> WindowsResult<()> {
        match update {
            CalendarDatePickerUpdate::Date(value) => {
                self.apply_calendar_date_picker_date(id, *value)
            }
            CalendarDatePickerUpdate::Header(value) => {
                self.apply_calendar_date_picker_header(id, value)
            }
            CalendarDatePickerUpdate::Placeholder(value) => {
                self.apply_calendar_date_picker_placeholder(id, value)
            }
            CalendarDatePickerUpdate::TodayHighlighted(value) => {
                self.apply_calendar_date_picker_today_highlighted(id, *value)
            }
        }
    }

    pub(super) fn apply_calendar_date_picker_date(
        &self,
        id: NodeId,
        value: Option<DateTime>,
    ) -> WindowsResult<()> {
        let Handle::CalendarDatePicker {
            value: picker,
            expected,
            ..
        } = &self.node(id)?.handle
        else {
            panic!("date update target is not a CalendarDatePicker");
        };
        let previous = expected.replace(value);
        let result = picker.SetDate(value);
        if result.is_err() {
            expected.set(previous);
        }
        result
    }

    pub(super) fn apply_calendar_date_picker_header(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        let header = value.as_deref().map(inspectable_text);
        self.calendar_date_picker(id)?.SetHeader(header.as_ref())
    }

    pub(super) fn apply_calendar_date_picker_placeholder(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        self.calendar_date_picker(id)?
            .SetPlaceholderText(value.as_deref().unwrap_or_default())
    }

    pub(super) fn apply_calendar_date_picker_today_highlighted(
        &self,
        id: NodeId,
        value: bool,
    ) -> WindowsResult<()> {
        self.calendar_date_picker(id)?.SetIsTodayHighlighted(value)
    }

    fn calendar_date_picker(&self, id: NodeId) -> WindowsResult<bindings::ICalendarDatePicker> {
        let Handle::CalendarDatePicker { value, .. } = &self.node(id)?.handle else {
            panic!("property update target is not a CalendarDatePicker");
        };
        value.cast()
    }

    pub(super) fn apply_time_picker(
        &self,
        id: NodeId,
        update: &TimePickerUpdate,
    ) -> WindowsResult<()> {
        let Handle::TimePicker {
            value, expected, ..
        } = &self.node(id)?.handle
        else {
            panic!("property update target is not a TimePicker");
        };
        let picker: bindings::ITimePicker = value.cast()?;
        let header = update.header.as_deref().map(inspectable_text);
        picker.SetHeader(header.as_ref())?;
        picker.SetMinuteIncrement(update.minute_increment)?;
        let previous = expected.replace(update.time);
        let result = picker.SetSelectedTime(update.time);
        if result.is_err() {
            expected.set(previous);
        }
        result
    }

    pub(super) fn apply_calendar_view(
        &self,
        id: NodeId,
        update: &CalendarViewUpdate,
    ) -> WindowsResult<()> {
        let Handle::CalendarView { value, state, .. } = &self.node(id)?.handle else {
            panic!("property update target is not a CalendarView");
        };
        let view: bindings::ICalendarView = value.cast()?;
        let previous = state.expected.borrow().clone();
        state.suppressing.set(true);
        let result = (|| {
            view.SetSelectionMode(match update.selection_mode {
                CalendarSelectionMode::None => bindings::CalendarViewSelectionMode::None,
                CalendarSelectionMode::Single => bindings::CalendarViewSelectionMode::Single,
                CalendarSelectionMode::Multiple => bindings::CalendarViewSelectionMode::Multiple,
            })?;
            view.SetIsTodayHighlighted(update.today_highlighted)?;
            view.SetIsGroupLabelVisible(update.group_label_visible)?;
            let dates = view.SelectedDates()?;
            dates.Clear()?;
            for value in update.selected_dates.iter() {
                dates.Append(*value)?;
            }
            Ok(())
        })();
        state.suppressing.set(false);
        if result.is_ok() {
            *state.expected.borrow_mut() = update.selected_dates.to_vec();
        } else {
            *state.expected.borrow_mut() = previous;
        }
        result
    }
}

#[cfg(test)]
mod callback_state_tests {
    use super::*;

    #[test]
    fn scalar_state_preserves_synchronous_reentry_order() {
        let state = Rc::new(ScalarCallbackState {
            expected: Cell::new(1.0),
            suppressing: Cell::new(false),
        });
        let callback_state = Rc::clone(&state);
        let published = Cell::new(0);
        let callback = |value| {
            if callback_state.suppressing.get() {
                assert_eq!(callback_state.expected.get(), value);
                return;
            }
            callback_state.expected.set(value);
            published.set(published.get() + 1);
        };

        state.expected.set(2.0);
        state.suppressing.set(true);
        callback(2.0);
        state.suppressing.set(false);
        assert_eq!(published.get(), 0);

        callback(3.0);
        assert_eq!(state.expected.get(), 3.0);
        assert_eq!(published.get(), 1);
    }
}

#[cfg(test)]
#[path = "../../testing/private/winui/controlled_access.rs"]
pub(super) mod tests;
