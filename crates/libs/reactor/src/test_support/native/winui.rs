use super::*;

impl WinUiAdapter {
    pub(crate) fn virtual_repeater(&self, object: ObjectId) -> Option<native::ItemsRepeater> {
        match self.handles.get(&object) {
            Some(Handle::Generated(GeneratedHandle::ItemsRepeater(repeater))) => {
                Some(repeater.clone())
            }
            _ => None,
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
