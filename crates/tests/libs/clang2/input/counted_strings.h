#define _In_reads_(n) __attribute__((annotate("_In_reads_(" #n ")")))
#define _Inout_updates_(n) __attribute__((annotate("_Inout_updates_(" #n ")")))
#define _Out_writes_bytes_(n) __attribute__((annotate("_Out_writes_bytes_(" #n ")")))
typedef wchar_t* Text;
typedef const wchar_t* ConstText;
extern "C" void Update(_Inout_updates_(count) Text value, unsigned count);
extern "C" void Read(_In_reads_(count) ConstText value, unsigned count);
extern "C" void Bytes(_Out_writes_bytes_(count) Text value, unsigned count);
