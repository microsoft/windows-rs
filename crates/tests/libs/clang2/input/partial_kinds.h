struct Safe { int value; };
#if defined(_M_IX86)
struct ChangingKind { int value; };
typedef int ChangingAlias;
struct AliasConsumer { ChangingAlias value; };
#else
class ChangingKind { public: int value; };
template<typename T> using ChangingAlias = T;
struct AliasConsumer { ChangingAlias<int> value; };
#endif
struct KindConsumer { ChangingKind* value; };
