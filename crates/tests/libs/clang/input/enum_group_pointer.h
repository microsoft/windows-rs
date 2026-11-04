typedef void* HANDLE;
#define GR_GLOBAL ((HANDLE)-2)
#define GR_GDIOBJECTS 0

typedef __attribute__((annotate("Win32Metadata.Enum(GR_GDIOBJECTS)", GR_GDIOBJECTS)))
    unsigned int GET_GUI_RESOURCES_FLAGS;
typedef __attribute__((annotate("Win32Metadata.Enum(GR_GLOBAL)", (unsigned int)-2)))
    unsigned int GET_GUI_RESOURCES_FLAGS;
