//! Animate a single variable with one transition, sampling the value over time.

use animation_samples::*;

fn main() -> Result<()> {
    init_com();

    let manager = Manager::new()?;
    let library = TransitionLibrary::new()?;

    let variable = manager.create_variable(0.0)?;
    let transition = library.accelerate_decelerate(1.0, 100.0, 0.3, 0.7)?;
    manager.schedule_transition(&variable, &transition, 0.0)?;

    for step in 0..=10 {
        let time = f64::from(step) / 10.0;
        manager.update(time)?;
        let value = variable.value()?;
        println!("t={time:.1}s  value={value:6.2}  {}", bar(value, 100.0));
    }

    Ok(())
}
