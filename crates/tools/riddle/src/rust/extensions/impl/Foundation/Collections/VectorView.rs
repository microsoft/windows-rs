#[::windows_implement::implement(IVectorView<T>, IIterable<T>)]
struct StockVectorView<T>
where
    T: ::windows_core::RuntimeType + 'static,
    <T as ::windows_core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    values: std::vec::Vec<T::Default>,
}

impl<T> IIterable_Impl<T> for StockVectorView<T>
where
    T: ::windows_core::RuntimeType,
    <T as ::windows_core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    fn First(&self) -> ::windows_core::Result<IIterator<T>> {
        unsafe {
            // TODO: ideally we can do an AddRef rather than a QI here (via cast)...
            // and then we can get rid of the unsafe as well.
            Ok(StockVectorViewIterator {
                owner: self.cast()?,
                current: 0.into(),
            }
            .into())
        }
    }
}

impl<T> IVectorView_Impl<T> for StockVectorView<T>
where
    T: ::windows_core::RuntimeType,
    <T as ::windows_core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    fn GetAt(&self, index: u32) -> ::windows_core::Result<T> {
        let item = self
            .values
            .get(index as usize)
            .ok_or_else(|| ::windows_core::Error::from(::windows_core::imp::E_BOUNDS))?;
        T::from_default(item)
    }
    fn Size(&self) -> ::windows_core::Result<u32> {
        Ok(self.values.len() as u32)
    }
    fn IndexOf(&self, value: &T::Default, result: &mut u32) -> ::windows_core::Result<bool> {
        match self.values.iter().position(|element| element == value) {
            Some(index) => {
                *result = index as u32;
                Ok(true)
            }
            None => Ok(false),
        }
    }
    fn GetMany(&self, current: u32, values: &mut [T::Default]) -> ::windows_core::Result<u32> {
        let current = current as usize;
        if current >= self.values.len() {
            return Ok(0);
        }
        let actual = std::cmp::min(self.values.len() - current, values.len());
        let (values, _) = values.split_at_mut(actual);
        values.clone_from_slice(&self.values[current..current + actual]);
        Ok(actual as u32)
    }
}

#[::windows_implement::implement(IIterator<T>)]
struct StockVectorViewIterator<T>
where
    T: ::windows_core::RuntimeType + 'static,
    <T as ::windows_core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    owner: IIterable<T>,
    current: ::std::sync::atomic::AtomicUsize,
}

impl<T> IIterator_Impl<T> for StockVectorViewIterator<T>
where
    T: ::windows_core::RuntimeType,
    <T as ::windows_core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    fn Current(&self) -> ::windows_core::Result<T> {
        let owner: &StockVectorView<T> = unsafe { ::windows_core::AsImpl::as_impl(&self.owner) };
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        if owner.values.len() > current {
            T::from_default(&owner.values[current])
        } else {
            Err(::windows_core::Error::from(::windows_core::imp::E_BOUNDS))
        }
    }

    fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let owner: &StockVectorView<T> = unsafe { ::windows_core::AsImpl::as_impl(&self.owner) };
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        Ok(owner.values.len() > current)
    }

    fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let owner: &StockVectorView<T> = unsafe { ::windows_core::AsImpl::as_impl(&self.owner) };
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        if current < owner.values.len() {
            self.current
                .fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
        }

        Ok(owner.values.len() > current + 1)
    }

    fn GetMany(&self, values: &mut [T::Default]) -> ::windows_core::Result<u32> {
        let owner: &StockVectorView<T> = unsafe { ::windows_core::AsImpl::as_impl(&self.owner) };
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        let actual = std::cmp::min(owner.values.len() - current, values.len());
        let (values, _) = values.split_at_mut(actual);
        values.clone_from_slice(&owner.values[current..current + actual]);
        self.current
            .fetch_add(actual, ::std::sync::atomic::Ordering::Relaxed);
        Ok(actual as u32)
    }
}

impl<T> ::core::convert::TryFrom<::std::vec::Vec<T::Default>> for IVectorView<T>
where
    T: ::windows_core::RuntimeType,
    <T as ::windows_core::Type<T>>::Default: std::clone::Clone + std::cmp::PartialEq,
{
    type Error = ::windows_core::Error;
    fn try_from(values: ::std::vec::Vec<T::Default>) -> ::windows_core::Result<Self> {
        // TODO: should provide a fallible try_into or more explicit allocator
        Ok(StockVectorView { values }.into())
    }
}
