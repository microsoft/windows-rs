#ifndef FIELD_TYPE
#define FIELD_TYPE int
#endif
#ifndef PACKING
#define PACKING 8
#endif
#ifndef FIELD_ALIGNMENT
#define FIELD_ALIGNMENT
#endif
typedef FIELD_TYPE FieldValue;
#ifdef SHARED_ALIAS
typedef int SharedValue;
#else
#pragma pack(push, PACKING)
struct SharedValue {
#ifdef NESTED_FIELD
    union { int value; float other; };
#elif defined(OTHER_FIELD_NAME)
    FIELD_ALIGNMENT FieldValue other;
#else
    FIELD_ALIGNMENT FieldValue value;
#endif
#ifdef EXTRA_FIELD
    int extra;
#endif
};
#pragma pack(pop)
#endif
struct ValueConsumer {
    SharedValue value;
};
struct IndependentValue {
    int value;
};
typedef struct _AliasRecord { int value; } AliasRecord, *PointerAlias;
typedef void** DoublePointer;
struct PointerConsumer {
    PointerAlias value;
    DoublePointer handle;
};
