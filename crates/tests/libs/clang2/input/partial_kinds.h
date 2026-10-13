struct Safe { int value; };
#if defined(_M_IX86)
struct ChangingKind { int value; };
typedef int ChangingAlias;
#else
union ChangingKind { int value; float other; };
using ChangingAlias = int;
#endif
struct KindConsumer { ChangingKind* value; };
struct AliasConsumer { ChangingAlias value; };
