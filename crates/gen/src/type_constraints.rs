use super::*;
use std::collections::BTreeSet;
use std::fmt;

/// A specific set of type constraints.
///
/// Generates nothing if `Empty` is in use, or `#[cfg(*constraint*)]` if a
/// single constraint is present, or `#[cfg(any(...))]` if multiple constraints
/// are present.
pub enum TypeConstraints {
    Empty,
    Any(BTreeSet<TypeConstraint>),
}

impl TypeConstraints {
    /// Construct a set of type constraints from an iterator.
    pub(crate) fn from_iter<C>(iter: C) -> Self
    where
        C: IntoIterator,
        C::Item: AsRef<TypeConstraint>,
    {
        let mut iter = iter.into_iter();
        let first = iter.next();

        if first.is_none() {
            return Self::Empty;
        }

        let set = first
            .into_iter()
            .chain(iter)
            .map(|t| t.as_ref().clone())
            .collect();

        Self::Any(set)
    }

    /// Merge this set of type constraints with another.
    pub(crate) fn merge<C>(&mut self, constraints: C)
    where
        C: IntoIterator,
        C::Item: AsRef<TypeConstraint>,
    {
        let mut it = constraints.into_iter();
        let first = it.next();

        if first.is_none() {
            return;
        }

        let mut set = match std::mem::replace(self, Self::Empty) {
            Self::Empty => BTreeSet::new(),
            Self::Any(set) => set,
        };

        set.extend(first.into_iter().chain(it).map(|t| t.as_ref().clone()));
        *self = Self::Any(set);
    }
}

impl fmt::Debug for TypeConstraints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_set();

        match self {
            TypeConstraints::Empty => {}
            TypeConstraints::Any(any) => {
                f.entries(any);
            }
        }

        f.finish()
    }
}

impl ToTokens for TypeConstraints {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeConstraints::Empty => {}
            TypeConstraints::Any(any) if any.is_empty() => {}
            TypeConstraints::Any(any) => {
                if any.len() == 1 {
                    quote!(#[cfg(#(#any)*)]).to_tokens(tokens);
                } else {
                    quote!(#[cfg(any(#(#any),*))]).to_tokens(tokens);
                }
            }
        }
    }
}

/// Constraints attriutes for use when building types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TypeConstraint {
    /// Constraint indicating that a specific feature should be present.
    ///
    /// This is constrained as a `#[cfg(feature = "..")]` attribute on any
    /// generated elements.
    Feature(Box<str>),
}

impl AsRef<TypeConstraint> for TypeConstraint {
    fn as_ref(&self) -> &TypeConstraint {
        self
    }
}

impl ToTokens for TypeConstraint {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let quote = match self {
            Self::Feature(feature) => quote! { feature = #feature },
        };

        quote.to_tokens(tokens);
    }
}
