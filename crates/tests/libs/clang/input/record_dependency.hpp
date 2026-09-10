#include "record_dependency_inc.inl"

typedef struct Envelope {
    RemoteValue value;
    struct _RemoteValue direct_value;
    RemoteAlias alias_value;
    PointerValue* pointer;
    AnonymousValue anonymous_value;
} Envelope;

RemoteValue ReturnRemote(void);
void AcceptRemote(RemoteValue value);
struct _RemoteValue ReturnDirect(void);
RemoteAlias ReturnAlias(void);
PointerValue* ReturnPointer(void);
AnonymousValue ReturnAnonymous(void);
