use super::Writer;

pub(crate) struct GenericGuard<'a, 'b> {
    writer: &'a mut Writer<'b>,
    count: usize,
}

impl<'a, 'b> GenericGuard<'a, 'b> {
    pub fn new(writer: &'a mut Writer<'b>, count: usize) -> GenericGuard<'a, 'b> {
        GenericGuard { writer, count }
    }
}

impl<'a, 'b> std::ops::Deref for GenericGuard<'a, 'b> {
    type Target = Writer<'b>;

    fn deref(&self) -> &Self::Target {
        &self.writer
    }
}

impl<'a, 'b> std::ops::DerefMut for GenericGuard<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.writer
    }
}

impl<'a, 'b> Drop for GenericGuard<'a, 'b> {
    fn drop(&mut self) {
        if self.count > 0 {
            self.writer
                .generics
                .resize_with(self.writer.generics.len() - self.count, || {
                    panic!("TODO: drop GenericGuard")
                });
        }
    }
}
