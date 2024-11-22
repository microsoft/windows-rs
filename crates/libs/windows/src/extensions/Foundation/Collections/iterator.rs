use crate::Foundation::Collections::{IIterable, IIterator};

impl<T: windows_core::RuntimeType> Iterator for IIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.Current().ok();

        if result.is_some() {
            self.MoveNext().ok()?;
        }

        result
    }
}

impl<T: windows_core::RuntimeType> IntoIterator for IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}

impl<T: windows_core::RuntimeType> IntoIterator for &IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
