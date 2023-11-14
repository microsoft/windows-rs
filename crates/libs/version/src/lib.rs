#![cfg_attr(not(test), no_std)]

mod bindings;
use bindings::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub pack: u32,
    pub build: u32,
}

impl Version {
    pub const fn new(major: u32, minor: u32, pack: u32, build: u32) -> Self {
        Self {
            major,
            minor,
            pack,
            build,
        }
    }

    #[cfg(test)]
    const fn zeroed() -> Self {
        Self::new(0, 0, 0, 0)
    }

    #[cfg(test)]
    pub fn current() -> Self {
        test::test_current()
    }

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

    pub fn ge(&self) -> bool {
        use core::cmp::Ordering::*;
        let current = Self::current();

        match current.major.cmp(&self.major) {
            Greater => true,
            Less => false,
            Equal => match current.minor.cmp(&self.minor) {
                Greater => true,
                Less => false,
                Equal => match current.pack.cmp(&self.pack) {
                    Greater => true,
                    Less => false,
                    Equal => match current.build.cmp(&self.build) {
                        Greater => true,
                        Less => false,
                        Equal => true,
                    },
                },
            },
        }
    }
}

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

    static TEST_CURRENT: RwLock<Version> = RwLock::new(Version::zeroed());

    pub fn test_current() -> Version {
        *TEST_CURRENT.read().unwrap()
    }

    fn set_current(version: Version) {
        *TEST_CURRENT.write().unwrap() = version;
    }

    #[test]
    fn test() {
        assert_eq!(Version::current(), Version::zeroed());

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
