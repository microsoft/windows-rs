use super::*;
use ArrayDelegates::*;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    widget.CheckStrings(&StringsHandler::new(|input, fill, output| {
        if input.len() != fill.len() {
            return Err(Error::from_hresult(E_INVALIDARG));
        }
        for (target, source) in fill.iter_mut().zip(input.iter().rev()) {
            target.clone_from(source);
        }
        *output = Array::from_slice(input);
        Ok(Array::from_slice(fill))
    }))?;
    widget.CheckEntries(&EntriesHandler::new(|input, fill, output| {
        if input.len() != fill.len() {
            return Err(Error::from_hresult(E_INVALIDARG));
        }
        for (target, source) in fill.iter_mut().zip(input.iter().rev()) {
            target.clone_from(source);
        }
        *output = Array::from_slice(input);
        Ok(Array::from_slice(fill))
    }))?;
    Ok(())
}
