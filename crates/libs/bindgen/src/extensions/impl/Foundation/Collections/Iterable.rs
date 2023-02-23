#[::windows::core::implement(IIterable<T>)]
struct Iterable<T>
where
    T: ::windows::core::RuntimeType + 'static,
    <T as ::windows::core::Type<T>>::Default: ::std::clone::Clone,
{
    values: std::vec::Vec<T::Default>,
}

impl<T> IIterable_Impl<T> for Iterable<T>
where
    T: ::windows::core::RuntimeType,
    <T as ::windows::core::Type<T>>::Default: ::std::clone::Clone,
{
    fn First(&self) -> ::windows::core::Result<IIterator<T>> {
        unsafe {
            // TODO: ideally we can do an AddRef rather than a QI here (via cast)...
            // and then we can get rid of the unsafe as well.
            Ok(Iterator {
                owner: self.cast()?,
                current: 0.into(),
            }
            .into())
        }
    }
}

#[::windows::core::implement(IIterator<T>)]
struct Iterator<T>
where
    T: ::windows::core::RuntimeType + 'static,
    <T as ::windows::core::Type<T>>::Default: ::std::clone::Clone,
{
    owner: IIterable<T>,
    current: ::std::sync::atomic::AtomicUsize,
}

impl<T> IIterator_Impl<T> for Iterator<T>
where
    T: ::windows::core::RuntimeType,
    <T as ::windows::core::Type<T>>::Default: ::std::clone::Clone,
{
    fn Current(&self) -> ::windows::core::Result<T> {
        let owner = ::windows::core::AsImpl::as_impl(&self.owner);
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        if owner.values.len() > current {
            T::from_default(&owner.values[current])
        } else {
            Err(::windows::imp::E_BOUNDS.into())
        }
    }

    fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let owner = ::windows::core::AsImpl::as_impl(&self.owner);
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        Ok(owner.values.len() > current)
    }

    fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let owner = ::windows::core::AsImpl::as_impl(&self.owner);
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        if current < owner.values.len() {
            self.current
                .fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
        }

        Ok(owner.values.len() > current + 1)
    }

    fn GetMany(&self, values: &mut [T::Default]) -> ::windows::core::Result<u32> {
        let owner = ::windows::core::AsImpl::as_impl(&self.owner);
        let current = self.current.load(::std::sync::atomic::Ordering::Relaxed);

        let actual = std::cmp::min(owner.values.len() - current, values.len());
        let (values, _) = values.split_at_mut(actual);
        values.clone_from_slice(&owner.values[current..current + actual]);
        self.current
            .fetch_add(actual, ::std::sync::atomic::Ordering::Relaxed);
        Ok(actual as _)
    }
}

impl<T> ::core::convert::TryFrom<::std::vec::Vec<T::Default>> for IIterable<T>
where
    T: ::windows::core::RuntimeType,
    <T as ::windows::core::Type<T>>::Default: ::std::clone::Clone,
{
    type Error = ::windows::core::Error;
    fn try_from(values: ::std::vec::Vec<T::Default>) -> ::windows::core::Result<Self> {
        // TODO: should provide a fallible try_into or more explicit allocator
        Ok(Iterable { values }.into())
    }
}
