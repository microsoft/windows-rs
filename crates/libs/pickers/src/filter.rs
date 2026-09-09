use crate::bindings::{COMDLG_FILTERSPEC, IFileDialog};
use crate::dialog::wide;
use windows_core::{Error, HRESULT, PCWSTR, Result};

const E_INVALIDARG: HRESULT = HRESULT(0x8007_0057_u32 as i32);

/// A named file-type filter shown by an open or save picker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileFilter {
    pub(crate) name: String,
    pub(crate) patterns: Vec<String>,
}

impl FileFilter {
    /// Creates a filter from file extensions such as `"rs"` or `".rs"`.
    pub fn extensions<I, S>(name: impl Into<String>, extensions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self {
            name: name.into(),
            patterns: extensions
                .into_iter()
                .map(|extension| {
                    let extension = extension.as_ref().trim_start_matches('.');
                    format!("*.{extension}")
                })
                .collect(),
        }
    }

    /// Creates a filter from native wildcard patterns such as `"*.jpg"`.
    pub fn patterns<I, S>(name: impl Into<String>, patterns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            name: name.into(),
            patterns: patterns.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates an unrestricted file filter.
    pub fn all() -> Self {
        Self::patterns("All files", ["*.*"])
    }

    pub(crate) fn validate(&self) -> Result<()> {
        if self.name.is_empty()
            || self.name.contains('\0')
            || self.patterns.is_empty()
            || self
                .patterns
                .iter()
                .any(|pattern| pattern.is_empty() || pattern == "*." || pattern.contains('\0'))
        {
            return Err(Error::new(
                E_INVALIDARG,
                "file filters require a nonempty name and at least one nonempty pattern",
            ));
        }
        Ok(())
    }
}

pub(crate) struct PreparedFilters {
    _names: Vec<Vec<u16>>,
    _patterns: Vec<Vec<u16>>,
    specs: Vec<COMDLG_FILTERSPEC>,
    initial: Option<u32>,
}

impl PreparedFilters {
    pub(crate) fn new(filters: &[FileFilter], initial: Option<usize>) -> Result<Self> {
        for filter in filters {
            filter.validate()?;
        }

        let initial = match initial {
            Some(index) if index < filters.len() => {
                Some(u32::try_from(index + 1).map_err(|_| {
                    Error::new(
                        E_INVALIDARG,
                        "initial filter index exceeds the native range",
                    )
                })?)
            }
            Some(_) => {
                return Err(Error::new(
                    E_INVALIDARG,
                    "initial filter index is outside the configured filters",
                ));
            }
            None => None,
        };

        let names = filters
            .iter()
            .map(|filter| wide(&filter.name))
            .collect::<Result<Vec<_>>>()?;
        let patterns = filters
            .iter()
            .map(|filter| wide(&filter.patterns.join(";")))
            .collect::<Result<Vec<_>>>()?;
        let specs = names
            .iter()
            .zip(&patterns)
            .map(|(name, pattern)| COMDLG_FILTERSPEC {
                pszName: PCWSTR(name.as_ptr()),
                pszSpec: PCWSTR(pattern.as_ptr()),
            })
            .collect();

        Ok(Self {
            _names: names,
            _patterns: patterns,
            specs,
            initial,
        })
    }

    pub(crate) fn apply(&self, dialog: &IFileDialog) -> Result<()> {
        if !self.specs.is_empty() {
            unsafe {
                dialog
                    .SetFileTypes(self.specs.len() as u32, self.specs.as_ptr())
                    .ok()?;
            }
        }
        if let Some(index) = self.initial {
            unsafe { dialog.SetFileTypeIndex(index).ok()? };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extensions_are_normalized_to_patterns() {
        assert_eq!(
            FileFilter::extensions("Rust", ["rs", ".rlib"]).patterns,
            ["*.rs", "*.rlib"]
        );
    }

    #[test]
    fn empty_filters_are_rejected() {
        assert!(
            FileFilter::patterns("Empty", [] as [&str; 0])
                .validate()
                .is_err()
        );
        assert!(FileFilter::patterns("", ["*.txt"]).validate().is_err());
        assert!(FileFilter::extensions("Empty", [""]).validate().is_err());
    }

    #[test]
    fn embedded_nuls_are_rejected() {
        assert!(
            FileFilter::patterns("Invalid", ["*.txt\0*.md"])
                .validate()
                .is_err()
        );
    }

    #[test]
    fn prepared_storage_is_stable_and_ordered() {
        let filters = [
            FileFilter::extensions("Rust", ["rs", "rlib"]),
            FileFilter::all(),
        ];
        let prepared = PreparedFilters::new(&filters, Some(1)).unwrap();

        assert_eq!(prepared.specs.len(), 2);
        assert_eq!(prepared.initial, Some(2));
        assert_eq!(
            unsafe { prepared.specs[0].pszName.to_string() }.unwrap(),
            "Rust"
        );
        assert_eq!(
            unsafe { prepared.specs[0].pszSpec.to_string() }.unwrap(),
            "*.rs;*.rlib"
        );
        assert_eq!(
            unsafe { prepared.specs[1].pszSpec.to_string() }.unwrap(),
            "*.*"
        );
    }

    #[test]
    fn initial_filter_must_exist() {
        let filters = [FileFilter::all()];
        assert!(PreparedFilters::new(&filters, Some(1)).is_err());
        assert!(PreparedFilters::new(&[], Some(0)).is_err());
    }
}
