#define COM_OUT __attribute__((annotate("windows_clang:sal:2:_COM_Outptr_\n()\n()")))
#define AT(subject, ...) __attribute__((annotate("windows_clang:sal:2:_At_\n(" #subject "," #__VA_ARGS__ ")\n(" #subject "," #__VA_ARGS__ ")")))
#define PRE __attribute__((annotate("windows_clang:sal:2:_Pre_\n()\n()")))
#define CONDITIONAL __attribute__((annotate("windows_clang:sal:2:_When_\n(enabled,_Out_)\n(enabled,_Out_)")))

typedef void (*QueryExtent)(COM_OUT AT(*value, _Post_readable_size_(_Inexpressible_(varies))) void** value);
typedef void (*ByteExtent)(COM_OUT AT((*value), _Post_readable_byte_size_(_Inexpressible_("external size"))) void** value);
typedef void (*DataExtent)(AT(*value, _Post_readable_size_(_Inexpressible_(varies))) unsigned char** value);
typedef void (*OtherSubject)(void** other, COM_OUT AT(*other, _Post_readable_size_(_Inexpressible_(varies))) void** value);
typedef void (*Slot)(COM_OUT AT(value, _Post_readable_size_(_Inexpressible_(varies))) void** value);
typedef void (*Entry)(COM_OUT AT(*value, _Pre_readable_size_(_Inexpressible_(varies))) void** value);
typedef void (*Mixed)(COM_OUT AT(*value, _Post_readable_size_(_Inexpressible_(varies)) _Post_writable_size_(1)) void** value);
typedef void (*KnownCount)(COM_OUT AT(*value, _Post_readable_size_(1)) void** value);
typedef void (*Scoped)(COM_OUT PRE AT(*value, _Post_readable_size_(_Inexpressible_(varies))) void** value);
typedef void (*Conditional)(int enabled, COM_OUT AT(*value, _Post_readable_size_(_Inexpressible_(varies))) CONDITIONAL void** value);
