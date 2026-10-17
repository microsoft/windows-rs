#ifdef ANNOTATED
#define ALIAS_ATTRIBUTE __declspec(deprecated("marked"))
#else
#define ALIAS_ATTRIBUTE
#endif
typedef ALIAS_ATTRIBUTE unsigned long ULongA;
typedef unsigned long ULongB;
typedef unsigned int UInt;
typedef ULongA NestedA;
typedef ULongB NestedB;
typedef bool BoolA;
typedef bool BoolB;
typedef unsigned char ByteA;
typedef unsigned char ByteB;
typedef double RealA;
typedef double RealB;
typedef unsigned long* PointerA;
typedef unsigned long* PointerB;
enum NativeEnum : int { MEMBER = 3 };
typedef NativeEnum EnumA;
typedef NativeEnum EnumB;

#ifdef OTHER
#ifdef NATIVE_MISMATCH
#define INTEGER_TYPE UInt
#else
#define INTEGER_TYPE NestedB
#endif
#define BOOLEAN_TYPE BoolB
#define BYTE_TYPE ByteB
#define REAL_TYPE RealB
#define POINTER_TYPE PointerB
#define ENUM_TYPE EnumB
#else
#define INTEGER_TYPE NestedA
#define BOOLEAN_TYPE BoolA
#define BYTE_TYPE ByteA
#define REAL_TYPE RealA
#define POINTER_TYPE PointerA
#define ENUM_TYPE EnumA
#endif

#if defined(ANNOTATION_MISMATCH) && defined(OTHER)
#define VALUE_ATTRIBUTE __declspec(deprecated("other"))
#elif defined(ANNOTATION_MISMATCH) || defined(VALUE_ANNOTATED)
#define VALUE_ATTRIBUTE __declspec(deprecated("value"))
#else
#define VALUE_ATTRIBUTE
#endif
#ifdef DECLARATION_ONLY
VALUE_ATTRIBUTE extern const INTEGER_TYPE SHARED;
#elif defined(OTHER) && defined(VALUE_MISMATCH)
VALUE_ATTRIBUTE extern const INTEGER_TYPE SHARED = 8;
#else
VALUE_ATTRIBUTE extern const INTEGER_TYPE SHARED = 7;
#endif
const ULongA UNIQUE = 9;
const BOOLEAN_TYPE BOOLEAN = true;
#define SHARED_MACRO ((INTEGER_TYPE)7)
#define BYTE_MACRO ((BYTE_TYPE)255)
#ifdef FLOAT_MISMATCH
#ifdef OTHER
const REAL_TYPE FLOATING = -0.0;
#else
const REAL_TYPE FLOATING = 0.0;
#endif
#else
const REAL_TYPE FLOATING = 1.25;
#endif
const POINTER_TYPE POINTER = (POINTER_TYPE)0;
const ENUM_TYPE ENUM_VALUE = MEMBER;
