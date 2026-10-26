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

const CHURN: [&str; 8] = [
    "check", "toggle", "switch", "expander", "slider", "number", "text", "list",
];
const ROUNDS: usize = 8;

/// While the app declares new values on every timer tick, programmatic churn alone reports
/// nothing, and once churning stops every control settles on its declared value.
#[test]
fn churned_values_settle_without_reports() -> Result<()> {
    process::run("controlled-churn", |ui| {
        ui.invoke("churn")?;
        ui.name("mode", "churning")?;
        std::thread::sleep(Duration::from_millis(500));
        let calls = settle(ui)?;
        assert_eq!(calls, "");
        let ticks = element_name(ui, "ticks")?.parse::<u32>().unwrap();
        assert!(ticks >= 10, "churn ran only {ticks} ticks");
        Ok(())
    })
}

/// User input interleaved with programmatic churn is reported at most once per action and never
/// echoes a programmed value, and every control settles on its declared value.
#[test]
fn churned_user_input_settles_on_declared_values() -> Result<()> {
    process::run("controlled-churn", |ui| churn_with_input(ui, false))
}

/// Rejected user input interleaved with programmatic churn leaves every control on the
/// programmed value the app last declared.
#[test]
fn churned_rejected_input_restores_declared_values() -> Result<()> {
    process::run("controlled-churn", |ui| churn_with_input(ui, true))
}

fn churn_with_input(ui: &Automation, reject: bool) -> Result<()> {
    let elements = CHURN
        .iter()
        .map(|id| ui.element(id))
        .collect::<Result<Vec<_>>>()?;
    let items = test_reactor::CHURN_ITEMS[2..]
        .iter()
        .map(|name| ui.list_item("list", name))
        .collect::<Result<Vec<_>>>()?;
    if reject {
        ui.invoke("reject")?;
    }
    ui.invoke("churn")?;
    ui.name("mode", "churning")?;
    eprintln!("drive {ROUNDS} rounds of user input while churning");
    for round in 0..ROUNDS {
        unsafe {
            for element in &elements[..3] {
                element
                    .GetCurrentPatternAs::<IUIAutomationTogglePattern>(TOGGLE_PATTERN)?
                    .Toggle()
                    .ok()?;
            }
            let expander = elements[3].GetCurrentPatternAs::<IUIAutomationExpandCollapsePattern>(
                EXPAND_COLLAPSE_PATTERN,
            )?;
            if round % 2 == 0 {
                expander.Expand().ok()?;
            } else {
                expander.Collapse().ok()?;
            }
            for element in &elements[4..6] {
                element
                    .GetCurrentPatternAs::<IUIAutomationRangeValuePattern>(RANGE_VALUE_PATTERN)?
                    .SetValue(60.0 + round as f64)
                    .ok()?;
            }
            elements[6]
                .GetCurrentPatternAs::<IUIAutomationValuePattern>(VALUE_PATTERN)?
                .SetValue(&format!("u{round}").into())
                .ok()?;
            items[round % items.len()]
                .GetCurrentPatternAs::<IUIAutomationSelectionItemPattern>(SELECTION_ITEM_PATTERN)?
                .Select()
                .ok()?;
        }
    }
    let calls = settle(ui)?;
    let calls = calls.split("; ").filter(|call| !call.is_empty());
    for id in CHURN {
        let values = calls
            .clone()
            .filter_map(|call| call.strip_prefix(id)?.strip_prefix(' '))
            .collect::<Vec<_>>();
        // TextChanged is asynchronous, so a churned write can replace the entered text before it
        // reports, and a tick can leave the Expander already in the requested state. Every other
        // action changes its control's state and reports synchronously.
        assert!(
            reject || matches!(id, "text" | "expander") || !values.is_empty(),
            "{id} reported nothing for {ROUNDS} actions"
        );
        assert!(
            values.len() <= ROUNDS,
            "{id} reported {} times for {ROUNDS} actions: {values:?}",
            values.len()
        );
        // These inputs enter a unique value each round, so a repeat is a duplicate report.
        if matches!(id, "slider" | "number" | "text") {
            let mut unique = values.clone();
            unique.sort_unstable();
            unique.dedup();
            assert_eq!(
                unique.len(),
                values.len(),
                "{id} reported a value twice: {values:?}"
            );
        }
        // Programmed values are below 60, `p<n>`, and the first two items.
        let user = |value: &str| match id {
            "slider" => value.parse::<f64>().is_ok_and(|value| value >= 60.0),
            "number" => value
                .strip_prefix("Some(")
                .and_then(|value| value.strip_suffix(')'))
                .and_then(|value| value.parse::<f64>().ok())
                .is_some_and(|value| value >= 60.0),
            "text" => value.starts_with('u'),
            "list" => !matches!(value, "None" | "Some(0)" | "Some(1)"),
            _ => true,
        };
        assert!(
            values.iter().all(|value| user(value)),
            "{id} reported a programmed value: {values:?}"
        );
    }
    if reject {
        for (id, programmed) in [
            ("slider", ["10", "20"]),
            ("number", ["10", "20"]),
            ("text", ["p0", "p1"]),
            ("list", ["Zero", "One"]),
        ] {
            let declared = element_name(ui, &format!("declared-{id}"))?;
            assert!(
                programmed.contains(&declared.as_str()),
                "rejected input changed declared {id} to {declared}"
            );
        }
    }
    Ok(())
}

/// Stops churning, waits until every native value equals its declaration and no late report
/// arrives, then returns the logged calls.
fn settle(ui: &Automation) -> Result<String> {
    ui.invoke("settle")?;
    ui.name("mode", "settled")?;
    let elements = CHURN
        .iter()
        .map(|id| ui.element(id))
        .collect::<Result<Vec<_>>>()?;
    let declared = CHURN
        .iter()
        .map(|id| ui.element(&format!("declared-{id}")))
        .collect::<Result<Vec<_>>>()?;
    let sample = || {
        CHURN
            .iter()
            .zip(&elements)
            .zip(&declared)
            .map(|((id, element), declared)| {
                Ok((churn_read(ui, id, element)?, unsafe {
                    String::from_utf16_lossy(&declared.CurrentName()?)
                }))
            })
            .collect::<windows_core::Result<Vec<_>>>()
    };
    let settled =
        |values: &Vec<(String, String)>| values.iter().all(|(native, declared)| native == declared);
    ui.wait("native values settle on declared values", sample, settled)?;
    let calls = element_name(ui, "calls")?;
    ui.name_stays("calls", &calls, Duration::from_millis(500))?;
    let values = sample()?;
    assert!(
        settled(&values),
        "native state departed after settling: {values:?}"
    );
    Ok(calls)
}

fn churn_read(
    ui: &Automation,
    id: &str,
    element: &IUIAutomationElement,
) -> windows_core::Result<String> {
    unsafe {
        match id {
            "text" => Ok(String::from_utf16_lossy(
                &element
                    .GetCurrentPatternAs::<IUIAutomationValuePattern>(VALUE_PATTERN)?
                    .CurrentValue()?,
            )),
            "list" => {
                let selection = element
                    .GetCurrentPatternAs::<IUIAutomationSelectionPattern>(SELECTION_PATTERN)?
                    .GetCurrentSelection()?;
                if selection.Length()? == 0 {
                    return Ok("none".to_string());
                }
                // DataItem text belongs to the container's child peer.
                let all = ui.client.CreateTrueCondition()?;
                let content = selection
                    .GetElement(0)?
                    .FindAll(TreeScope_Descendants, &all)?;
                for index in 0..content.Length()? {
                    let name = content.GetElement(index)?.CurrentName()?;
                    if !name.is_empty() {
                        return Ok(String::from_utf16_lossy(&name));
                    }
                }
                Ok(String::new())
            }
            _ => read(id, element),
        }
    }
}

fn element_name(ui: &Automation, id: &str) -> Result<String> {
    let name = unsafe { ui.element(id)?.CurrentName()? };
    Ok(String::from_utf16_lossy(&name))
}
