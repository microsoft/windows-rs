typedef unsigned short IncludedStatus;
typedef IncludedStatus ChainedStatus;
typedef unsigned long DWORD;
typedef unsigned short UnusedStatus;
typedef unsigned short TypeShadowStatus;
typedef unsigned short FunctionShadowStatus;
typedef unsigned short NumericShadowStatus;
typedef unsigned short RuntimeStatus;

typedef enum _ConstantState : unsigned int {
    ConstantStateReady = 1,
} ConstantState;

enum TagOnlyStatus : unsigned short {
    TagOnlyReady = 1,
};

enum _MergedStatus {
    MergedStatusReady = 1,
};
typedef unsigned long MergedStatus;
