// https://github.com/microsoft/winrt-rs/issues/292

// This is needed for structs with IReference fields, to avoid having to use Option<IReference<T>>
// partly because that's lame and partly because structs must be Default but IReference<T> is not.
