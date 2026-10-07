struct _GUID {
    unsigned long data1;
    unsigned short data2;
    unsigned short data3;
    unsigned char data4[8];
};

struct Notification {
    _GUID context;
    int muted;
    float volume;
    unsigned int channel_count;
    float channels[1];
};
