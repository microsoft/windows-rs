use super::*;

const CONTROLS: [&str; 7] = [
    "check", "toggle", "switch", "expander", "slider", "number", "rating",
];
const DECLARED: [&str; 7] = ["off", "off", "off", "collapsed", "150", "150", "4"];
const PROGRAMMED: [&str; 7] = ["off", "off", "off", "collapsed", "40", "40", "4"];
const USER_CALLS: &str = "check Some(true); toggle Some(true); switch true; expanded true; \
                          slider 70; number Some(70.0); rating Some(3.0)";

/// Declared values are written without reporting them, user changes are reported, and a
/// rejected change is replaced by the declared value.
#[test]
fn controlled_values_follow_declared_state() -> Result<()> {
    process::run("controlled", |ui| {
        let controls = Controls::new(ui)?;
        controls.wait(DECLARED)?;
        ui.name(
            "state",
            "Some(false) Some(false) false false 150 Some(150.0) Some(4.0)",
        )?;
        ui.name_stays("calls", "", Duration::from_millis(500))?;

        controls.user_changes()?;
        ui.name("calls", USER_CALLS)?;
        ui.name(
            "state",
            "Some(true) Some(true) true true 70 Some(70.0) Some(3.0)",
        )?;
        controls.wait(["on", "on", "on", "expanded", "70", "70", "3"])?;

        ui.invoke("program")?;
        controls.wait(PROGRAMMED)?;
        ui.name_stays("calls", USER_CALLS, Duration::from_millis(500))?;

        ui.invoke("reject")?;
        controls.user_changes()?;
        let calls = format!("{USER_CALLS}; {USER_CALLS}");
        ui.name("calls", &calls)?;
        controls.wait(PROGRAMMED)?;
        ui.name_stays("calls", &calls, Duration::from_millis(500))
    })
}

/// A range that excludes the declared value coerces the control without reporting it, and the
/// declared value returns once the range allows it. Widening a range together with the value
/// writes the range first.
#[test]
fn controlled_ranges_keep_declared_values() -> Result<()> {
    process::run("controlled", |ui| {
        let controls = Controls::new(ui)?;
        controls.wait(DECLARED)?;

        ui.invoke("shrink")?;
        controls.wait(["off", "off", "off", "collapsed", "20", "20", "2"])?;
        ui.invoke("widen")?;
        controls.wait(DECLARED)?;

        ui.invoke("grow")?;
        controls.wait(["off", "off", "off", "collapsed", "250", "250", "7"])?;
        ui.name_stays("calls", "", Duration::from_millis(500))
    })
}

/// Checking a grouped `RadioButton` reports only the checked button, and a rejected choice
/// restores the declared button even though WinUI unchecked it as a side effect.
#[test]
fn radio_group_restores_declared_choice() -> Result<()> {
    process::run("radio", |ui| {
        let radios = [ui.element("first")?, ui.element("second")?];
        let wait = |expected: [bool; 2]| {
            ui.wait(
                &format!("radio buttons become {expected:?}"),
                || {
                    radios
                        .iter()
                        .map(|radio| unsafe {
                            Ok(radio
                                .GetCurrentPatternAs::<IUIAutomationSelectionItemPattern>(
                                    SELECTION_ITEM_PATTERN,
                                )?
                                .CurrentIsSelected()?
                                .as_bool())
                        })
                        .collect::<windows_core::Result<Vec<_>>>()
                },
                |values| values == &expected,
            )
        };
        let select = |index: usize| unsafe {
            radios[index]
                .GetCurrentPatternAs::<IUIAutomationSelectionItemPattern>(SELECTION_ITEM_PATTERN)?
                .Select()
                .ok()
        };
        wait([true, false])?;
        ui.name_stays("calls", "", Duration::from_millis(500))?;

        select(1)?;
        ui.name("calls", "second Some(true)")?;
        ui.name_stays("calls", "second Some(true)", Duration::from_millis(500))?;
        wait([false, true])?;

        ui.invoke("reject")?;
        select(0)?;
        wait([false, true])?;
        let calls = "second Some(true); first Some(true)";
        ui.name("calls", calls)?;
        ui.name_stays("calls", calls, Duration::from_millis(500))?;
        wait([false, true]).map(drop)
    })
}

/// Password, color, and date follow the same contract as the other controlled values. Each
/// control reports user input once, and programmatic or rejected values are not reported.
#[test]
fn controlled_inputs_follow_declared_state() -> Result<()> {
    process::run("controlled-input", |ui| {
        inputs(ui, "secret", "#112233", 15)?;
        ui.name("state", "secret 112233 2024-03-15")?;
        ui.name_stays("calls", "", Duration::from_millis(500))?;

        let user = "password secretx; color abcdef; date 2024-03-16";
        input_changes(ui, 16)?;
        ui.name("calls", user)?;
        ui.name("state", "secretx abcdef 2024-03-16")?;
        inputs(ui, "secretx", "#ABCDEF", 16)?;

        ui.invoke("program")?;
        inputs(ui, "program", "#445566", 10)?;
        ui.name_stays("calls", user, Duration::from_millis(500))?;

        ui.invoke("reject")?;
        input_changes(ui, 12)?;
        let calls = format!("{user}; password programx; color abcdef; date 2024-03-12");
        ui.name("calls", &calls)?;
        inputs(ui, "program", "#445566", 10)?;
        ui.name_stays("calls", &calls, Duration::from_millis(500))
    })
}

/// Writing a date outside the range coerces the picker without reporting it, and the declared
/// date returns once the range allows it.
#[test]
fn controlled_date_range_keeps_declared_date() -> Result<()> {
    process::run("controlled-input", |ui| {
        inputs(ui, "secret", "#112233", 15)?;
        ui.invoke("narrow")?;
        ui.invoke("program")?;
        date(ui, 20)?;
        ui.invoke("widen")?;
        date(ui, 10)?;
        ui.name("state", "program 445566 2024-03-10")?;
        ui.name_stays("calls", "", Duration::from_millis(500))
    })
}

fn inputs(ui: &Automation, password: &str, color: &str, day: u32) -> Result<()> {
    ui.value("password", password)?;
    ui.value("HexTextBox", color)?;
    date(ui, day)
}

/// Compares the picker text with the same date formatted the way the picker formats it, so the
/// check holds in any locale and calendar.
fn date(ui: &Automation, day: u32) -> Result<()> {
    let expected =
        DateTimeFormatter::CreateDateTimeFormatter("shortdate")?.Format(test_reactor::date(day))?;
    let text = ui.element("DateText")?;
    ui.wait(
        &format!("picker shows {expected}"),
        || unsafe { text.CurrentName() },
        |shown| *shown == *expected,
    )?;
    Ok(())
}

fn input_changes(ui: &Automation, day: u32) -> Result<()> {
    ui.press_keys("password", &[VK_END])?;
    ui.type_text("password", "x")?;
    ui.set_value("HexTextBox", "#abcdef")?;
    ui.invoke("date")?;
    let calendar = ui.element("CalendarView")?;
    let name = day.to_string();
    let item = ui.wait(
        &format!("find calendar day {day}"),
        || unsafe {
            let all = ui.client.CreateTrueCondition()?;
            let items = calendar.FindAll(TreeScope_Descendants, &all)?;
            for index in 0..items.Length()? {
                let item = items.GetElement(index)?;
                if item.CurrentControlType()? == DATA_ITEM && item.CurrentName()? == name.as_str() {
                    return Ok(Some(item));
                }
            }
            Ok(None)
        },
        Option::is_some,
    )?;
    // Select with the keyboard: a pointer click races the popup animation on CI, and the day's
    // SelectionItem peer clears the date before selecting, which a user never sees.
    let item = item.unwrap();
    unsafe { item.SetFocus().ok()? };
    ui.press_keys_on(&item, &name, &[VK_SPACE])?;
    ui.gone("CalendarView")
}

const DATA_ITEM: i32 = 50029;
const VK_END: i32 = 0x23;
const VK_SPACE: i32 = 0x20;
const RANGE_VALUE_PATTERN: i32 = 10003;
const EXPAND_COLLAPSE_PATTERN: i32 = 10005;
const SELECTION_ITEM_PATTERN: i32 = 10010;
const TOGGLE_PATTERN: i32 = 10015;
const VK_LEFT: i32 = 0x25;

struct Controls<'a> {
    ui: &'a Automation,
    elements: Vec<IUIAutomationElement>,
}

impl<'a> Controls<'a> {
    fn new(ui: &'a Automation) -> Result<Self> {
        let elements = CONTROLS
            .iter()
            .map(|id| ui.element(id))
            .collect::<Result<_>>()?;
        Ok(Self { ui, elements })
    }

    fn get(&self, id: &str) -> &IUIAutomationElement {
        &self.elements[CONTROLS.iter().position(|control| *control == id).unwrap()]
    }

    fn wait(&self, expected: [&str; 7]) -> Result<()> {
        self.ui.wait(
            &format!("native values become {expected:?}"),
            || {
                CONTROLS
                    .iter()
                    .zip(&self.elements)
                    .map(|(id, element)| read(id, element))
                    .collect::<windows_core::Result<Vec<_>>>()
            },
            |values| values == &expected,
        )?;
        Ok(())
    }

    /// Changes every control as a user would. RatingControl reports only pointer and keyboard
    /// input, not values set through its automation peer.
    fn user_changes(&self) -> Result<()> {
        eprintln!("change every control through UI Automation");
        unsafe {
            for id in ["check", "toggle", "switch"] {
                self.get(id)
                    .GetCurrentPatternAs::<IUIAutomationTogglePattern>(TOGGLE_PATTERN)?
                    .Toggle()
                    .ok()?;
            }
            self.get("expander")
                .GetCurrentPatternAs::<IUIAutomationExpandCollapsePattern>(EXPAND_COLLAPSE_PATTERN)?
                .Expand()
                .ok()?;
            for id in ["slider", "number"] {
                self.get(id)
                    .GetCurrentPatternAs::<IUIAutomationRangeValuePattern>(RANGE_VALUE_PATTERN)?
                    .SetValue(70.0)
                    .ok()?;
            }
        }
        self.ui.press_keys("rating", &[VK_LEFT])
    }
}

fn read(id: &str, element: &IUIAutomationElement) -> windows_core::Result<String> {
    unsafe {
        Ok(match id {
            "check" | "toggle" | "switch" => match element
                .GetCurrentPatternAs::<IUIAutomationTogglePattern>(TOGGLE_PATTERN)?
                .CurrentToggleState()?
            {
                0 => "off",
                1 => "on",
                _ => "indeterminate",
            }
            .to_string(),
            "expander" => match element
                .GetCurrentPatternAs::<IUIAutomationExpandCollapsePattern>(EXPAND_COLLAPSE_PATTERN)?
                .CurrentExpandCollapseState()?
            {
                0 => "collapsed",
                1 => "expanded",
                _ => "partial",
            }
            .to_string(),
            _ => element
                .GetCurrentPatternAs::<IUIAutomationRangeValuePattern>(RANGE_VALUE_PATTERN)?
                .CurrentValue()?
                .to_string(),
        })
    }
}
