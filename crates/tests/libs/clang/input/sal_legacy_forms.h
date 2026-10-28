#define SAL11(name) __attribute__((annotate("windows_clang:sal:1.1:" name "\n()\n()")))
#define SAL11_COUNT(name) __attribute__((annotate("windows_clang:sal:1.1:" name "\n(count)\n(count)")))
#define SAL11_PART(name) __attribute__((annotate("windows_clang:sal:1.1:" name "\n(capacity,*written)\n(capacity,*written)")))
#define SAL2(name) __attribute__((annotate("windows_clang:sal:2:" name "\n()\n()")))

typedef void (*InCount)(SAL11_COUNT("_In_count_") const int* values, unsigned count);
typedef void (*InOptCount)(SAL11_COUNT("_In_opt_count_") const int* values, unsigned count);
typedef void (*InByteCount)(SAL11_COUNT("_In_bytecount_") const void* values, unsigned count);
typedef void (*InOptByteCount)(SAL11_COUNT("_In_opt_bytecount_") const void* values, unsigned count);
typedef void (*OutCapPostCount)(SAL11_PART("_Out_cap_post_count_") int* values, unsigned capacity, unsigned* written);
typedef void (*OutOptCapPostCount)(SAL11_PART("_Out_opt_cap_post_count_") int* values, unsigned capacity, unsigned* written);
typedef void (*OutOptByteCapPostByteCount)(SAL11_PART("_Out_opt_bytecap_post_bytecount_") void* values, unsigned capacity, unsigned* written);
typedef int (*DerefOut)(SAL11("_Deref_out_") int** value);
typedef int (*DerefOutOpt)(SAL11("_Deref_out_opt_") int** value);
typedef int (*DerefOptOut)(SAL11("_Deref_opt_out_") int** value);
typedef int (*DerefOptOutOpt)(SAL11("_Deref_opt_out_opt_") int** value);
typedef int (*NullOnFailure)(SAL2("_Result_nullonfailure_") SAL2("_Outptr_result_maybenull_") int** value);
typedef int (*NullOnFailureWithoutPointer)(SAL2("_Result_nullonfailure_") SAL2("_Out_") int** value);
