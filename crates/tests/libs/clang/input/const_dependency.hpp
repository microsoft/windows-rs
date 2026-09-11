#include "const_dependency_inc.inl"

#define STATUS_LITERAL ((IncludedStatus)7)
#define STATUS_CHAINED ((ChainedStatus)9)
#define STATUS_EVALUATED ((IncludedStatus)(1 + 2))
#define STATUS_FUNDAMENTAL ((DWORD)(1 + 2))
#define STATUS_BASE 4
#define STATUS_NONCAST ((STATUS_BASE) + 1)
#define STATUS_REDEFINED ((ChainedStatus)(1 + 2))
#undef STATUS_REDEFINED
#define STATUS_REDEFINED (4 + 5)
#define STATUS_DIRECT_REDEFINED ((UnusedStatus)1)
#undef STATUS_DIRECT_REDEFINED
#define STATUS_DIRECT_REDEFINED 9
#define STATUS_DIRECT_FLOAT 1.5
#define STATUS_TERMINAL_UNDEF ((UnusedStatus)3)
#undef STATUS_TERMINAL_UNDEF
#define STATUS_DROPPED_REDEFINITION ((UnusedStatus)2)
#undef STATUS_DROPPED_REDEFINITION
#define STATUS_DROPPED_REDEFINITION(value) value
#define STATUS_ENUM ((ConstantState)1)
#define STATUS_ENUM_TAG ((_ConstantState)(1 + 1))
#define STATUS_TAG_ONLY ((TagOnlyStatus)(1 + 1))
#define STATUS_MERGED ((MergedStatus)(1 + 1))

#define STATUS_TYPE_SHADOW ((TypeShadowStatus)(1 + 2))
#define TypeShadowStatus unsigned int
#define FunctionShadowStatus(value) value
#define STATUS_FUNCTION_SHADOW ((FunctionShadowStatus)(1 + 2))
#define STATUS_NUMERIC_SHADOW NumericShadowStatus
#define NumericShadowStatus 6
extern int runtime_status;
#define STATUS_RUNTIME ((RuntimeStatus)runtime_status)

typedef unsigned int MacroNameCollision;
#define MacroNameCollision 1
#undef MacroNameCollision
#define MacroNameCollision(value) value
