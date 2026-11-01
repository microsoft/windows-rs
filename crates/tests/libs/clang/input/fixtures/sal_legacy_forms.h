#include <sal.h>

typedef void (*InCount)(_In_count_(count) const int* values, unsigned count);
typedef void (*InOptCount)(_In_opt_count_(count) const int* values, unsigned count);
typedef void (*InByteCount)(_In_bytecount_(count) const void* values, unsigned count);
typedef void (*InOptByteCount)(_In_opt_bytecount_(count) const void* values, unsigned count);
typedef void (*OutCapPostCount)(_Out_cap_post_count_(capacity, *written) int* values, unsigned capacity, unsigned* written);
typedef void (*OutOptCapPostCount)(_Out_opt_cap_post_count_(capacity, *written) int* values, unsigned capacity, unsigned* written);
typedef void (*OutOptByteCapPostByteCount)(_Out_opt_bytecap_post_bytecount_(capacity, *written) void* values, unsigned capacity, unsigned* written);
typedef int (*DerefOut)(_Deref_out_ int** value);
typedef int (*DerefOutOpt)(_Deref_out_opt_ int** value);
typedef int (*DerefOptOut)(_Deref_opt_out_ int** value);
typedef int (*DerefOptOutOpt)(_Deref_opt_out_opt_ int** value);
typedef int (*NullOnFailure)(_Result_nullonfailure_ _Outptr_result_maybenull_ int** value);
typedef int (*NullOnFailureWithoutPointer)(_Result_nullonfailure_ _Out_ int** value);
