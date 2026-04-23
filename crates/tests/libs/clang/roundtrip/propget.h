struct __declspec(uuid("cd33ad7d-cb91-471d-a494-6a178012a31f"))
IFoo {
    virtual /* [id][helpstring][propget] */ unsigned int __stdcall get_Count() = 0;
    virtual /* [helpstring] */ unsigned int __stdcall get_Other() = 0;
    virtual unsigned int __stdcall get_NoComment() = 0;
};

struct __declspec(uuid("cd33ad7d-cb91-471d-a494-6a178012a31e"))
IBar {
    virtual /* [id][helpstring][propput] */ void __stdcall put_Count(unsigned int value) = 0;
    virtual /* [helpstring] */ void __stdcall put_Other(unsigned int value) = 0;
    virtual void __stdcall put_NoComment(unsigned int value) = 0;
};
