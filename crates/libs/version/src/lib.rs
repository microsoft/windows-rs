#![cfg_attr(not(test), no_std)]

mod bindings;
use bindings::*;

/// Operating system version information.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Version {
    /// The major version number of the operating system.
    pub major: u32,

    /// The minor version number of the operating system.
    pub minor: u32,

    /// The major version number of the latest service pack installed on the system.
    pub pack: u32,

    /// The build number of the operating system.
    pub build: u32,
}

impl Version {
    /// Creates a new `Version` with the given values.
    pub const fn new(major: u32, minor: u32, pack: u32, build: u32) -> Self {
        Self {
            major,
            minor,
            pack,
            build,
        }
    }

    /// Gets the version information of the currently running operating system.
    #[cfg(not(test))]
    pub fn current() -> Self {
        unsafe {
            let mut info = OSVERSIONINFOEXW {
                dwOSVersionInfoSize: core::mem::size_of::<OSVERSIONINFOEXW>() as u32,
                ..core::mem::zeroed()
            };

            RtlGetVersion(&mut info as *mut _ as *mut _);

            Version {
                major: info.dwMajorVersion,
                minor: info.dwMinorVersion,
                pack: info.wServicePackMajor as u32,
                build: info.dwBuildNumber,
            }
        }
    }

    /// Determines if the currently running operating system version is greater than or equal to the version of `self`.
    pub fn ge(&self) -> bool {
        let current = Self::current();

        current
            .major
            .cmp(&self.major)
            .then_with(|| current.minor.cmp(&self.minor))
            .then_with(|| current.pack.cmp(&self.pack))
            .then_with(|| current.build.cmp(&self.build))
            .is_ge()
    }

    /// Hook used for testing `ge`.
    #[cfg(test)]
    fn current() -> Self {
        test::test_current()
    }
}

/// Determines if the currently running operating system is a Windows Server release.
pub fn is_server() -> bool {
    unsafe {
        let mut info = OSVERSIONINFOEXW {
            dwOSVersionInfoSize: core::mem::size_of::<OSVERSIONINFOEXW>() as u32,
            ..core::mem::zeroed()
        };

        RtlGetVersion(&mut info as *mut _ as *mut _);
        info.wProductType as u32 != VER_NT_WORKSTATION
    }
}

#[cfg(test)]
mod test {
    use super::Version;
    use std::sync::RwLock;

    static TEST_CURRENT: RwLock<Version> = RwLock::new(Version::new(0, 0, 0, 0));

    pub fn test_current() -> Version {
        *TEST_CURRENT.read().unwrap()
    }

    fn set_current(version: Version) {
        *TEST_CURRENT.write().unwrap() = version;
    }

    #[test]
    fn test() {
        assert_eq!(Version::current(), Version::new(0, 0, 0, 0));

        set_current(Version::new(1, 2, 3, 4));
        assert_eq!(Version::current(), Version::new(1, 2, 3, 4));

        set_current(Version::new(10, 0, 0, 0));
        assert!(Version::new(9, 0, 0, 0).ge());
        assert!(Version::new(10, 0, 0, 0).ge());
        assert!(!Version::new(11, 0, 0, 0).ge());

        set_current(Version::new(10, 100, 0, 0));
        assert!(Version::new(10, 99, 0, 0).ge());
        assert!(Version::new(10, 100, 0, 0).ge());
        assert!(!Version::new(10, 101, 0, 0).ge());

        set_current(Version::new(10, 100, 1000, 0));
        assert!(Version::new(10, 100, 999, 0).ge());
        assert!(Version::new(10, 100, 1000, 0).ge());
        assert!(!Version::new(10, 100, 1001, 0).ge());

        set_current(Version::new(10, 100, 1_000, 10_000));
        assert!(Version::new(10, 100, 1_000, 9_999).ge());
        assert!(Version::new(10, 100, 1_000, 10_000).ge());
        assert!(!Version::new(10, 100, 1_000, 10_001).ge());
    }

    #[test]
    fn bindgen() {
        let args = [
            "--out",
            "src/bindings.rs",
            "--config",
            "flatten",
            "sys",
            "--filter",
            "Windows.Wdk.System.SystemServices.RtlGetVersion",
            "Windows.Win32.System.SystemInformation.OSVERSIONINFOEXW",
            "Windows.Win32.System.SystemServices.VER_NT_WORKSTATION",
        ];

        windows_bindgen::bindgen(args).unwrap();
    }
}
