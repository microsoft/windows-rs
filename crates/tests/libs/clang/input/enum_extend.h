//! library test.dll
//! flat
//! reference enum_extend_ref

// The reference winmd carries a truncated `FILE_INFORMATION_CLASS` (only
// `FileDirectoryInformation`, as `winternl.h` declares it). The scrape sees the full enum from
// the driver headers and carries members the reference lacks, so the enum is un-excluded and
// emitted in full. The winmd merge later unions this complete copy with the truncated reference
// copy into a single enum.
typedef enum _FILE_INFORMATION_CLASS {
    FileDirectoryInformation = 1,
    FileRenameInformation = 10,
    FileRenameInformationEx = 65,
} FILE_INFORMATION_CLASS;

// The reference already owns this enum with the same members, so the scrape adds nothing. It
// stays excluded (the reference copy covers it) and is not re-emitted.
typedef enum _KEY_INFO_CLASS {
    KeyBasicInformation = 0,
} KEY_INFO_CLASS;
