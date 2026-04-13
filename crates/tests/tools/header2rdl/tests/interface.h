struct IUnknown {
    virtual long QueryInterface(int riid, void** ppv) = 0;
    virtual unsigned int AddRef() = 0;
    virtual unsigned int Release() = 0;
};

struct IWidget : IUnknown {
    virtual long GetId(int* id) = 0;
    virtual long SetName(const char* name) = 0;
};

struct IWidgetFactory : IUnknown {
    virtual long CreateWidget(const char* name, IWidget** out) = 0;
};
