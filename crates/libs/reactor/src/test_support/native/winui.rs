use super::*;

impl WinUiAdapter {
    pub(crate) fn realize_virtual_item(
        &self,
        object: ObjectId,
        index: usize,
    ) -> Result<(), WinUiError> {
        let index = i32::try_from(index).map_err(|_| WinUiError::IndexOverflow(index))?;
        match self.handles.get(&object) {
            Some(Handle::Generated(GeneratedHandle::ItemsRepeater(repeater))) => {
                repeater.GetOrCreateElement(index)?;
                Ok(())
            }
            Some(_) => Err(WinUiError::InvalidObject(object)),
            None => Err(WinUiError::MissingObject(object)),
        }
    }

    pub(crate) fn virtual_shell_counts(&self) -> (usize, usize) {
        self.virtual_items
            .values()
            .fold((0, 0), |(live, retired), items| {
                let pool = items.shells.pool.borrow();
                (live + pool.shells.len(), retired + pool.retired.len())
            })
    }
}
