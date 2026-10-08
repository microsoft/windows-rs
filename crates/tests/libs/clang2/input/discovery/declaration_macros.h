#define Linkage extern "C"
#define LinkageAlias Linkage
#define Import __declspec(dllimport)
#define PublicApi LinkageAlias Import
#define PublicApiAlias PublicApi
#define BeginDeclarations Linkage {
#define EndDeclarations }
#define Convention __cdecl
#define ConventionAlias Convention
#define Noexcept noexcept
#define NoexceptAlias Noexcept
#define NoexceptValue noexcept(1 + 2)
#define KeywordText "extern noexcept __cdecl"
#define Value 17
#define ValueAlias Value
#define ValueExpression ValueAlias + 1
#define CycleA CycleB
#define CycleB CycleA
#define CyclePrefix CycleA Import

BeginDeclarations
void Native(int value) Noexcept;
EndDeclarations

PublicApi void Imported(int value);
#define CallExpression Native(17)
