enum class IncludedForward : unsigned short;
enum class ForwardOnly : unsigned short;

#include "enum_dependency_inc.inl"

IncludedForward ReturnIncludedForward(void);
IncludedEnum ReturnIncludedEnum(void);
TypedefBacked ReturnTypedefBacked(void);
ForwardOnly ReturnForwardOnly(void);
