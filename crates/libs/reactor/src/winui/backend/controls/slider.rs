//! Typed handler for the `Slider` widget.

use crate::bindings as Xaml;
use crate::core::widgets::Slider;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(w: &Slider, handle: &Handle) -> windows_core::Result<()> {
    let rb = handle.cast_inner::<Xaml::IRangeBase>()?;
    let sl = handle.cast_inner::<Xaml::ISlider>()?;
    rb.put_Minimum(w.minimum)?;
    rb.put_Maximum(w.maximum)?;
    rb.put_Value(w.value)?;
    if let Some(step) = w.step {
        sl.put_StepFrequency(step)?;
        rb.put_SmallChange(step)?;
    }
    if let Some(s) = &w.header {
        sl.put_Header(&string_as_textblock(s)?)?;
    }
    if w.vertical {
        sl.put_Orientation(Xaml::Orientation::Vertical)?;
    }
    if !w.is_enabled {
        handle
            .cast_inner::<Xaml::IControl>()?
            .put_IsEnabled(false)?;
    }
    Ok(())
}

pub fn diff(old: &Slider, new: &Slider, handle: &Handle) -> windows_core::Result<()> {
    let rb = handle.cast_inner::<Xaml::IRangeBase>()?;
    let sl = handle.cast_inner::<Xaml::ISlider>()?;

    if new.minimum != old.minimum {
        rb.put_Minimum(new.minimum)?;
    }
    if new.maximum != old.maximum {
        rb.put_Maximum(new.maximum)?;
    }
    if new.value != old.value {
        rb.put_Value(new.value)?;
    }
    if new.step != old.step {
        match new.step {
            Some(step) => {
                sl.put_StepFrequency(step)?;
                rb.put_SmallChange(step)?;
            }
            None => {
                sl.put_StepFrequency(1.0)?;
                rb.put_SmallChange(1.0)?;
            }
        }
    }
    if new.header != old.header {
        match &new.header {
            Some(s) => sl.put_Header(&string_as_textblock(s)?)?,
            None => sl.put_Header(None)?,
        }
    }
    if new.vertical != old.vertical {
        let orient = if new.vertical {
            Xaml::Orientation::Vertical
        } else {
            Xaml::Orientation::Horizontal
        };
        sl.put_Orientation(orient)?;
    }
    if new.is_enabled != old.is_enabled {
        if new.is_enabled {
            handle
                .cast_inner::<Xaml::IDependencyObject>()?
                .ClearValue(&Xaml::Control::get_IsEnabledProperty()?)?;
        } else {
            handle
                .cast_inner::<Xaml::IControl>()?
                .put_IsEnabled(false)?;
        }
    }
    Ok(())
}
