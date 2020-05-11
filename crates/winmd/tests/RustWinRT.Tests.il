// Since the MIDL toolchain is Windows-only, this WinMD file is written in IL.
// The WinMD generated from this IL is similar to what the following MIDL expresses:
//
// namespace RustWinRT.Tests
// {
//     interface ITestNoException
//     {
//         [noexcept] String NoException();
//         String MaybeException();
//     }
// }
//
// Since there are currently no APIs in the Windows SDK that use noexcept,
// this IL is needed to test that functionality.

.assembly extern mscorlib
{
    .publickeytoken = (b7 7a 5c 56 19 34 e0 89)
    .ver 4:0:0:0
}

.assembly extern windowsruntime Windows.Foundation
{
    .ver 4:0:0:0
}

.assembly extern windowsruntime winrtbase
{
    .ver 255:255:255:255
}

.module RustWinRT.Tests.winmd
.imagebase 0x00400000
.file alignment 0x00000200
.stackreserve 0x00100000
.subsystem 0x0003 // WindowsCui
.corflags 0x00000001 // ILOnly


.class private auto ansi '<Module>'
{
} // end of class <Module>

.class interface public auto ansi abstract import windowsruntime RustWinRT.Tests.ITestNoException
{
    .custom instance void [Windows.Foundation]Windows.Foundation.Metadata.VersionAttribute::.ctor(uint32) = (01 00 01 00 00 00 00 00)
    .custom instance void [Windows.Foundation]Windows.Foundation.Metadata.GuidAttribute::.ctor(uint32, uint16, uint16, uint8, uint8, uint8, uint8, uint8, uint8, uint8, uint8) 
        = (01 00 56 0f 15 2e 5d 31 ca 58 84 49 d2 7d ed bb 67 6b 00 00)

    .method public hidebysig newslot abstract virtual instance string NoException() runtime managed
    {
        .custom instance void [winrtbase]Windows.Foundation.Metadata.NoExceptionAttribute::.ctor() = (01 00 00 00)
    }
    .method public hidebysig newslot abstract virtual instance string MaybeException() runtime managed
    {
    }
}
