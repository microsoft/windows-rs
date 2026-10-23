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
