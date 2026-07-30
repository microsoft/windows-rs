typedef __int64 INT64;
typedef unsigned __int64 UINT64;
typedef __int64 INT_PTR, *PINT_PTR;
typedef unsigned __int64 UINT_PTR, *PUINT_PTR;

#define MAXUINT64 ((UINT64)~((UINT64)0))
#define MAXINT64 ((INT64)(MAXUINT64 >> 1))
#define MININT64 ((INT64)~MAXINT64)
#define MAXUINT_PTR (~((UINT_PTR)0))
#define MAXINT_PTR ((INT_PTR)(MAXUINT_PTR >> 1))
#define MININT_PTR (~MAXINT_PTR)
#define SMALL_INT_PTR ((INT_PTR)-24)
#define SMALL_UINT_PTR ((UINT_PTR)24)
