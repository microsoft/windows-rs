//! reference-default
//! args -x c++ --target=x86_64-pc-windows-msvc -fms-extensions

typedef struct IDataObject IDataObject;
typedef IDataObject* LPDATAOBJECT;

#define DOBJ_NULL ((LPDATAOBJECT)0)
#define DOBJ_CUSTOMOCX ((LPDATAOBJECT)-1)
#define TEXT_CALLBACK ((char*)-1)
#define DIRECT_POINTER ((IDataObject*)1)
