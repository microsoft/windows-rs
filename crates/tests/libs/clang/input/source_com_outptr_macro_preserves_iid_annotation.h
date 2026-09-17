//! namespace ComOut
//! library api.dll

#define _COM_Outptr_ __attribute__((annotate("_Outptr_")))
             struct IThing;
             extern "C" int Create(const void* iid, _COM_Outptr_ void** object);
             extern "C" int CreateThing(_COM_Outptr_ IThing** object);
