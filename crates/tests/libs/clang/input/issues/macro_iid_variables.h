typedef struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID;
typedef GUID IID;

#define k_7zip_GUID_Data1 0x23170F69
#define k_7zip_GUID_Data2 0x40C1
#define k_7zip_GUID_Data3_Common 0x278A

#define Z7_DEFINE_GUID(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8) \
    extern "C" const GUID name; \
    extern "C" const GUID name = { l, w1, w2, { b1, b2, b3, b4, b5, b6, b7, b8 } }

#define Z7_DECL_IFACE_7ZIP_SUB(i, groupId, subId) \
    Z7_DEFINE_GUID(IID_##i, k_7zip_GUID_Data1, k_7zip_GUID_Data2, k_7zip_GUID_Data3_Common, \
        0, 0, 0, (groupId), 0, (subId), 0, 0)

Z7_DECL_IFACE_7ZIP_SUB(ISequentialInStream, 3, 0x01);
