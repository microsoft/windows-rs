struct Record { int value; };
typedef Record* Pointer;
typedef void (*Callback)(int);
extern "C" void FunctionW(int value);
enum Mode { Active = 3 };
const int Number = 7;
#define Function FunctionW
#define FunctionChain Function
#define TypeAlias Record
#define PointerAlias Pointer
#define CallbackAlias Callback
#define EnumAlias Mode
#define EnumValue Active
#define ConstantAlias Number
#define LiteralValue 11
#define Expression (LiteralValue + 1)
#define StringValue "FunctionW"
#define Invoker(value) FunctionW(value)
#define CycleA CycleB
#define CycleB CycleA
#define Changed FunctionW
#undef Changed
#define Changed 17
extern "C" void Shadowed(int value);
#define Shadowed 23
#define ValueChain Shadowed
#define DeclarationAttribute __declspec(deprecated)
#define AttributeAlias DeclarationAttribute
#define AttributeChain AttributeAlias
#define ImportAttribute __attribute__((dllimport))
#define BracketAttribute [[deprecated]]
#define AttributeText "__declspec(deprecated)"
#define TemporaryFunction(value)
#undef TemporaryFunction
#define TemporarySpliced\
(value)
#undef TemporarySpliced
#define ParenthesizedValue (23)
#define CommentSeparated/**/(23)
#define SpacedContinuation\
 (23)
#define ChangedAttribute __declspec(deprecated)
#undef ChangedAttribute
#define ChangedAttribute 23
#define TemporaryObject FunctionW
#undef TemporaryObject
