typedef long Status;
typedef Status Alias;
struct StatusRecord { Alias value; };
extern "C" Alias Convert(Status input, Status* output, StatusRecord value);
struct __declspec(uuid("00000001-0000-0000-c000-000000000046")) IStatus {
    virtual Alias __stdcall Convert(Status input, Status* output) = 0;
};
