//! flat
// The C flags/enum idiom: a tagged enum `_FOO` paired with a *separate* integer
// typedef `FOO` linked only by the `_` naming convention. The typedef carries the
// authoritative storage type, so the merged enum adopts it (`u32` here) and its
// high-bit member projects as a positive constant rather than a negative `int`.
typedef unsigned long DWORD;
#define DEFINE_ENUM_FLAG_OPERATORS(T)

enum _SHCONTF {
    SHCONTF_FOLDERS = 0x20,
    SHCONTF_NONFOLDERS = 0x40,
    SHCONTF_INIT_ON_FIRST_NEXT = 0x100,
};
typedef DWORD SHCONTF;

// A signed variant: `typedef int` keeps the enum `i32`.
enum _TASKDIALOG_FLAGS {
    TDF_ENABLE_HYPERLINKS = 0x0001,
    TDF_USE_HICON_MAIN = 0x0002,
    TDF_ALL = 0x80000000,
};
typedef int TASKDIALOG_FLAGS;

// MIDL `[v1_enum]` idiom: the enum's typedef name equals its tag (`_SVGIO`), the public
// integer typedef is separate (`typedef int SVGIO`), and DEFINE_ENUM_FLAG_OPERATORS keys
// on the internal tag. The merge renames `_SVGIO` -> `SVGIO`, drops both the self-named
// enum typedef and the int typedef, and preserves the flag promotion (i32 -> u32) so the
// high-bit member stays a positive `u32`.
typedef enum _SVGIO {
    SVGIO_BACKGROUND = 0,
    SVGIO_TYPE_MASK = 0xf,
    SVGIO_FLAG_VIEWORDER = 0x80000000,
} _SVGIO;
DEFINE_ENUM_FLAG_OPERATORS(_SVGIO)
typedef int SVGIO;

// A struct field typed by each public name resolves to the merged enum.
struct DIALOG_CONFIG {
    SHCONTF flags;
    TASKDIALOG_FLAGS dialog;
    SVGIO view;
};
