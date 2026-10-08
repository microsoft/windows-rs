struct Context;
union Payload;
typedef Context* ContextPointer;
struct Owner { Context* context; Payload* payload; };
extern "C" void Use(Context* context, Payload* payload);
extern "C" void ByValue(Context context);
typedef Payload (*Result)();
