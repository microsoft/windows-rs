enum class ForwardStatus : unsigned short;

#include "const_dependency_inc.inl"

#define STATUS_LITERAL ((IncludedStatus)7)
#define STATUS_CHAINED ((ChainedStatus)9)
#define STATUS_EVALUATED ((IncludedStatus)(1 + 2))
#define STATUS_FORWARD ((ForwardStatus)11)
