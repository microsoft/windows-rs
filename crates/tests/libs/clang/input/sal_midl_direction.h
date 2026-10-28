#define OUT __attribute__((annotate("windows_clang:sal:2:_Out_\n()\n()")))
#define COM_OUT __attribute__((annotate("windows_clang:sal:2:_COM_Outptr_\n()\n()")))
#define BOTH __attribute__((annotate("windows_clang:sal:2:_Inout_\n()\n()")))
#define CONDITIONAL __attribute__((annotate("windows_clang:sal:2:_When_\n(enabled,_Out_)\n(enabled,_Out_)")))
#define BUFFER __attribute__((annotate("windows_clang:sal:2:_Outptr_result_buffer_\n(_Inexpressible_(\"external count\"))\n(_Inexpressible_(\"external count\"))")))
#define READS __attribute__((annotate("windows_clang:sal:2:_In_reads_\n(count)\n(count)")))
#define RESERVED __attribute__((annotate("windows_clang:sal:2:_Reserved_\n()\n()")))

typedef void (*CommentBoth)(/* [in][out] */ void** value);
typedef void (*SalOut)(/* [annotation][in][out] */ OUT void** value);
typedef void (*ComOut)(/* [annotation][in][out] */ COM_OUT void** value);
typedef void (*ComConditional)(int enabled, /* [in][out] */ COM_OUT CONDITIONAL void** value);
typedef void (*ComInOut)(/* [in][out] */ COM_OUT BOTH void** value);
typedef void (*SalBoth)(/* [out] */ BOTH void** value);
typedef void (*Opposite)(/* [in] */ OUT int* value);
typedef void (*Conditional)(int enabled, /* [in][out] */ CONDITIONAL void** value);
typedef void (*UnknownExtent)(/* [in][out] */ BUFFER void** value);
typedef void (*Result)(/* [retval][out] */ OUT int* value);
typedef void (*Query)(/* [iid_is][out] */ OUT void** value);

struct Column {
    unsigned status;
};
typedef void (*ColumnUpdates)(unsigned count, /* [in][out] */ READS Column* columns);
typedef void (*ReservedPrefix)(RESERVED unsigned reserved, OUT Column* result);
