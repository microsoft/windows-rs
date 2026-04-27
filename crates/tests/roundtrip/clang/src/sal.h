// Test SAL annotation extraction.
//
// SAL macros are defined here using __attribute__((annotate("…"))) so that
// CXCursor_AnnotateAttr children appear on each ParmDecl cursor, which is the
// mechanism the SAL extractor checks first.
#define _In_      __attribute__((annotate("_In_")))
#define _Out_     __attribute__((annotate("_Out_")))
#define _Inout_   __attribute__((annotate("_Inout_")))
#define _In_opt_  __attribute__((annotate("_In_opt_")))
#define _Out_opt_ __attribute__((annotate("_Out_opt_")))

// _In_ on a mutable pointer: must emit #[r#in] because the default for
// mutable-pointer parameters is Out.
void ReadBuffer(_In_ int* data, int count);

// _Out_ on a mutable pointer: no explicit attribute needed in the round-trip
// output since the default is already Out for mutable pointers.
void GetValue(_Out_ int* result);

// _Inout_: both In and Out are explicit.
void Transform(_Inout_ int* value);

// _In_opt_ on a const pointer: optional, no direction override needed.
void LookupByName(_In_opt_ const char* name);

// _Out_opt_ on a mutable pointer: optional, no direction override needed.
void TryGetValue(_Out_opt_ int* result);

// No SAL — plain parameters with no annotation.
void Plain(int x);
