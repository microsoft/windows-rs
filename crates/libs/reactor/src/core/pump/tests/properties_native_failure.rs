//! Property publication and fatal native-failure contract tests.

use super::super::*;
use super::support::*;

#[test]
fn successful_property_updates_publish_known_values() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBlock::new().text("first").into()).unwrap();
    let root = pump.root().unwrap();

    pump.update(TextBlock::new().text("second").into()).unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("second".into()))
    );
    assert_eq!(
        pump.tree
            .native(root)
            .properties
            .get(&PropertyId::TextBlockText),
        Some(&Some(PropertyValue::Str("second".into())))
    );

    pump.update(TextBlock::new().into()).unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::TextBlockText),
        None
    );
}

#[test]
fn native_failure_stops_the_batch_and_poisons_the_pump() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBlock::new().text("first").into()).unwrap();
    let root = pump.root().unwrap();
    let version = pump.version();
    pump.runtime_mut().fail_at(0);

    assert_eq!(
        pump.update(TextBlock::new().text("second").into()),
        Err(PumpError::NativeApplyFailed(NativeApplyError {
            command: 0,
            error: RuntimeError::Injected,
        }))
    );
    assert!(pump.poisoned());
    assert_eq!(pump.version(), version);
    assert_eq!(
        pump.tree.native(root).desired,
        Element::from(TextBlock::new().text("first"))
            .into_parts()
            .props
    );
    assert_eq!(
        pump.update(TextBlock::new().text("third").into()),
        Err(PumpError::Poisoned)
    );
}

#[test]
fn failure_at_each_command_position_reports_the_exact_command() {
    let mut probe = Pump::new(RecordingRuntime::default());
    probe.mount(representative_tree()).unwrap();
    let command_count = probe.runtime().commands()[0].len();

    for failed_command in 0..command_count {
        let mut runtime = RecordingRuntime::default();
        runtime.fail_at(failed_command);
        let mut pump = Pump::new(runtime);

        assert_eq!(
            pump.mount(representative_tree()),
            Err(PumpError::NativeApplyFailed(NativeApplyError {
                command: failed_command,
                error: RuntimeError::Injected,
            }))
        );
        assert!(pump.poisoned());
        assert_eq!(pump.root(), None);
        assert_eq!(pump.version(), 0);
    }
}
