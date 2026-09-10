typedef struct NestedValue {
    short code;
} NestedValue;

typedef struct _RemoteValue {
    long long payload;
    NestedValue nested;
} RemoteValue;

typedef struct _RemoteValue RemoteAlias;

typedef struct PointerValue {
    unsigned int payload;
} PointerValue;

typedef struct {
    int marker;
} AnonymousValue;
