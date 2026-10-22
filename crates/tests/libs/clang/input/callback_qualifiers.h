typedef void (*Plain)(int*);
typedef void (*Unaligned)(__unaligned int*);
#if defined(_M_X64) || defined(_M_ARM64)
static_assert(!__is_same(Plain, Unaligned));
#endif
#ifdef QUALIFIED
typedef Unaligned Callback;
#else
typedef Plain Callback;
#endif
typedef __unaligned int UnalignedInt;
typedef UnalignedInt InheritedUnaligned;
struct UnalignedRecord { __unaligned int* value; };
struct CallbackUser { Callback value; };
struct CallbackDependent { CallbackUser value; };
struct QualifierSurvivor { int value; };
