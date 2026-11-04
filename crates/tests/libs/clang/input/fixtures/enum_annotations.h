#define _WIN32META_ANNOTATION_(...) __attribute__((annotate("Win32Metadata." __VA_ARGS__)))
#define _WIN32META_STRINGIFY_(...) #__VA_ARGS__
#define _WIN32META_EXPAND_STRINGIFY_(...) _WIN32META_STRINGIFY_(__VA_ARGS__)
#define _Win32_AssociatedEnum_(name) \
    _WIN32META_ANNOTATION_("AssociatedEnum", "Win32Metadata.EnumName:" _WIN32META_EXPAND_STRINGIFY_(name))
#define _Win32_Enum_(name, storage, ...) \
    typedef _WIN32META_ANNOTATION_("Enum(" #__VA_ARGS__ ")", __VA_ARGS__) storage name
#define _Win32_FlagsEnum_(name, storage, ...) \
    typedef _WIN32META_ANNOTATION_("FlagsEnum(" #__VA_ARGS__ ")", __VA_ARGS__) storage name

typedef unsigned long DWORD;
typedef unsigned short ADDRESS_FAMILY;

#define DUPLICATE_CLOSE_SOURCE 1
#define DUPLICATE_SAME_ACCESS 2
_Win32_FlagsEnum_(DUPLICATE_HANDLE_OPTIONS, unsigned long,
    DUPLICATE_CLOSE_SOURCE, DUPLICATE_SAME_ACCESS);

#define GENERIC_READ 0x80000000L
#define GENERIC_WRITE 0x40000000L
_Win32_FlagsEnum_(GENERIC_ACCESS_RIGHTS, unsigned long, GENERIC_READ, GENERIC_WRITE);

#define AF_UNSPEC 0
#define AF_INET 2
#define AF_INET6 23
_Win32_Enum_(ADDRESS_FAMILY, unsigned short, AF_UNSPEC, AF_INET, AF_INET6);

enum ResourceType { Heap, Handle };

[[deprecated("use CURRENT_VALUE")]] const int LEGACY_VALUE = 3;
_Win32_Enum_(LegacyValue, int, LEGACY_VALUE);

struct Arguments {
    _Win32_AssociatedEnum_(ADDRESS_FAMILY) unsigned short family;
    _Win32_AssociatedEnum_(enum ResourceType) unsigned long resource;
};

typedef void (*Callback)(_Win32_AssociatedEnum_(DUPLICATE_HANDLE_OPTIONS) DWORD options);

struct IOptions {
    virtual void __stdcall Apply(
        _Win32_AssociatedEnum_(DUPLICATE_HANDLE_OPTIONS) DWORD options) = 0;
};

extern "C" {
    void Duplicate(_Win32_AssociatedEnum_(DUPLICATE_HANDLE_OPTIONS) DWORD options);
    void Access(_Win32_AssociatedEnum_(GENERIC_ACCESS_RIGHTS) DWORD rights);
    void Socket(_Win32_AssociatedEnum_(ADDRESS_FAMILY) int family);
    void Resource(_Win32_AssociatedEnum_(enum ResourceType) unsigned long resource);
    _Win32_AssociatedEnum_(ADDRESS_FAMILY) int Family();
    void Families(_Win32_AssociatedEnum_(ADDRESS_FAMILY) unsigned short* families);
}
