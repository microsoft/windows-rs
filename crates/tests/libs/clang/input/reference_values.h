#ifndef FIELD_TYPE
#define FIELD_TYPE int
#endif
#ifndef FIELD_NAME
#define FIELD_NAME value
#endif
#ifndef PACKING
#define PACKING 8
#endif
typedef FIELD_TYPE FieldValue;
#ifdef SHARED_ALIAS
typedef int SharedValue;
#else
#pragma pack(push, PACKING)
struct SharedValue {
#ifdef NESTED_FIELD
    union { int value; float other; };
#else
    FieldValue FIELD_NAME;
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
