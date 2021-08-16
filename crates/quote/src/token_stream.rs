use std::borrow::Cow;

// TODO: maybe just remove TokenStream type and use String directly.

/// A stream of tokens
#[derive(Debug, Clone)]
pub struct TokenStream {
    inner: String,
}

impl TokenStream {
    /// Create a new `TokenStream`
    pub fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }

    /// Appends an identifier to the stream
    ///
    /// note: a space will be inserted before the identifier
    pub fn append(&mut self, ident: Ident) {
        self.push_space();
        self.inner.push_str(ident.as_str())
    }

    /// Appends another stream to the stream
    ///
    /// note: a space will be inserted before the other stream
    pub fn combine(&mut self, other: &TokenStream) {
        self.push_space();
        self.inner.push_str(&other.inner)
    }

    /// View the stream as a string
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Convert the stream into a `String`
    pub fn into_string(self) -> String {
        self.inner
    }

    /// Parse the token stream as something
    ///
    /// Mostly used with `proc_macro2::TokenStream` or `proc_macro::TokenStream`
    pub fn parse<T: std::str::FromStr>(self) -> Result<T, T::Err> {
        self.into_string().parse()
    }

    pub(crate) fn push_space(&mut self) {
        match self.last_char() {
            None | Some(' ') => {}
            _ => self.inner.push(' '),
        }
    }

    pub fn push(&mut self, c: char) {
        self.inner.push(c)
    }

    pub fn push_str(&mut self, str: &str) {
        self.inner.push_str(str)
    }

    fn last_char(&self) -> Option<char> {
        self.inner.chars().last()
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        Self::new()
    }
}

impl std::iter::FromIterator<TokenStream> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenStream>>(iter: I) -> Self {
        iter.into_iter()
            .fold(None, |accum: Option<TokenStream>, n| {
                let mut ts = accum.unwrap_or_else(TokenStream::new);
                ts.combine(&n);
                Some(ts)
            })
            .unwrap_or_else(TokenStream::new)
    }
}

// TODO: need this?

/// An identifier
#[derive(Clone, Debug)]
pub struct Ident {
    inner: Cow<'static, str>,
}

impl Ident {
    /// Create a new `Identifier`
    pub fn new<T: Into<Cow<'static, str>>>(str: T) -> Self {
        Self { inner: str.into() }
    }

    /// View the identifier as a string
    pub fn as_str(&self) -> &str {
        &*self.inner
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

/// A delimiter around a block of code
#[derive(Copy, Clone)]
pub enum Delimiter {
    /// `[]`
    Bracket,
    /// `{}`
    Brace,
    /// `()`
    Parenthesis,
}

impl Delimiter {
    /// The opening delimiter
    pub fn open(self) -> char {
        match self {
            Delimiter::Bracket => '[',
            Delimiter::Brace => '{',
            Delimiter::Parenthesis => '(',
        }
    }

    /// The closing delimiter
    pub fn close(self) -> char {
        match self {
            Delimiter::Bracket => ']',
            Delimiter::Brace => '}',
            Delimiter::Parenthesis => ')',
        }
    }
}

/// A literal of some sort
pub struct Literal {
    inner: String,
}

macro_rules! unsuffixed {
    ($ty:ty => $name:ident) => {
        pub fn $name(n: $ty) -> Self {
            Self {
                inner: n.to_string(),
            }
        }
    };
}

impl Literal {
    unsuffixed!(usize => usize_unsuffixed);
    unsuffixed!(u32 => u32_unsuffixed);
    unsuffixed!(u16 => u16_unsuffixed);
    unsuffixed!(u8 => u8_unsuffixed);

    pub fn byte_string(s: &[u8]) -> Self {
        Self {
            inner: format!(
                "b\"{}\"",
                std::str::from_utf8(s).expect("Could not turn bytes into byte literal")
            ),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accept_owned_and_borrowed() {
        assert_eq!(
            Ident::new("hello").as_str(),
            Ident::new(String::from("hello")).as_str()
        );
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
