struct Empty {};
struct Incomplete;
struct OpaquePointer { Incomplete* value; };
struct ConstField { const int value; };
struct VolatileField { volatile int value; };
struct FieldCollision { int self; int self_; };

typedef struct _Common { int value; } First;
typedef struct _Common Second;
