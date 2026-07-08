// C++ base-class inheritance: the Win32 `*EX`/`*2`/`*3` extension-struct idiom
// (`struct tagMONITORINFOEXA : public tagMONITORINFO`). The base subobject sits
// at the front of the derived layout, so it must be captured as a leading field
// of the base type — otherwise the derived struct is too small (wrong ABI).
struct MonitorBase {
    int cbSize;
    int flags;
};

typedef struct tagMonitorEx : public MonitorBase {
    int extra;
} MonitorEx;
