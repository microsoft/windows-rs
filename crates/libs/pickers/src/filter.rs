use super::*;

const E_INVALIDARG: HRESULT = HRESULT(0x8007_0057_u32 as i32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileFilter {
    name: String,
    patterns: Vec<String>,
}

impl FileFilter {
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

    pub fn all() -> Self {
        Self::patterns("All files", ["*.*"])
    }

    pub fn validate(&self) -> Result<()> {
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
                "file filters require a nonempty null-free name and at least one nonempty \
                 null-free pattern other than \"*.\"",
            ));
        }
        Ok(())
    }
}

pub struct PreparedFilters {
    _names: Vec<Vec<u16>>,
    _patterns: Vec<Vec<u16>>,
    specs: Vec<COMDLG_FILTERSPEC>,
    initial: Option<u32>,
}

impl PreparedFilters {
    pub fn new(filters: &[FileFilter], initial: Option<usize>) -> Result<Self> {
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

    pub fn apply(&self, dialog: &IFileDialog) -> Result<()> {
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

    fn assert_invalid(filter: &FileFilter) {
        assert_eq!(filter.validate().unwrap_err().code(), E_INVALIDARG);
    }

    #[test]
    fn extensions_are_normalized_to_patterns() {
        assert_eq!(
            FileFilter::extensions("Rust", ["rs", ".rlib"]).patterns,
            ["*.rs", "*.rlib"]
        );
    }

    #[test]
    fn empty_filters_are_rejected() {
        assert_invalid(&FileFilter::patterns("Empty", [] as [&str; 0]));
        assert_invalid(&FileFilter::patterns("", ["*.txt"]));
        assert_invalid(&FileFilter::extensions("Empty", [""]));
    }

    #[test]
    fn embedded_nuls_are_rejected() {
        assert_invalid(&FileFilter::patterns("Invalid", ["*.txt\0*.md"]));
    }

    #[test]
    fn prepared_storage_is_stable_and_ordered() {
        let filters = [
            FileFilter::extensions("Rust", ["rs", "rlib"]),
            FileFilter::patterns("Reports", ["report-*.csv", "summary-*.csv"]),
            FileFilter::all(),
        ];
        let prepared = PreparedFilters::new(&filters, Some(1)).unwrap();

        assert_eq!(prepared.specs.len(), 3);
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
            "report-*.csv;summary-*.csv"
        );
        assert_eq!(
            unsafe { prepared.specs[2].pszSpec.to_string() }.unwrap(),
            "*.*"
        );
    }

    #[test]
    fn initial_filter_must_exist() {
        let filters = [FileFilter::all()];
        assert_eq!(
            PreparedFilters::new(&filters, Some(1))
                .err()
                .unwrap()
                .code(),
            E_INVALIDARG
        );
        assert_eq!(
            PreparedFilters::new(&[], Some(0)).err().unwrap().code(),
            E_INVALIDARG
        );
    }
}
