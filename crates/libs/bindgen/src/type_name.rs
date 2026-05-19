use super::*;

/// A (namespace, name) pair used as a map key and for type comparisons throughout bindgen.
///
/// `Cow<'static, str>` allows both:
/// - zero-cost `const` values (using `Cow::Borrowed("literal")`)
/// - owned strings built at runtime from metadata (using `Cow::Owned(string)`)
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TypeName(pub Cow<'static, str>, pub Cow<'static, str>);

impl Ord for TypeName {
    fn cmp(&self, other: &Self) -> Ordering {
        // Type names are sorted before namespaces. The `Type` sort order depends on this.
        // This is more efficient in general as many types typically share a namespace.
        (self.1.as_ref(), self.0.as_ref()).cmp(&(other.1.as_ref(), other.0.as_ref()))
    }
}

impl PartialOrd for TypeName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TypeName {
    pub const Object: Self = Self(Cow::Borrowed("System"), Cow::Borrowed("Object"));

    pub const IAsyncAction: Self =
        Self(Cow::Borrowed("Windows.Foundation"), Cow::Borrowed("IAsyncAction"));
    pub const IAsyncActionWithProgress: Self = Self(
        Cow::Borrowed("Windows.Foundation"),
        Cow::Borrowed("IAsyncActionWithProgress"),
    );
    pub const IAsyncOperation: Self =
        Self(Cow::Borrowed("Windows.Foundation"), Cow::Borrowed("IAsyncOperation"));
    pub const IAsyncOperationWithProgress: Self = Self(
        Cow::Borrowed("Windows.Foundation"),
        Cow::Borrowed("IAsyncOperationWithProgress"),
    );

    pub const IIterable: Self = Self(
        Cow::Borrowed("Windows.Foundation.Collections"),
        Cow::Borrowed("IIterable"),
    );
    pub const IIterator: Self = Self(
        Cow::Borrowed("Windows.Foundation.Collections"),
        Cow::Borrowed("IIterator"),
    );

    pub const VARIANT: Self = Self(
        Cow::Borrowed("Windows.Win32.System.Variant"),
        Cow::Borrowed("VARIANT"),
    );
    pub const PROPVARIANT: Self = Self(
        Cow::Borrowed("Windows.Win32.System.Com.StructuredStorage"),
        Cow::Borrowed("PROPVARIANT"),
    );

    /// Construct an owned `TypeName` from borrowed strings (allocates).
    pub fn new(namespace: &str, name: &str) -> Self {
        Self(
            Cow::Owned(namespace.to_string()),
            Cow::Owned(name.to_string()),
        )
    }

    pub fn namespace(&self) -> &str {
        &self.0
    }

    pub fn name(&self) -> &str {
        &self.1
    }

    pub fn write(&self, config: &Config, generics: &[Type]) -> TokenStream {
        let name = to_ident(self.name());
        let namespace = config.write_namespace(self);

        if generics.is_empty() {
            quote! { #namespace #name }
        } else {
            let generics = generics.iter().map(|ty| ty.write_name(config));
            quote! { #namespace #name < #(#generics),* > }
        }
    }
}

impl std::fmt::Display for TypeName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}.{}", self.0, self.1)
    }
}
