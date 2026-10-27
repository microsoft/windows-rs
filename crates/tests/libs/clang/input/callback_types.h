struct Item { int value; };
typedef unsigned Count;
typedef int (*Handler)(Count count, Item* item);
typedef void (__cdecl *Cdecl)(int value);
typedef void (__stdcall *Stdcall)(int value);
typedef void (*Dispatch)(Handler handler, Item* item);
typedef Handler* HandlerAddress;
typedef Handler (*Factory)();
struct Callbacks { Handler handler; Dispatch dispatch; Handler handlers[2]; };
