struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
};

unsigned int __stdcall NoOleCoCreate(
    const struct _GUID &rclsid,
    const struct _GUID &riid,
    void **ppv);
